use nessus::FieldWitnesses;
use nessus::projection::{ProjectionBuilder, empty};
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
        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .build();

        let expected = bson::doc! {
            "name": 1,
            "age": 1
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_excludes_generates_correct_paths() {
        // Test that excludes method generates correct field paths through field_path
        let doc = empty::<User>()
            .excludes::<user_fields::Email>()
            .excludes::<user_fields::Id>()
            .build();

        let expected = bson::doc! {
            "email": 0,
            "id": 0
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_project_custom_expression() {
        // Test that project method generates correct field paths through field_path
        let custom_expr = bson::doc! { "$slice": [0, 10] };

        let doc = empty::<User>()
            .project("name".to_string(), custom_expr.clone().into())
            .build();

        let expected = bson::doc! {
            "name": custom_expr.clone()
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_mixed_includes_excludes_project() {
        // Test using includes, excludes, and project together
        let slice_expr = bson::doc! { "$slice": 5 };

        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Email>()
            .project("id".to_string(), slice_expr.clone().into())
            .build();

        let expected = bson::doc! {
            "name": 1,
            "email": 0,
            "id": slice_expr.clone()
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_empty_build() {
        // Test building an empty projection
        let doc = empty::<User>().build();

        let expected = bson::doc! {};

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_duplicate_field_handling() {
        // Test that later operations on the same field override earlier ones
        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Name>()
            .build();

        let expected = bson::doc! {
            "name": 0  // excludes wins
        };

        assert_eq!(doc, expected);
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

        let doc = empty::<User>()
            .project("name".to_string(), elem_match.clone().into())
            .project("age".to_string(), conditional.clone().into())
            .build();

        let expected = bson::doc! {
            "name": elem_match.clone(),
            "age": conditional.clone()
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_separate_structs() {
        // Test that different struct types create separate projections
        let user_doc = empty::<User>()
            .includes::<user_fields::Name>()
            .build();

        let address_doc = empty::<Address>()
            .includes::<address_fields::Street>()
            .build();

        let expected_user = bson::doc! {
            "name": 1
        };

        let expected_address = bson::doc! {
            "street": 1
        };

        assert_eq!(user_doc, expected_user);
        assert_eq!(address_doc, expected_address);
    }

    #[test]
    fn projection_builder_all_supported_operations_chained() {
        // Test all operations together
        let slice_expr = bson::doc! { "$slice": 5 };
        let conditional = bson::doc! {
            "$cond": {
                "if": { "$ne": ["$email", null] },
                "then": "$email",
                "else": "no-email"
            }
        };

        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Email>()
            .project("id".to_string(), slice_expr.clone().into())
            .project("age".to_string(), conditional.clone().into())
            .build();

        let expected = bson::doc! {
            "name": 1,
            "email": 0,
            "id": slice_expr.clone(),
            "age": conditional.clone()
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_field_type_verification() {
        // Test that the builder correctly handles different field types
        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .includes::<user_fields::Email>()
            .includes::<user_fields::Id>()
            .build();

        let expected = bson::doc! {
            "name": 1,
            "age": 1,
            "email": 1,
            "id": 1
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_multiple_struct_types() {
        // Test projections with multiple different struct types
        let user_doc = empty::<User>()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .build();

        let address_doc = empty::<Address>()
            .includes::<address_fields::City>()
            .includes::<address_fields::Zip>()
            .build();

        let profile_doc = empty::<Profile>()
            .includes::<profile_fields::Bio>()
            .includes::<profile_fields::AvatarUrl>()
            .build();

        let expected_user = bson::doc! {
            "name": 1,
            "age": 1
        };

        let expected_address = bson::doc! {
            "city": 1,
            "zip": 1
        };

        let expected_profile = bson::doc! {
            "bio": 1,
            "avatar_url": 1
        };

        assert_eq!(user_doc, expected_user);
        assert_eq!(address_doc, expected_address);
        assert_eq!(profile_doc, expected_profile);
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

        let doc = empty::<User>()
            .project("name".to_string(), conditional.clone().into())
            .project("id".to_string(), array_ops.clone().into())
            .project("email".to_string(), date_ops.clone().into())
            .project("age".to_string(), string_ops.clone().into())
            .build();

        let expected = bson::doc! {
            "name": conditional.clone(),
            "id": array_ops.clone(),
            "email": date_ops.clone(),
            "age": string_ops.clone()
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_performance_many_fields() {
        // Test performance with many fields - single field
        let single_doc = empty::<User>()
            .includes::<user_fields::Name>()
            .build();

        let expected_single = bson::doc! {
            "name": 1
        };

        assert_eq!(single_doc, expected_single);

        // Test performance with many fields - multiple fields
        let many_doc = empty::<User>()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .includes::<user_fields::Email>()
            .includes::<user_fields::Id>()
            .build();

        let expected_many = bson::doc! {
            "name": 1,
            "age": 1,
            "email": 1,
            "id": 1
        };

        assert_eq!(many_doc, expected_many);

        // Test with mixed operations
        let mixed_doc = empty::<User>()
            .project("name".to_string(), bson::Bson::Null)
            .project("age".to_string(), bson::Bson::Int32(42))
            .build();

        let expected_mixed = bson::doc! {
            "name": bson::Bson::Null,
            "age": 42
        };

        assert_eq!(mixed_doc, expected_mixed);
    }

    #[test]
    fn projection_builder_field_override_patterns() {
        // Test various field override patterns
        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .excludes::<user_fields::Name>()
            .project(
                "name".to_string(),
                bson::doc! { "$toUpper": "$name" }.into(),
            )
            .build();

        let expected = bson::doc! {
            "name": { "$toUpper": "$name" }  // project call wins (last operation)
        };

        assert_eq!(doc, expected);
    }

    #[test]
    fn projection_builder_real_world_usage() {
        // Test a real-world usage pattern
        let user_projection = empty::<User>()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .excludes::<user_fields::Email>() // Don't return sensitive email
            .project("id".to_string(), bson::doc! { "$toString": "$_id" }.into()) // Convert ObjectId to string
            .build();

        let expected = bson::doc! {
            "name": 1,
            "age": 1,
            "email": 0,
            "id": { "$toString": "$_id" }
        };

        assert_eq!(user_projection, expected);
    }

    #[test]
    fn projection_builder_method_chaining_works() {
        // Test that full method chaining works, including build() at the end
        let doc = empty::<User>()
            .includes::<user_fields::Name>()
            .includes::<user_fields::Age>()
            .excludes::<user_fields::Email>()
            .build();

        let expected = bson::doc! {
            "name": 1,
            "age": 1,
            "email": 0
        };

        assert_eq!(doc, expected);
    }
}
