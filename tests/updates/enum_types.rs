//! Tests for enum types used in updates (UpdateOperation, CurrentDateType, PopStrategy)

use tnuctipun::updates::{CurrentDateType, PopStrategy, UpdateOperation};

#[test]
fn test_update_operation_to_string() {
    assert_eq!(UpdateOperation::Set.to_string(), "$set");
    assert_eq!(UpdateOperation::Unset.to_string(), "$unset");
    assert_eq!(UpdateOperation::Inc.to_string(), "$inc");
    assert_eq!(UpdateOperation::Mul.to_string(), "$mul");
    assert_eq!(UpdateOperation::Rename.to_string(), "$rename");
    assert_eq!(UpdateOperation::CurrentDate.to_string(), "$currentDate");
    assert_eq!(UpdateOperation::AddToSet.to_string(), "$addToSet");
    assert_eq!(UpdateOperation::Pop.to_string(), "$pop");
    assert_eq!(UpdateOperation::Pull.to_string(), "$pull");
    assert_eq!(UpdateOperation::PullAll.to_string(), "$pullAll");
    assert_eq!(UpdateOperation::Push.to_string(), "$push");
}

#[test]
fn test_update_operation_consistency() {
    // Test that multiple calls to to_string() return the same value
    let set_op = UpdateOperation::Set;

    assert_eq!(set_op.to_string(), set_op.to_string());

    let unset_op = UpdateOperation::Unset;

    assert_eq!(unset_op.to_string(), unset_op.to_string());
}

#[test]
fn test_update_operation_all_variants_covered() {
    // This test ensures all enum variants are explicitly tested
    let operations = [
        UpdateOperation::Set,
        UpdateOperation::Unset,
        UpdateOperation::Inc,
        UpdateOperation::Mul,
        UpdateOperation::Rename,
        UpdateOperation::CurrentDate,
        UpdateOperation::AddToSet,
        UpdateOperation::Pop,
        UpdateOperation::Pull,
        UpdateOperation::PullAll,
        UpdateOperation::Push,
    ];

    let expected_strings = [
        "$set",
        "$unset",
        "$inc",
        "$mul",
        "$rename",
        "$currentDate",
        "$addToSet",
        "$pop",
        "$pull",
        "$pullAll",
        "$push",
    ];

    assert_eq!(operations.len(), expected_strings.len());

    for (op, expected) in operations.iter().zip(expected_strings.iter()) {
        assert_eq!(op.to_string(), *expected);
    }
}

#[test]
fn test_update_operation_string_format() {
    // Test that all operations start with '$' as expected by MongoDB
    let operations = vec![
        UpdateOperation::Set,
        UpdateOperation::Unset,
        UpdateOperation::Inc,
        UpdateOperation::Mul,
        UpdateOperation::Rename,
        UpdateOperation::CurrentDate,
        UpdateOperation::AddToSet,
        UpdateOperation::Pop,
        UpdateOperation::Pull,
        UpdateOperation::PullAll,
        UpdateOperation::Push,
    ];

    for op in operations {
        let string_repr = op.to_string();

        assert!(
            string_repr.starts_with('$'),
            "Operation {string_repr} should start with '$'"
        );

        assert!(
            !string_repr.is_empty(),
            "Operation string should not be empty"
        );
    }
}

#[test]
fn test_update_operation_hash_equality() {
    use std::collections::HashSet;

    // Test that the same operation variants are equal and hash to the same value
    let mut set = HashSet::new();

    set.insert(UpdateOperation::Set);
    set.insert(UpdateOperation::Set); // Duplicate should not increase size

    assert_eq!(set.len(), 1);

    // Test that different operations are not equal
    set.insert(UpdateOperation::Unset);

    assert_eq!(set.len(), 2);
}

// Tests for CurrentDateType enum
#[test]
fn test_current_date_type_to_string() {
    assert_eq!(CurrentDateType::Date.to_string(), "date");
    assert_eq!(CurrentDateType::Timestamp.to_string(), "timestamp");
}

#[test]
fn test_current_date_type_consistency() {
    let date_type = CurrentDateType::Date;

    assert_eq!(date_type.to_string(), date_type.to_string());

    let timestamp_type = CurrentDateType::Timestamp;

    assert_eq!(timestamp_type.to_string(), timestamp_type.to_string());
}

// Tests for PopStrategy enum
#[test]
fn test_pop_strategy_from_conversion() {
    let first_strategy = PopStrategy::First;
    let first_bson: bson::Bson = first_strategy.into();

    assert_eq!(first_bson, bson::Bson::Int32(-1));

    let last_strategy = PopStrategy::Last;
    let last_bson: bson::Bson = last_strategy.into();

    assert_eq!(last_bson, bson::Bson::Int32(1));
}

#[test]
fn test_pop_strategy_consistency() {
    let strategy = PopStrategy::First;
    let bson1: bson::Bson = strategy.into();
    let bson2: bson::Bson = PopStrategy::First.into();

    assert_eq!(bson1, bson2);
}
