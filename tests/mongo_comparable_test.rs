use nessus::mongo_comparable::MongoComparable as MongoComparableTrait;
use nessus::{FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

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
        <CollectionProduct as nessus::field_witnesses::HasField<
            collectionproduct_fields::VecTags,
        >>::Value,
        String
    );

    // Test HashSet collection
    static_assert_implements_mongo_comparable!(
        CollectionProduct,
        <CollectionProduct as nessus::field_witnesses::HasField<
            collectionproduct_fields::HashSetCategories,
        >>::Value,
        String
    );

    // Test BTreeSet collection
    static_assert_implements_mongo_comparable!(
        CollectionProduct,
        <CollectionProduct as nessus::field_witnesses::HasField<
            collectionproduct_fields::BtreeSetLabels,
        >>::Value,
        i32
    );
}

// Test struct with various primitive types
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = true)]
pub struct PrimitiveTypes {
    string_field: String,
    bool_field: bool,
    i32_field: i32,
    i64_field: i64,
    f64_field: f64,
    char_field: char,
    option_field: Option<String>,
}

#[test]
fn test_primitive_types() {
    // The test passes if it compiles successfully
    // These static_assertions verify that the MongoComparable trait is implemented
    // for different primitive types

    // Test String type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::StringField>>::Value,
        String
    );

    // Test bool type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::BoolField>>::Value,
        bool
    );

    // Test i32 type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::I32Field>>::Value,
        i32
    );

    // Test i64 type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::I64Field>>::Value,
        i64
    );

    // Test f64 type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::F64Field>>::Value,
        f64
    );

    // Test char type (should convert to String)
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::CharField>>::Value,
        char
    );

    // Test Option type
    static_assert_implements_mongo_comparable!(
        PrimitiveTypes,
        <PrimitiveTypes as nessus::field_witnesses::HasField<primitivetypes_fields::OptionField>>::Value,
        String
    );
}
