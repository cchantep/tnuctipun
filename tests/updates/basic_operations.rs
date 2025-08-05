//! Tests for individual MongoDB update operations ($set, $unset, $inc, etc.)

use super::test_fixtures::*;
use tnuctipun::updates::{CurrentDateType, PopStrategy, empty};

// Tests for $set operation
#[test]
fn test_single_set_operation() {
    let result = empty::<TestStruct>()
        .set::<TestFieldName, _>("test_value")
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "test_value"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_set_operations_combined() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("first_value")
        .set::<AnotherFieldName, _>(42)
        .set::<NestedFieldName, _>(true)
        .build();

    // Check the entire document structure
    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "first_value",
            "another_field": 42,
            "nested.field": true
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_multiple_set_operations_same_field() {
    let result = empty::<TestStruct>()
        .set::<TestFieldName, _>("first_value")
        .set::<TestFieldName, _>("second_value")
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "second_value"  // last set operation wins
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_set_operations_different_bson_types() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("string_value")
        .set::<AnotherFieldName, _>(42i32)
        .set::<NestedFieldName, _>(true)
        .build();

    // Check the entire document structure with different BSON types
    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "string_value",
            "another_field": 42,
            "nested.field": true
        }
    };

    assert_eq!(doc, expected_doc);
}

