//! Tests for basic filter operations

use super::test_fixtures::*;
use nessus::filters::empty;

#[test]
fn test_eq_filter() {
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Laptop".to_string())
        .and();

    let expected = bson::doc! {
        "name": "Laptop"
    };

    assert_eq!(result, expected);
}

#[test]
fn test_gt_filter() {
    let result = empty::<Product>()
        .gt::<product_fields::Price, _>(50.0)
        .and();

    let expected = bson::doc! {
        "price": { "$gt": 50.0 }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_lt_filter() {
    let result = empty::<Product>()
        .lt::<product_fields::Price, _>(200.0)
        .and();

    let expected = bson::doc! {
        "price": { "$lt": 200.0 }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_in_filter() {
    let categories = vec!["Electronics".to_string(), "Computers".to_string()];
    let result = empty::<Product>()
        .r#in::<product_fields::Categories, _>(categories.clone())
        .and();

    let expected = bson::doc! {
        "categories": { "$in": categories.clone() }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_ne_filter() {
    let result = empty::<Product>()
        .ne::<product_fields::Name, _>("Unavailable".to_string())
        .and();

    let expected = bson::doc! {
        "name": { "$ne": "Unavailable" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_gte_filter() {
    let result = empty::<Product>().gte::<product_fields::Stock, _>(10).and();

    let expected = bson::doc! {
        "stock": { "$gte": 10 }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_lte_filter() {
    let result = empty::<Product>()
        .lte::<product_fields::Price, _>(500.0)
        .and();

    let expected = bson::doc! {
        "price": { "$lte": 500.0 }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_exists_filter() {
    let result = empty::<Product>()
        .exists::<product_fields::Categories>(true)
        .and();

    let expected = bson::doc! {
        "categories": { "$exists": true }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_nin_filter() {
    let excluded_categories = vec!["Discontinued".to_string(), "Out of Stock".to_string()];
    let result = empty::<Product>()
        .nin::<product_fields::Categories, _>(excluded_categories.clone())
        .and();

    let expected = bson::doc! {
        "categories": { "$nin": excluded_categories }
    };

    assert_eq!(result, expected);
}
