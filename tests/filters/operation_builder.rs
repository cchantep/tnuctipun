//! Tests for operation builder functionality

use super::test_fixtures::*;
use tnuctipun::filters::empty;

#[test]
fn test_operation_builder_gt() {
    let mut builder = empty::<Product>();

    builder.gt::<product_fields::Price, _>(100.0);

    let clauses = builder.clauses();
    let expected = vec![bson::doc! {
        "price": {
            "$gt": 100.0
        }
    }];

    assert_eq!(clauses, &expected);
}

#[test]
fn test_operation_builder_chaining() {
    let mut builder = empty::<Product>();

    builder
        .eq::<product_fields::Name, _>("Test Product".to_string())
        .gt::<product_fields::Price, _>(0.0)
        .gte::<product_fields::Stock, _>(1);

    let clauses = builder.clauses();
    let expected = vec![
        bson::doc! { "name": "Test Product" },
        bson::doc! { "price": { "$gt": 0.0 } },
        bson::doc! { "stock": { "$gte": 1 } },
    ];

    assert_eq!(clauses, &expected);
}
