//! Tests for basic projection operations: includes, excludes, project

use super::test_fixtures::*;
use tnuctipun::projection::{ProjectionBuilder, empty};

#[test]
fn projection_includes_generates_correct_paths() {
    // Test that the includes method generates correct field paths
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
fn projection_excludes_generates_correct_paths() {
    // Test that excludes method generates correct field paths
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
fn projection_custom_expression() {
    // Test that project method generates correct field paths
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
fn projection_mixed_includes_excludes_project() {
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
fn projection_empty_build() {
    // Test building an empty projection
    let doc = empty::<User>().build();
    let expected = bson::doc! {};

    assert_eq!(doc, expected);
}

#[test]
fn projection_duplicate_field_handling() {
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