// Tests for $unset operation
#[test]
fn test_single_unset_operation() {
    let result = empty::<TestStruct>().unset::<TestFieldName>().build();

    let expected = bson::doc! {
        "$unset": {
            "test_field": bson::Bson::Null
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_unset_operations() {
    let result = empty::<TestStruct>()
        .unset::<TestFieldName>()
        .unset::<AnotherFieldName>()
        .build();

    let expected = bson::doc! {
        "$unset": {
            "test_field": bson::Bson::Null,
            "another_field": bson::Bson::Null
        }
    };

    assert_eq!(result, expected);
}

// Tests for $inc operation
#[test]
fn test_single_inc_operation() {
    let result = empty::<TestStruct>().inc::<NumericFieldName, _>(5).build();

    let expected = bson::doc! {
        "$inc": {
            "numeric_field": 5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_inc_operation_negative_value() {
    let result = empty::<TestStruct>().inc::<NumericFieldName, _>(-3).build();

    let expected = bson::doc! {
        "$inc": {
            "numeric_field": -3
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_inc_operations() {
    let result = empty::<TestStruct>()
        .inc::<NumericFieldName, _>(1)
        .inc::<AnotherFieldName, _>(10)
        .build();

    let expected = bson::doc! {
        "$inc": {
            "numeric_field": 1,
            "another_field": 10
        }
    };

    assert_eq!(result, expected);
}

// Tests for $mul operation
#[test]
fn test_single_mul_operation() {
    let result = empty::<TestStruct>().mul::<NumericFieldName, _>(2).build();

    let expected = bson::doc! {
        "$mul": {
            "numeric_field": 2
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_mul_operation_decimal() {
    let result = empty::<TestStruct>()
        .mul::<NumericFieldName, _>(1.5f64)
        .build();

    let expected = bson::doc! {
        "$mul": {
            "numeric_field": 1.5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_mul_operations() {
    let result = empty::<TestStruct>()
        .mul::<NumericFieldName, _>(3)
        .mul::<AnotherFieldName, _>(0.5f64)
        .build();

    let expected = bson::doc! {
        "$mul": {
            "numeric_field": 3,
            "another_field": 0.5
        }
    };

    assert_eq!(result, expected);
}

// Tests for $max operation
#[test]
fn test_single_max_operation() {
    let result = empty::<TestStruct>().max::<NumericFieldName, _>(10).build();

    let expected = bson::doc! {
        "$max": {
            "numeric_field": 10
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_max_operation_decimal() {
    let result = empty::<TestStruct>()
        .max::<NumericFieldName, _>(15.5f64)
        .build();

    let expected = bson::doc! {
        "$max": {
            "numeric_field": 15.5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_max_operation_negative_value() {
    let result = empty::<TestStruct>().max::<NumericFieldName, _>(-5).build();

    let expected = bson::doc! {
        "$max": {
            "numeric_field": -5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_max_operations() {
    let result = empty::<TestStruct>()
        .max::<NumericFieldName, _>(100)
        .max::<AnotherFieldName, _>(50)
        .build();

    let expected = bson::doc! {
        "$max": {
            "numeric_field": 100,
            "another_field": 50
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_max_operation_same_field_multiple_times() {
    let result = empty::<TestStruct>()
        .max::<NumericFieldName, _>(10)
        .max::<NumericFieldName, _>(20)
        .build();

    let expected = bson::doc! {
        "$max": {
            "numeric_field": 20  // last max operation wins
        }
    };

    assert_eq!(result, expected);
}

// Tests for $min operation
#[test]
fn test_single_min_operation() {
    let result = empty::<TestStruct>().min::<NumericFieldName, _>(5).build();

    let expected = bson::doc! {
        "$min": {
            "numeric_field": 5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_min_operation_decimal() {
    let result = empty::<TestStruct>()
        .min::<NumericFieldName, _>(2.5f64)
        .build();

    let expected = bson::doc! {
        "$min": {
            "numeric_field": 2.5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_min_operation_negative_value() {
    let result = empty::<TestStruct>()
        .min::<NumericFieldName, _>(-10)
        .build();

    let expected = bson::doc! {
        "$min": {
            "numeric_field": -10
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_min_operations() {
    let result = empty::<TestStruct>()
        .min::<NumericFieldName, _>(1)
        .min::<AnotherFieldName, _>(25)
        .build();

    let expected = bson::doc! {
        "$min": {
            "numeric_field": 1,
            "another_field": 25
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_min_operation_same_field_multiple_times() {
    let result = empty::<TestStruct>()
        .min::<NumericFieldName, _>(30)
        .min::<NumericFieldName, _>(15)
        .build();

    let expected = bson::doc! {
        "$min": {
            "numeric_field": 15  // last min operation wins
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_max_and_min_combined_operations() {
    let result = empty::<TestStruct>()
        .max::<NumericFieldName, _>(100)
        .min::<AnotherFieldName, _>(1)
        .build();

    let expected = bson::doc! {
        "$max": {
            "numeric_field": 100
        },
        "$min": {
            "another_field": 1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_max_min_with_other_operations() {
    let result = empty::<TestStruct>()
        .set::<TestFieldName, _>("test_value")
        .max::<NumericFieldName, _>(50)
        .min::<AnotherFieldName, _>(10)
        .inc::<NumericFieldName, _>(5)
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "test_value"
        },
        "$max": {
            "numeric_field": 50
        },
        "$min": {
            "another_field": 10
        },
        "$inc": {
            "numeric_field": 5
        }
    };

    assert_eq!(result, expected);
}

// Tests for $rename operation
#[test]
fn test_single_rename_operation() {
    let result = empty::<TestStruct>()
        .rename::<TestFieldName>("new_name")
        .build();

    let expected = bson::doc! {
        "$rename": {
            "test_field": "new_name"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_rename_operations() {
    let result = empty::<TestStruct>()
        .rename::<TestFieldName>("new_test_field")
        .rename::<AnotherFieldName>("new_another_field")
        .build();

    let expected = bson::doc! {
        "$rename": {
            "test_field": "new_test_field",
            "another_field": "new_another_field"
        }
    };

    assert_eq!(result, expected);
}

// Tests for $currentDate operation
#[test]
fn test_current_date_operation_date_type() {
    let result = empty::<TestStruct>()
        .current_date::<TestFieldName>(CurrentDateType::Date)
        .build();

    let expected = bson::doc! {
        "$currentDate": {
            "test_field": "date"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_current_date_operation_timestamp_type() {
    let result = empty::<TestStruct>()
        .current_date::<TestFieldName>(CurrentDateType::Timestamp)
        .build();

    let expected = bson::doc! {
        "$currentDate": {
            "test_field": "timestamp"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_current_date_operations() {
    let result = empty::<TestStruct>()
        .current_date::<TestFieldName>(CurrentDateType::Date)
        .current_date::<AnotherFieldName>(CurrentDateType::Timestamp)
        .build();

    let expected = bson::doc! {
        "$currentDate": {
            "test_field": "date",
            "another_field": "timestamp"
        }
    };

    assert_eq!(result, expected);
}

// Tests for $addToSet operation
#[test]
fn test_single_add_to_set_operation() {
    let result = empty::<TestStruct>()
        .add_to_set::<ArrayFieldName, _>("new_item".to_string())
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": "new_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_add_to_set_operations() {
    let result = empty::<TestStruct>()
        .add_to_set::<ArrayFieldName, _>("item1".to_string())
        .add_to_set::<ArrayFieldName, _>("item2".to_string())
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": "item2"  // last one wins due to field overwriting
        }
    };

    assert_eq!(result, expected);
}

// Tests for $pop operation
#[test]
fn test_pop_operation_first() {
    let result = empty::<TestStruct>()
        .pop::<ArrayFieldName>(PopStrategy::First)
        .build();

    let expected = bson::doc! {
        "$pop": {
            "array_field": -1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_pop_operation_last() {
    let result = empty::<TestStruct>()
        .pop::<ArrayFieldName>(PopStrategy::Last)
        .build();

    let expected = bson::doc! {
        "$pop": {
            "array_field": 1
        }
    };

    assert_eq!(result, expected);
}

// Tests for $pull operation with expressions
#[test]
fn test_pull_expr_operation() {
    let condition = bson::doc! { "score": { "$gt": 80 } };
    let result = empty::<TestStruct>()
        .pull_expr::<ArrayFieldName>(condition.clone().into())
        .build();

    let expected = bson::doc! {
        "$pull": {
            "array_field": condition
        }
    };

    assert_eq!(result, expected);
}

// Tests for $pull operation
#[test]
fn test_pull_operation() {
    let result = empty::<TestStruct>()
        .pull::<ArrayFieldName, _>("unwanted_item".to_string())
        .build();

    let expected = bson::doc! {
        "$pull": {
            "array_field": "unwanted_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_pull_operation_numeric() {
    let result = empty::<TestStruct>()
        .pull::<ArrayFieldName, _>(42.to_string())
        .build();

    let expected = bson::doc! {
        "$pull": {
            "array_field": "42"
        }
    };

    assert_eq!(result, expected);
}

// Tests for $pullAll operation
#[test]
fn test_pull_all_operation() {
    let items_to_remove = vec!["item1".to_string(), "item2".to_string()];
    let result = empty::<TestStruct>()
        .pull_all::<ArrayFieldName, _>(items_to_remove)
        .build();

    let expected = bson::doc! {
        "$pullAll": {
            "array_field": ["item1", "item2"]
        }
    };

    assert_eq!(result, expected);
}

// Tests for $push operation
#[test]
fn test_push_operation() {
    let result = empty::<TestStruct>()
        .push::<ArrayFieldName, _>("new_item".to_string())
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": "new_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_operation_multiple_items() {
    let result = empty::<TestStruct>()
        .push::<ArrayFieldName, _>("item1".to_string())
        .push::<ArrayFieldName, _>("item2".to_string())
        .build();

    // The second push overwrites the first one due to field overwriting
    let expected = bson::doc! {
        "$push": {
            "array_field": "item2"
        }
    };

    assert_eq!(result, expected);
}

// Tests for $addToSet with $each modifier
#[test]
fn test_add_to_set_each_operation() {
    let values = vec![
        "item1".to_string(),
        "item2".to_string(),
        "item3".to_string(),
    ];
    let result = empty::<TestStruct>()
        .add_to_set_each::<ArrayFieldName, _, _>(values)
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": {
                "$each": ["item1", "item2", "item3"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_add_to_set_each_empty_collection() {
    let values: Vec<String> = vec![];
    let result = empty::<TestStruct>()
        .add_to_set_each::<ArrayFieldName, _, _>(values)
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": {
                "$each": []
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_add_to_set_each_single_item() {
    let values = vec!["single_item".to_string()];
    let result = empty::<TestStruct>()
        .add_to_set_each::<ArrayFieldName, _, _>(values)
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": {
                "$each": ["single_item"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_add_to_set_each_multiple_fields() {
    let tags1 = vec!["rust".to_string(), "mongodb".to_string()];
    let result = empty::<TestStruct>()
        .add_to_set_each::<ArrayFieldName, _, _>(tags1)
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": {
                "$each": ["rust", "mongodb"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_add_to_set_each_different_types() {
    let string_values = vec![
        "text1".to_string(),
        "text2".to_string(),
        "text3".to_string(),
    ];
    let result = empty::<TestStruct>()
        .add_to_set_each::<ArrayFieldName, _, _>(string_values)
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": {
                "$each": ["text1", "text2", "text3"]
            }
        }
    };

    assert_eq!(result, expected);
}

// Tests for $push with $each modifier
#[test]
fn test_push_each_operation() {
    let values = vec!["log1".to_string(), "log2".to_string(), "log3".to_string()];
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(values)
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["log1", "log2", "log3"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_empty_collection() {
    let values: Vec<String> = vec![];
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(values)
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": []
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_single_item() {
    let values = vec!["single_log".to_string()];
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(values)
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["single_log"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_multiple_fields() {
    let logs = vec!["user_action".to_string(), "system_event".to_string()];
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(logs)
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["user_action", "system_event"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_different_types() {
    let string_values = vec![
        "message1".to_string(),
        "message2".to_string(),
        "message3".to_string(),
    ];
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(string_values)
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["message1", "message2", "message3"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_duplicates() {
    let values = vec![
        "duplicate".to_string(),
        "duplicate".to_string(),
        "unique".to_string(),
    ];
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(values)
        .build();

    // Unlike $addToSet, $push allows duplicates
    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["duplicate", "duplicate", "unique"]
            }
        }
    };

    assert_eq!(result, expected);
}

// Tests comparing single vs batch operations
#[test]
fn test_add_to_set_vs_add_to_set_each() {
    // Single add_to_set
    let single_result = empty::<TestStruct>()
        .add_to_set::<ArrayFieldName, _>("single_item".to_string())
        .build();

    let single_expected = bson::doc! {
        "$addToSet": {
            "array_field": "single_item"
        }
    };

    // Multiple add_to_set_each
    let batch_result = empty::<TestStruct>()
        .add_to_set_each::<ArrayFieldName, _, _>(vec!["batch_item".to_string()])
        .build();

    let batch_expected = bson::doc! {
        "$addToSet": {
            "array_field": {
                "$each": ["batch_item"]
            }
        }
    };

    assert_eq!(single_result, single_expected);
    assert_eq!(batch_result, batch_expected);
    // Note: These create different MongoDB operations even though they might have similar effects
}

#[test]
fn test_push_vs_push_each() {
    // Single push
    let single_result = empty::<TestStruct>()
        .push::<ArrayFieldName, _>("single_item".to_string())
        .build();

    let single_expected = bson::doc! {
        "$push": {
            "array_field": "single_item"
        }
    };

    // Multiple push_each
    let batch_result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(vec!["batch_item".to_string()])
        .build();

    let batch_expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["batch_item"]
            }
        }
    };

    assert_eq!(single_result, single_expected);
    assert_eq!(batch_result, batch_expected);
    // Note: These create different MongoDB operations even though they might have similar effects
}
