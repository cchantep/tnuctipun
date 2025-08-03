//! Tests for logical operations (and, or, not)

use super::test_fixtures::*;
use nessus::filters::empty;

#[test]
fn test_and_function_empty_builder() {
    // Test with empty builder - should return empty document
    let builder = empty::<Product>();
    let result = builder.and();
    let expected = bson::doc! {};

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_single_clause() {
    // Test with single clause - should return the clause directly (no $and wrapper)
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Smartphone".to_string())
        .and();

    let expected = bson::doc! { "name": "Smartphone" };

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_multiple_clauses() {
    // Test with multiple clauses - should wrap in $and
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Smartphone".to_string())
        .eq::<product_fields::Price, _>(599.99)
        .and();

    let expected = bson::doc! {
        "$and": [
            { "name": "Smartphone" },
            { "price": 599.99 }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_or_operations() {
    // Test OR operations using the builder
    let names = vec!["Laptop".to_string(), "Smartphone".to_string()];
    let result = empty::<Product>()
        .gt::<product_fields::Price, _>(100.0)
        .or::<product_fields::Name, _, _>(names, |filter, name| {
            filter.eq::<product_fields::Name, _>(name.to_string())
        })
        .and();

    let expected = bson::doc! {
        "$and": [
            { "price": { "$gt": 100.0 } },
            { "$or": [
                { "name": "Laptop" },
                { "name": "Smartphone" }
            ]}
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_not_filter_basic() {
    let filter = empty::<Product>()
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .and();

    let expected = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };

    assert_eq!(filter, expected);
}

#[test]
fn test_not_filter_with_and_combination() {
    let filter = empty::<Product>()
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .eq::<product_fields::Categories, _>("Electronics".to_string())
        .and();

    let expected = bson::doc! {
        "$and": [
            {
                "name": {
                    "$not": {
                        "name": { "$eq": "Smartphone" }
                    }
                }
            },
            { "categories": "Electronics" }
        ]
    };

    assert_eq!(filter, expected);
}

#[test]
fn test_or_standalone_operation() {
    // Test OR operation without other conditions
    let categories = vec![
        "Electronics".to_string(),
        "Books".to_string(),
        "Home".to_string(),
    ];

    let result = empty::<Product>()
        .or::<product_fields::Categories, _, _>(categories, |filter, category| {
            filter.eq::<product_fields::Categories, _>(category.to_string())
        })
        .and();

    let expected = bson::doc! {
        "$or": [
            { "categories": "Electronics" },
            { "categories": "Books" },
            { "categories": "Home" }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_or_operations() {
    // Test multiple OR operations combined
    let names = vec!["Laptop".to_string(), "Smartphone".to_string()];
    let brands = vec!["Apple".to_string(), "Samsung".to_string()];

    let result = empty::<Product>()
        .or::<product_fields::Name, _, _>(names, |filter, name| {
            filter.eq::<product_fields::Name, _>(name.to_string())
        })
        .or::<product_fields::Brand, _, _>(brands, |filter, brand| {
            filter.eq::<product_fields::Brand, _>(brand.to_string())
        })
        .and();

    let expected = bson::doc! {
        "$and": [
            { "$or": [
                { "name": "Laptop" },
                { "name": "Smartphone" }
            ]},
            { "$or": [
                { "brand": "Apple" },
                { "brand": "Samsung" }
            ]}
        ]
    };

    assert_eq!(result, expected);
}
