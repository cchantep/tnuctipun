//! Tests for ProjectionBuilder functionality, method chaining, and builder patterns

use super::test_fixtures::*;
use tnuctipun::projection::empty;

#[test]
fn projection_builder_method_chaining() {
    // Test that method chaining works correctly
    let doc = empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .excludes::<user_fields::Id>()
        .build();

    let expected = bson::doc! {
        "name": 1,
        "email": 1,
        "id": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_builder_field_type_verification() {
    // Test that field types are properly handled
    let doc = empty::<User>()
        .includes::<user_fields::Name>() // String field
        .includes::<user_fields::Age>() // u32 field
        .build();

    let expected = bson::doc! {
        "name": 1,
        "age": 1
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_builder_multiple_struct_types() {
    // Test projections with different struct types
    let user_projection = empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .build();

    let profile_projection = empty::<Profile>()
        .includes::<profile_fields::Bio>()
        .excludes::<profile_fields::AvatarUrl>()
        .build();

    let expected_user = bson::doc! {
        "name": 1,
        "email": 1
    };

    let expected_profile = bson::doc! {
        "bio": 1,
        "avatar_url": 0
    };

    assert_eq!(user_projection, expected_user);
    assert_eq!(profile_projection, expected_profile);
}

#[test]
fn projection_builder_field_override_patterns() {
    // Test field override scenarios
    let doc = empty::<User>()
        .includes::<user_fields::Name>()
        .excludes::<user_fields::Name>()
        .includes::<user_fields::Name>() // Final include should win
        .build();

    let expected = bson::doc! {
        "name": 1
    };

    assert_eq!(doc, expected);
}
