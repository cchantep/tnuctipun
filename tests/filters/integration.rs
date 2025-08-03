//! Integration tests combining multiple filter operations

use super::test_fixtures::*;
use nessus::filters::empty;

#[test]
fn test_comprehensive_filter_chain() {
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Gaming Laptop".to_string())
        .gt::<product_fields::Price, _>(500.0)
        .lt::<product_fields::Price, _>(2000.0)
        .gte::<product_fields::Stock, _>(1)
        .r#in::<product_fields::Categories, _>(vec![
            "Electronics".to_string(),
            "Computers".to_string(),
        ])
        .and();

    let expected = bson::doc! {
        "$and": [
            { "name": "Gaming Laptop" },
            { "price": { "$gt": 500.0 } },
            { "price": { "$lt": 2000.0 } },
            { "stock": { "$gte": 1 } },
            { "categories": { "$in": ["Electronics", "Computers"] } }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_product_availability_filter() {
    // Filter for available products within price range
    let result = empty::<Product>()
        .gt::<product_fields::Stock, _>(0)
        .gte::<product_fields::Price, _>(10.0)
        .lte::<product_fields::Price, _>(500.0)
        .r#in::<product_fields::Categories, _>(vec![
            "Electronics".to_string(),
            "Books".to_string(),
            "Home".to_string(),
        ])
        .and();

    let expected = bson::doc! {
        "$and": [
            { "stock": { "$gt": 0 } },
            { "price": { "$gte": 10.0 } },
            { "price": { "$lte": 500.0 } },
            { "categories": { "$in": ["Electronics", "Books", "Home"] } }
        ]
    };

    assert_eq!(result, expected);
}
