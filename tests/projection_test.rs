use bson;
use nessus::FieldWitnesses;
use nessus::projection::ProjectionBuilder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FieldWitnesses)]
struct User {
    id: String,
    name: String,
    email: String,
    age: u32,
}

#[derive(Deserialize, Serialize, FieldWitnesses)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[derive(Deserialize, Serialize, FieldWitnesses)]
struct Profile {
    bio: String,
    location: String,
    avatar_url: String,
}

mod projection_builder_integration_tests {
    use super::*;

    #[test]
    fn projection_builder_includes_generates_correct_paths() {
        // Test that the includes method generates correct field paths through field_path
        let doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .build();

        assert!(doc.contains_key("name"));
        assert!(doc.contains_key("age"));
        assert_eq!(doc.get("name").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(doc.get("age").unwrap(), &bson::Bson::Int32(1));
    }

    #[test]
    fn projection_builder_excludes_generates_correct_paths() {
        // Test that excludes method generates correct field paths through field_path
        let doc = ProjectionBuilder::<User>::new()
            .excludes::<user_fields::Email>()
            .excludes::<user_fields::Id>()
            .build();

        assert!(doc.contains_key("email"));
        assert!(doc.contains_key("id"));
        assert_eq!(doc.get("email").unwrap(), &bson::Bson::Int32(0));
        assert_eq!(doc.get("id").unwrap(), &bson::Bson::Int32(0));
    }

    #[test]
    fn projection_builder_project_custom_expression() {
        // Test that project method generates correct field paths through field_path
        let custom_expr = bson::doc! { "$slice": [0, 10] };

        let doc = ProjectionBuilder::<User>::new()
            .project::<user_fields::Name>(custom_expr.clone().into())
            .build();

        assert!(doc.contains_key("name"));
        assert_eq!(doc.get("name").unwrap(), &custom_expr.clone().into());
    }

    #[test]
    fn projection_builder_mixed_includes_excludes_project() {
        // Test using includes, excludes, and project together in a chain
        let slice_expr = bson::doc! { "$slice": 5 };

        let doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Email>()
            .project::<user_fields::Id>(slice_expr.clone().into())
            .build();

        assert!(doc.contains_key("name"));
        assert!(doc.contains_key("email"));
        assert!(doc.contains_key("id"));

        assert_eq!(doc.get("name").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(doc.get("email").unwrap(), &bson::Bson::Int32(0));
        assert_eq!(doc.get("id").unwrap(), &slice_expr.clone().into());
    }

    #[test]
    fn projection_builder_empty_build() {
        // Test building an empty projection
        let doc = ProjectionBuilder::<User>::new().build();

        assert_eq!(doc.len(), 0);
        assert!(doc.is_empty());
    }

    #[test]
    fn projection_builder_duplicate_field_handling() {
        // Test that later operations on the same field override earlier ones
        let doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Name>()
            .build();

        assert!(doc.contains_key("name"));
        assert_eq!(doc.get("name").unwrap(), &bson::Bson::Int32(0)); // excludes wins
    }

    #[test]
    fn projection_builder_complex_expressions() {
        // Test various MongoDB projection expressions
        let elem_match = bson::doc! {
            "$elemMatch": { "score": { "$gt": 80 } }
        };
        let conditional = bson::doc! {
            "$cond": {
                "if": { "$gte": ["$age", 18] },
                "then": "$name",
                "else": "Minor"
            }
        };

        let doc = ProjectionBuilder::<User>::new()
            .project::<user_fields::Name>(elem_match.clone().into())
            .project::<user_fields::Age>(conditional.clone().into())
            .build();

        assert!(doc.contains_key("name"));
        assert!(doc.contains_key("age"));
        assert_eq!(doc.get("name").unwrap(), &elem_match.clone().into());
        assert_eq!(doc.get("age").unwrap(), &conditional.clone().into());
    }

    #[test]
    fn projection_builder_separate_structs() {
        // Test that different struct types create separate projections
        let user_doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .build();

        let address_doc = ProjectionBuilder::<Address>::new()
            .includes::<address_fields::Street>()
            .build();

        assert!(user_doc.contains_key("name"));
        assert!(!user_doc.contains_key("street"));

        assert!(address_doc.contains_key("street"));
        assert!(!address_doc.contains_key("name"));
    }

    #[test]
    fn projection_builder_all_supported_operations_chained() {
        // Test chaining all operations in one expression
        let slice_expr = bson::doc! { "$slice": 5 };
        let conditional = bson::doc! {
            "$cond": {
                "if": { "$ne": ["$email", null] },
                "then": "$email",
                "else": "no-email"
            }
        };

        let doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Email>()
            .project::<user_fields::Id>(slice_expr.clone().into())
            .project::<user_fields::Age>(conditional.clone().into())
            .build();

        assert_eq!(doc.len(), 4);
        assert_eq!(doc.get("name").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(doc.get("email").unwrap(), &bson::Bson::Int32(0));
        assert_eq!(doc.get("id").unwrap(), &slice_expr.clone().into());
        assert_eq!(doc.get("age").unwrap(), &conditional.clone().into());
    }

    #[test]
    fn projection_builder_field_type_verification() {
        // Test that the builder correctly handles different field types
        let doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .includes::<user_fields::Email>()
            .includes::<user_fields::Id>()
            .build();

        // All fields should be present with include value
        assert_eq!(doc.len(), 4);
        assert_eq!(doc.get("name").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(doc.get("age").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(doc.get("email").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(doc.get("id").unwrap(), &bson::Bson::Int32(1));
    }

    #[test]
    fn projection_builder_multiple_struct_types() {
        // Test projections with multiple different struct types
        let user_doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .build();

        let address_doc = ProjectionBuilder::<Address>::new()
            .includes::<address_fields::City>()
            .includes::<address_fields::Zip>()
            .build();

        let profile_doc = ProjectionBuilder::<Profile>::new()
            .includes::<profile_fields::Bio>()
            .includes::<profile_fields::AvatarUrl>()
            .build();

        // Verify each projection contains only its own fields
        assert_eq!(user_doc.len(), 2);
        assert!(user_doc.contains_key("name"));
        assert!(user_doc.contains_key("age"));

        assert_eq!(address_doc.len(), 2);
        assert!(address_doc.contains_key("city"));
        assert!(address_doc.contains_key("zip"));

        assert_eq!(profile_doc.len(), 2);
        assert!(profile_doc.contains_key("bio"));
        assert!(profile_doc.contains_key("avatar_url"));
    }

    #[test]
    fn projection_builder_advanced_mongodb_expressions() {
        // Test advanced MongoDB projection expressions
        let conditional = bson::doc! {
            "$cond": {
                "if": { "$gte": ["$age", 21] },
                "then": "$name",
                "else": "underage"
            }
        };

        let array_ops = bson::doc! {
            "$map": {
                "input": "$tags",
                "as": "tag",
                "in": { "$toUpper": "$$tag" }
            }
        };

        let date_ops = bson::doc! {
            "$dateToString": {
                "format": "%Y-%m-%d",
                "date": "$created_at"
            }
        };

        let string_ops = bson::doc! {
            "$concat": ["$first_name", " ", "$last_name"]
        };

        let doc = ProjectionBuilder::<User>::new()
            .project::<user_fields::Name>(conditional.clone().into())
            .project::<user_fields::Id>(array_ops.clone().into())
            .project::<user_fields::Email>(date_ops.clone().into())
            .project::<user_fields::Age>(string_ops.clone().into())
            .build();

        assert_eq!(doc.len(), 4);
        assert_eq!(doc.get("name").unwrap(), &conditional.clone().into());
        assert_eq!(doc.get("id").unwrap(), &array_ops.clone().into());
        assert_eq!(doc.get("email").unwrap(), &date_ops.clone().into());
        assert_eq!(doc.get("age").unwrap(), &string_ops.clone().into());
    }

    #[test]
    fn projection_builder_performance_many_fields() {
        // Test performance with many fields - single field
        let single_doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .build();
        assert_eq!(single_doc.len(), 1);

        // Test performance with many fields - multiple fields
        let many_doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .includes::<user_fields::Email>()
            .includes::<user_fields::Id>()
            .build();
        assert_eq!(many_doc.len(), 4);

        // Test with mixed operations
        let mixed_doc = ProjectionBuilder::<User>::new()
            .project::<user_fields::Name>(bson::Bson::Null)
            .project::<user_fields::Age>(bson::Bson::Int32(42))
            .build();

        assert_eq!(mixed_doc.len(), 2);
        assert_eq!(mixed_doc.get("name").unwrap(), &bson::Bson::Null);
        assert_eq!(mixed_doc.get("age").unwrap(), &bson::Bson::Int32(42));
    }

    #[test]
    fn projection_builder_field_override_patterns() {
        // Test various field override patterns
        let doc = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Name>()
            .project::<user_fields::Name>(bson::doc! { "$toUpper": "$name" }.into())
            .build();

        // The project call should win (last operation)
        assert_eq!(doc.len(), 1);
        assert!(doc.contains_key("name"));
        let expected = bson::doc! { "$toUpper": "$name" };
        assert_eq!(doc.get("name").unwrap(), &expected.into());
    }

    #[test]
    fn projection_builder_real_world_usage() {
        // Test a real-world usage pattern
        let user_projection = ProjectionBuilder::<User>::new()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .excludes::<user_fields::Email>() // Don't return sensitive email
            .project::<user_fields::Id>(bson::doc! { "$toString": "$_id" }.into()) // Convert ObjectId to string
            .build();

        assert!(user_projection.contains_key("name"));
        assert!(user_projection.contains_key("age"));
        assert!(user_projection.contains_key("email"));
        assert!(user_projection.contains_key("id"));

        assert_eq!(user_projection.get("name").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(user_projection.get("age").unwrap(), &bson::Bson::Int32(1));
        assert_eq!(user_projection.get("email").unwrap(), &bson::Bson::Int32(0));

        let expected_id_expr = bson::doc! { "$toString": "$_id" };
        assert_eq!(user_projection.get("id").unwrap(), &expected_id_expr.into());
    }
}
