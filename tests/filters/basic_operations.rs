//! Tests for basic filter operations

use super::test_fixtures::*;
use tnuctipun::filters::empty;

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

#[test]
fn test_untyped_filter_regex() {
    let regex_condition = bson::doc! {
        "$regex": "laptop",
        "$options": "i"
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Name>(regex_condition.clone())
        .and();

    let expected = bson::doc! {
        "name": regex_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_exists() {
    let exists_condition = bson::doc! {
        "$exists": true
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Categories>(exists_condition.clone())
        .and();

    let expected = bson::doc! {
        "categories": exists_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_array_elem_match() {
    let elem_match_condition = bson::doc! {
        "$elemMatch": {
            "$regex": "electronics",
            "$options": "i"
        }
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Categories>(elem_match_condition.clone())
        .and();

    let expected = bson::doc! {
        "categories": elem_match_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_mod_operator() {
    let mod_condition = bson::doc! {
        "$mod": [10, 0]
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Stock>(mod_condition.clone())
        .and();

    let expected = bson::doc! {
        "stock": mod_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_type_operator() {
    let type_condition = bson::doc! {
        "$type": "string"
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Name>(type_condition.clone())
        .and();

    let expected = bson::doc! {
        "name": type_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_complex_expression() {
    let complex_condition = bson::doc! {
        "$expr": {
            "$gt": [
                { "$multiply": ["$price", "$stock"] },
                1000.0
            ]
        }
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Price>(complex_condition.clone())
        .and();

    let expected = bson::doc! {
        "price": complex_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_combined_with_typed() {
    let regex_condition = bson::doc! {
        "$regex": "gaming",
        "$options": "i"
    };

    let result = empty::<Product>()
        .gt::<product_fields::Price, _>(100.0)
        .untyped::<product_fields::Name>(regex_condition.clone())
        .lte::<product_fields::Stock, _>(50)
        .and();

    let expected = bson::doc! {
        "$and": [
            { "price": { "$gt": 100.0 } },
            { "name": regex_condition },
            { "stock": { "$lte": 50 } }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_empty_document() {
    let empty_condition = bson::doc! {};

    let result = empty::<Product>()
        .untyped::<product_fields::Name>(empty_condition.clone())
        .and();

    let expected = bson::doc! {
        "name": empty_condition
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_filter_nested_conditions() {
    let nested_condition = bson::doc! {
        "$and": [
            { "$ne": "discontinued" },
            { "$regex": "^[A-Z]", "$options": "" }
        ]
    };

    let result = empty::<Product>()
        .untyped::<product_fields::Name>(nested_condition.clone())
        .and();

    let expected = bson::doc! {
        "name": nested_condition
    };

    assert_eq!(result, expected);
}
