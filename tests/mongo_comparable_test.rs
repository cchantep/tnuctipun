use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};
use tnuctipun::mongo_comparable::{
    MongoComparable as MongoComparableTrait, MongoOrdered as MongoOrderedTrait,
};
use tnuctipun::{FieldWitnesses, MongoComparable};

// Macro to statically assert that a type implements MongoComparable
macro_rules! static_assert_implements_mongo_comparable {
    ($struct_type:ty, $field_type:ty, $value_type:ty) => {
        const _: fn() = || {
            // This function only type-checks and is never called
            fn assert_implements_mongo_comparable<T, A, B>()
            where
                T: MongoComparableTrait<A, B>,
            {
            }

            assert_implements_mongo_comparable::<$struct_type, $field_type, $value_type>();
        };
    };
}

// Macro to statically assert that a type implements MongoOrdered
macro_rules! static_assert_implements_mongo_ordered {
    ($struct_type:ty, $field_type:ty, $value_type:ty) => {
        const _: fn() = || {
            fn assert_implements_mongo_ordered<T, A, B>()
            where
                T: MongoOrderedTrait<A, B>,
            {
            }

            assert_implements_mongo_ordered::<$struct_type, $field_type, $value_type>();
        };
    };
}

// Test struct with different collection types
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct CollectionProduct {
    pub id: String,
    pub vec_tags: Vec<String>,
    pub hash_set_categories: HashSet<String>,
    pub btree_set_labels: BTreeSet<i32>,
}

#[test]
fn test_collection_types() {
    // The test passes if it compiles successfully
    // These static_assertions verify that the MongoComparable trait is implemented
    // for different collection types

    // Test Vector collection
    static_assert_implements_mongo_comparable!(
        CollectionProduct,
        <CollectionProduct as tnuctipun::field_witnesses::HasField<
            collectionproduct_fields::VecTags,
        >>::Value,
        String
    );

    // Test HashSet collection
    static_assert_implements_mongo_comparable!(
        CollectionProduct,
        <CollectionProduct as tnuctipun::field_witnesses::HasField<
            collectionproduct_fields::HashSetCategories,
        >>::Value,
        String
    );

    // Test BTreeSet collection
    static_assert_implements_mongo_comparable!(
        CollectionProduct,
        <CollectionProduct as tnuctipun::field_witnesses::HasField<
            collectionproduct_fields::BtreeSetLabels,
        >>::Value,
        i32
    );
}

// Test struct with various primitive types
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[tnuctipun(include_private = true)]
pub struct PrimitiveTypes {
    string_field: String,
    bool_field: bool,
    i32_field: i32,
    i64_field: i64,
    f64_field: f64,
    char_field: char,
    option_field: Option<String>,
}

#[derive(Debug, Clone, FieldWitnesses, MongoComparable)]
pub struct TemporalTypes {
    pub timestamp: DateTime<Utc>,
}

#[test]
fn test_primitive_types() {
    // The test passes if it compiles successfully
    // These static_assertions verify that the MongoComparable trait is implemented
    // for different primitive types

    // Test String type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<
            primitivetypes_fields::StringField,
        >>::Value,
        String
    );

    // Test bool type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<
            primitivetypes_fields::BoolField,
        >>::Value,
        bool
    );

    // Test i32 type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<primitivetypes_fields::I32Field>>::Value,
        i32
    );

    // Test i64 type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<primitivetypes_fields::I64Field>>::Value,
        i64
    );

    // Test f64 type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<primitivetypes_fields::F64Field>>::Value,
        f64
    );

    // Test char type (should convert to String)
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<
            primitivetypes_fields::CharField,
        >>::Value,
        char
    );

    // Test Option type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<
            primitivetypes_fields::OptionField,
        >>::Value,
        String
    );
}

#[test]
fn test_mongo_ordered_numeric_and_temporal_types() {
    // f64 field is ordered with numeric compatible types such as i32.
    static_assert_implements_mongo_ordered!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<primitivetypes_fields::F64Field>>::Value,
        i32
    );

    // Same-type numeric ordering must be supported.
    static_assert_implements_mongo_ordered!(
        PrimitiveTypes,
        <PrimitiveTypes as tnuctipun::field_witnesses::HasField<primitivetypes_fields::I32Field>>::Value,
        i32
    );

    // DateTime field is ordered and comparable with i64 timestamps.
    static_assert_implements_mongo_ordered!(
        TemporalTypes,
        <TemporalTypes as tnuctipun::field_witnesses::HasField<temporaltypes_fields::Timestamp>>::Value,
        i64
    );
}
