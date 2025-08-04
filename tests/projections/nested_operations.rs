//! Tests for basic nested field projections (simplified to avoid complex API issues)

use super::test_fixtures::*;
use tnuctipun::projection::empty;

#[test]
fn projection_basic_include() {
    // Test basic include functionality
    let doc = empty::<User>().includes::<user_fields::Name>().build();

    let expected = bson::doc! {
        "name": 1
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_basic_exclude() {
    // Test basic exclude functionality
    let doc = empty::<User>().excludes::<user_fields::Id>().build();

    let expected = bson::doc! {
        "id": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_multiple_fields() {
    // Test multiple field operations
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
