//! Tests for PushEach functionality with different modifiers and combinations

use super::test_fixtures::*;
use bson::{Bson, doc};
use tnuctipun::updates::{
    PushEach, PushEachPosition, PushEachSlice, PushEachSort, UpdateOperation, empty,
};

#[test]
fn test_push_each_with_position_last() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            slice: None,
            sort: None,
            position: Some(PushEachPosition::PushTakeLast(1)),
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["1", "2", "3"],
                "$position": -1
            }
        }
    };

    assert_eq!(result, expected);
}

// Tests for PushEach with different modifiers
#[test]
fn test_push_each_basic() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ])
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["1", "2", "3"]
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_slice_first() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            slice: Some(PushEachSlice::PushFirstSlice(5)),
            sort: None,
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["1", "2", "3"],
                "$slice": 5
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_slice_last() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            slice: Some(PushEachSlice::PushLastSlice(10)),
            sort: None,
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["a", "b", "c"],
                "$slice": -10
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_slice_empty() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            slice: Some(PushEachSlice::PushEmptySlice),
            sort: None,
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["1", "2", "3"],
                "$slice": 0
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_sort_ascending() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["3".to_string(), "1".to_string(), "2".to_string()],
            slice: None,
            sort: Some(PushEachSort::PushSortAscending),
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["3", "1", "2"],
                "$sort": 1
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_sort_descending() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["1".to_string(), "3".to_string(), "2".to_string()],
            slice: None,
            sort: Some(PushEachSort::PushSortDescending),
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["1", "3", "2"],
                "$sort": -1
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_sort_expression() {
    let sort_expr = doc! { "score": -1, "name": 1 };
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["element1".to_string(), "element2".to_string()],
            slice: None,
            sort: Some(PushEachSort::PushSortExpression(sort_expr.clone())),
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["element1", "element2"],
                "$sort": sort_expr
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_position_first() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            slice: None,
            sort: None,
            position: Some(PushEachPosition::PushTakeFirst(0)),
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["1", "2", "3"],
                "$position": 0
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_position_middle() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["x".to_string(), "y".to_string()],
            slice: None,
            sort: None,
            position: Some(PushEachPosition::PushTakeFirst(5)),
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["x", "y"],
                "$position": 5
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_with_all_modifiers() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["3".to_string(), "1".to_string(), "2".to_string()],
            slice: Some(PushEachSlice::PushFirstSlice(3)),
            sort: Some(PushEachSort::PushSortAscending),
            position: Some(PushEachPosition::PushTakeFirst(0)),
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": {
                "$each": ["3", "1", "2"],
                "$slice": 3,
                "$sort": 1,
                "$position": 0
            }
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_each_method_chaining() {
    let result = empty::<TestStruct>()
        .push_each::<ArrayFieldName, _, _, _>(PushEach {
            values: vec!["10".to_string(), "20".to_string(), "30".to_string()],
            slice: Some(PushEachSlice::PushLastSlice(5)),
            sort: Some(PushEachSort::PushSortDescending),
            position: None,
        })
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": doc! {
        "$each": ["10", "20", "30"],
        "$sort": -1,
        "$slice": -5
    }
        }
    };

    assert_eq!(result, expected);
}

// Tests for untyped update operations
#[test]
fn test_untyped_update_operation() {
    let complex_expr = doc! {
        "$each": ["1", "2", "3"],
        "$slice": -5,
        "$sort": 1
    };

    let result = empty::<TestStruct>()
        .untyped::<ArrayFieldName>(UpdateOperation::Push, Bson::Document(complex_expr.clone()))
        .build();

    let expected = bson::doc! {
        "$push": {
            "array_field": complex_expr
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_with_multiple_operations() {
    let result = empty::<TestStruct>()
        .untyped::<TestFieldName>(
            UpdateOperation::Set,
            Bson::String("custom_value".to_string()),
        )
        .untyped::<NumericFieldName>(UpdateOperation::Inc, Bson::Int32(5))
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "custom_value"
        },
        "$inc": {
            "numeric_field": 5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_untyped_with_conditional_expression() {
    let condition = doc! {
        "$cond": {
            "if": { "$gt": ["$score", 100] },
            "then": 200,
            "else": 100
        }
    };

    let result = empty::<TestStruct>()
        .untyped::<NumericFieldName>(UpdateOperation::Set, Bson::Document(condition.clone()))
        .build();

    let expected = bson::doc! {
        "$set": {
            "numeric_field": condition
        }
    };

    assert_eq!(result, expected);
}

// Tests for PushEach enum conversions
#[test]
fn test_push_each_slice_conversions() {
    let empty_slice: Bson = PushEachSlice::PushEmptySlice.into();

    assert_eq!(empty_slice, Bson::Int32(0));

    let first_slice: Bson = PushEachSlice::PushFirstSlice(5).into();

    assert_eq!(first_slice, Bson::Int32(5));

    let last_slice: Bson = PushEachSlice::PushLastSlice(3).into();

    assert_eq!(last_slice, Bson::Int32(-3));
}

#[test]
fn test_push_each_sort_conversions() {
    let asc_sort: Bson = PushEachSort::PushSortAscending.into();

    assert_eq!(asc_sort, Bson::Int32(1));

    let desc_sort: Bson = PushEachSort::PushSortDescending.into();

    assert_eq!(desc_sort, Bson::Int32(-1));

    let expr_sort = doc! { "field": 1 };
    let custom_sort: Bson = PushEachSort::PushSortExpression(expr_sort.clone()).into();

    assert_eq!(custom_sort, Bson::Document(expr_sort));
}

#[test]
fn test_push_each_position_conversions() {
    let first_pos: Bson = PushEachPosition::PushTakeFirst(3).into();

    assert_eq!(first_pos, Bson::Int32(3));

    let last_pos: Bson = PushEachPosition::PushTakeLast(2).into();

    assert_eq!(last_pos, Bson::Int32(-2));
}
