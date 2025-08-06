//! Tests for conditional update operations using `if_some`

use super::test_fixtures::*;
use tnuctipun::updates::empty;

#[test]
fn test_if_some_with_some_value_applies_operation() {
    let maybe_value = Some("conditional_value");

    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("base_value")
        .if_some(maybe_value, |builder, value| {
            builder.set::<AnotherFieldName, _>(format!("updated_with_{value}"))
        })
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "base_value",
            "another_field": "updated_with_conditional_value"
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_if_some_with_none_value_skips_operation() {
    let maybe_value: Option<&str> = None;

    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("base_value")
        .if_some(maybe_value, |builder, value| {
            builder.set::<AnotherFieldName, _>(format!("updated_with_{value}"))
        })
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "base_value"
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_if_some_chaining_multiple_conditions() {
    let maybe_string = Some("test_string".to_string());
    let maybe_number = Some(42);
    let maybe_missing: Option<String> = None;

    let doc = empty::<TestStruct>()
        .if_some(maybe_string, |builder, value| {
            builder.set::<TestFieldName, _>(value)
        })
        .if_some(maybe_number, |builder, value| {
            builder.inc::<NumericFieldName, _>(value)
        })
        .if_some(maybe_missing, |builder, value| {
            builder.set::<AnotherFieldName, _>(value)
        })
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "test_string"
        },
        "$inc": {
            "numeric_field": 42
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_if_some_with_complex_operations() {
    let maybe_tags = Some(vec!["tag1".to_string(), "tag2".to_string()]);

    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("base_item")
        .if_some(maybe_tags, |builder, tags| {
            // Use push_each to add all tags in a single operation
            builder.push_each::<ArrayFieldName, _, _, _>(tags)
        })
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "base_item"
        },
        "$push": {
            "array_field": {
                "$each": ["tag1", "tag2"]
            }
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_if_some_with_different_operation_types() {
    let maybe_increment = Some(5);

    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("initial")
        .if_some(maybe_increment, |builder, increment| {
            builder
                .inc::<NumericFieldName, _>(increment)
                .set::<AnotherFieldName, _>(format!("incremented_by_{increment}"))
        })
        .unset::<NestedFieldName>()
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "initial",
            "another_field": "incremented_by_5"
        },
        "$inc": {
            "numeric_field": 5
        },
        "$unset": {
            "nested.field": bson::Bson::Null
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_if_some_returns_builder_for_chaining() {
    let maybe_value = Some(100);

    // This test specifically verifies that if_some returns &mut Self
    // allowing for continued method chaining
    let doc = empty::<TestStruct>()
        .if_some(maybe_value, |builder, value| {
            builder.set::<NumericFieldName, _>(value)
        })
        .set::<TestFieldName, _>("after_if_some") // This should work
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "numeric_field": 100,
            "test_field": "after_if_some"
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_if_some_with_option_from_function() {
    fn maybe_get_email(has_email: bool) -> Option<String> {
        if has_email {
            Some("user@example.com".to_string())
        } else {
            None
        }
    }

    // Test with email present
    let doc_with_email = empty::<TestStruct>()
        .set::<TestFieldName, _>("John")
        .if_some(maybe_get_email(true), |builder, email| {
            builder.set::<AnotherFieldName, _>(email)
        })
        .build();

    let expected_with_email = bson::doc! {
        "$set": {
            "test_field": "John",
            "another_field": "user@example.com"
        }
    };

    assert_eq!(doc_with_email, expected_with_email);

    // Test with email absent
    let doc_without_email = empty::<TestStruct>()
        .set::<TestFieldName, _>("Jane")
        .if_some(maybe_get_email(false), |builder, email| {
            builder.set::<AnotherFieldName, _>(email)
        })
        .build();

    let expected_without_email = bson::doc! {
        "$set": {
            "test_field": "Jane"
        }
    };

    assert_eq!(doc_without_email, expected_without_email);
}

#[test]
fn test_if_some_with_nested_options() {
    let outer_option = Some(Some("nested_value".to_string()));
    let empty_inner_option = Some(None::<String>);
    let none_outer_option: Option<Option<String>> = None;

    // Test with nested Some(Some(value))
    let doc1 = empty::<TestStruct>()
        .if_some(outer_option, |builder, inner_option| {
            builder.if_some(inner_option, |inner_builder, value| {
                inner_builder.set::<TestFieldName, _>(value)
            })
        })
        .build();

    let expected1 = bson::doc! {
        "$set": {
            "test_field": "nested_value"
        }
    };

    assert_eq!(doc1, expected1);

    // Test with Some(None)
    let doc2 = empty::<TestStruct>()
        .if_some(empty_inner_option, |builder, inner_option| {
            builder.if_some(inner_option, |inner_builder, value| {
                inner_builder.set::<TestFieldName, _>(value)
            })
        })
        .set::<AnotherFieldName, _>("fallback")
        .build();

    let expected2 = bson::doc! {
        "$set": {
            "another_field": "fallback"
        }
    };

    assert_eq!(doc2, expected2);

    // Test with None
    let doc3 = empty::<TestStruct>()
        .if_some(none_outer_option, |builder, inner_option| {
            builder.if_some(inner_option, |inner_builder, value| {
                inner_builder.set::<TestFieldName, _>(value)
            })
        })
        .set::<NumericFieldName, _>(123)
        .build();

    let expected3 = bson::doc! {
        "$set": {
            "numeric_field": 123
        }
    };

    assert_eq!(doc3, expected3);
}
