//! Tests for advanced projection features and edge cases

use super::test_fixtures::*;
use nessus::projection::empty;

#[test]
fn projection_mixed_includes_excludes() {
    // Test mixing includes and excludes in the same projection
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
fn projection_field_precedence_rules() {
    // Test field precedence when both include and exclude are specified
    let doc = empty::<User>()
        .includes::<user_fields::Name>()
        .excludes::<user_fields::Name>()
        .includes::<user_fields::Name>() // Last operation should win
        .build();

    let expected = bson::doc! {
        "name": 1
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_empty_projection() {
    // Test empty projection creation
    let doc = empty::<User>().build();
    let expected = bson::doc! {};

    assert_eq!(doc, expected);
}

#[test]
fn projection_single_field_operations() {
    // Test projections with single field operations
    let include_doc = empty::<User>().includes::<user_fields::Name>().build();
    let exclude_doc = empty::<User>().excludes::<user_fields::Id>().build();

    let expected_include = bson::doc! { "name": 1 };
    let expected_exclude = bson::doc! { "id": 0 };

    assert_eq!(include_doc, expected_include);
    assert_eq!(exclude_doc, expected_exclude);
}
