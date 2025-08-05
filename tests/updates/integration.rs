//! Integration tests combining multiple operations and complex scenarios

use super::test_fixtures::*;
use tnuctipun::updates::{PopStrategy, empty};

#[test]
fn test_mixed_operations_comprehensive() {
    let mut builder = empty::<TestStruct>();

    builder.set::<TestFieldName, _>("updated_value");
    builder.inc::<NumericFieldName, _>(5);
    builder.unset::<NestedFieldName>();
    builder.push::<ArrayFieldName, _>("new_item".to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "updated_value"
        },
        "$inc": {
            "numeric_field": 5
        },
        "$unset": {
            "nested.field": bson::Bson::Null
        },
        "$push": {
            "array_field": "new_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_comprehensive_array_operations() {
    let mut builder = empty::<TestStruct>();

    builder.add_to_set::<ArrayFieldName, _>("unique_item".to_string());
    builder.pop::<ArrayFieldName>(PopStrategy::Last);

    let result = builder.build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": "unique_item"
        },
        "$pop": {
            "array_field": 1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_comprehensive_comparison_operations() {
    let mut builder = empty::<TestStruct>();

    builder.set::<TestFieldName, _>("updated_value");
    builder.max::<NumericFieldName, _>(100);
    builder.min::<AnotherFieldName, _>(5);
    builder.inc::<NumericFieldName, _>(10);

    let result = builder.build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "updated_value"
        },
        "$max": {
            "numeric_field": 100
        },
        "$min": {
            "another_field": 5
        },
        "$inc": {
            "numeric_field": 10
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_complex_mixed_operations_with_comparisons() {
    let mut builder = empty::<TestStruct>();

    // Comprehensive mix of all operation types including max/min
    builder.set::<TestFieldName, _>("final_value");
    builder.max::<NumericFieldName, _>(50);
    builder.min::<AnotherFieldName, _>(1);
    builder.inc::<NumericFieldName, _>(5);
    builder.mul::<AnotherFieldName, _>(2);
    builder.unset::<NestedFieldName>();
    builder.push::<ArrayFieldName, _>("new_array_item".to_string());
    builder.add_to_set::<ArrayFieldName, _>("unique_item".to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "final_value"
        },
        "$max": {
            "numeric_field": 50
        },
        "$min": {
            "another_field": 1
        },
        "$inc": {
            "numeric_field": 5
        },
        "$mul": {
            "another_field": 2
        },
        "$unset": {
            "nested.field": bson::Bson::Null
        },
        "$push": {
            "array_field": "new_array_item"
        },
        "$addToSet": {
            "array_field": "unique_item"
        }
    };

    assert_eq!(result, expected);
}
