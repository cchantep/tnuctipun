//! Integration tests combining multiple projection features

use super::test_fixtures::*;
use nessus::projection::empty;

#[test]
fn projection_integration_comprehensive_user_projection() {
    // Comprehensive test combining multiple projection techniques
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
fn projection_integration_mixed_data_types() {
    // Test projections with various data types
    let doc = empty::<User>()
        .includes::<user_fields::Name>() // String
        .includes::<user_fields::Age>() // u32
        .includes::<user_fields::Email>() // String
        .excludes::<user_fields::Id>() // String
        .build();

    let expected = bson::doc! {
        "name": 1,
        "age": 1,
        "email": 1,
        "id": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_integration_performance_optimized_query() {
    // Test projection designed for performance optimization
    let doc = empty::<User>()
        .includes::<user_fields::Name>() // Essential field
        .includes::<user_fields::Email>() // Essential field
        .excludes::<user_fields::Id>() // Exclude if not needed
        .build();

    let expected = bson::doc! {
        "name": 1,
        "email": 1,
        "id": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_integration_api_response_shaping() {
    // Test projection for shaping API responses
    let public_user_view = empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .excludes::<user_fields::Id>()
        .build();

    let private_admin_view = empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .includes::<user_fields::Id>()
        .includes::<user_fields::Age>()
        .build();

    let expected_public = bson::doc! {
        "name": 1,
        "email": 1,
        "id": 0
    };

    let expected_admin = bson::doc! {
        "name": 1,
        "email": 1,
        "id": 1,
        "age": 1
    };

    assert_eq!(public_user_view, expected_public);
    assert_eq!(private_admin_view, expected_admin);
}
