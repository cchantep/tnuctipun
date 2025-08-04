//! Tests for UpdateBuilder functionality, method chaining, and builder patterns

use super::test_fixtures::*;
use tnuctipun::updates::{UpdateBuilder, empty};

#[test]
fn test_empty_builder_produces_empty_document() {
    let doc = empty::<TestStruct>().build();

    assert!(doc.is_empty());
}

#[test]
fn test_method_chaining_works_with_mut_self_pattern() {
    // This test demonstrates that method chaining now works fully,
    // including calling build() at the end of the chain
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("chained_value")
        .set::<AnotherFieldName, _>(100)
        .inc::<AnotherFieldName, _>(50)
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "chained_value",
            "another_field": 100
        },
        "$inc": {
            "another_field": 50
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_empty_function_creates_new_builder() {
    let mut builder = empty::<TestStruct>();
    let doc = builder.build();

    assert!(doc.is_empty());
}

#[test]
fn test_empty_function_method_chaining() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("test")
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "test"
        }
    };

    assert_eq!(doc, expected);
}

#[test]
fn test_set_operations_with_prefix() {
    let mut builder = UpdateBuilder::<TestStruct>::new();

    // Add prefix to simulate nested document updates
    builder.prefix.push("parent".to_string());
    builder.prefix.push("child".to_string());

    builder.set::<TestFieldName, _>("nested_value");
    builder.set::<AnotherFieldName, _>(100);

    let doc = builder.build();

    // Check the entire document structure with prefixed field paths
    let expected_doc = bson::doc! {
        "$set": {
            "parent.child.test_field": "nested_value",
            "parent.child.another_field": 100
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_comprehensive_document_structure() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("comprehensive_test")
        .set::<AnotherFieldName, _>(999)
        .set::<NestedFieldName, _>(false)
        .build();

    // This approach is much cleaner than checking individual fields!
    // It verifies the entire document structure in one assertion
    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "comprehensive_test",
            "another_field": 999,
            "nested.field": false
        }
    };

    // Single assertion covers structure, field names, values, and types
    assert_eq!(doc, expected_doc);
}
