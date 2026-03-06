//! Tests for FieldFilterBuilder in isolation

use super::test_fixtures::*;
use tnuctipun::field_filters::FieldFilterBuilder;

#[test]
fn build_returns_empty_document_without_operations() {
    let doc = FieldFilterBuilder::<product_fields::Price, Product>::new().build();

    assert_eq!(doc, bson::doc! {});
}

#[test]
fn build_eq_operation() {
    let doc = FieldFilterBuilder::<product_fields::Name, Product>::new()
        .eq("Laptop".to_string())
        .build();

    assert_eq!(doc, bson::doc! { "name": { "$eq": "Laptop" } });
}

#[test]
fn build_gt_operation() {
    let doc = FieldFilterBuilder::<product_fields::Price, Product>::new()
        .gt(100.0)
        .build();

    assert_eq!(doc, bson::doc! { "price": { "$gt": 100.0 } });
}

#[test]
fn build_gte_operation() {
    let doc = FieldFilterBuilder::<product_fields::Stock, Product>::new()
        .gte(5)
        .build();

    assert_eq!(doc, bson::doc! { "stock": { "$gte": 5 } });
}

#[test]
fn build_lt_operation() {
    let doc = FieldFilterBuilder::<product_fields::Price, Product>::new()
        .lt(300.0)
        .build();

    assert_eq!(doc, bson::doc! { "price": { "$lt": 300.0 } });
}

#[test]
fn build_lte_operation() {
    let doc = FieldFilterBuilder::<product_fields::Stock, Product>::new()
        .lte(10)
        .build();

    assert_eq!(doc, bson::doc! { "stock": { "$lte": 10 } });
}

#[test]
fn build_in_operation() {
    let values = vec!["Electronics".to_string(), "Computers".to_string()];
    let doc = FieldFilterBuilder::<product_fields::Categories, Product>::new()
        .r#in(values.clone())
        .build();

    assert_eq!(doc, bson::doc! { "categories": { "$in": values } });
}

#[test]
fn build_nin_operation() {
    let values = vec!["Discontinued".to_string(), "Archived".to_string()];
    let doc = FieldFilterBuilder::<product_fields::Brand, Product>::new()
        .nin(values.clone())
        .build();

    assert_eq!(doc, bson::doc! { "brand": { "$nin": values } });
}

#[test]
fn build_exists_operation() {
    let doc = FieldFilterBuilder::<product_fields::Brand, Product>::new()
        .exists(true)
        .build();

    assert_eq!(doc, bson::doc! { "brand": { "$exists": true } });
}

#[test]
fn build_combined_operations_for_same_field() {
    let doc = FieldFilterBuilder::<product_fields::Price, Product>::new()
        .gt(50.0)
        .lte(200.0)
        .build();

    assert_eq!(doc, bson::doc! { "price": { "$gt": 50.0, "$lte": 200.0 } });
}
