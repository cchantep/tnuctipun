// This test verifies that MongoComparable works correctly with collection types

use tnuctipun::{FieldWitnesses, MongoComparable, HasField, FieldName};
use tnuctipun::mongo_comparable::MongoComparable as MongoComparableTrait;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    Name: String,
    Price: f64,
    Tags: Vec<String>,
}

// Custom collection type that doesn't implement IntoIterator correctly
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CustomCollection;

fn main() {
    // This should fail because CustomCollection doesn't implement IntoIterator
    fn assert_implements_mongo_comparable<T, A, B>()
    where
        T: MongoComparableTrait<A, B>
    {}
    
    // Define a struct with a custom collection field
    #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    struct CustomProduct {
        pub Id: String,
        pub BadCollection: CustomCollection,
    }
    
    // Attempt to compare with the custom collection
    assert_implements_mongo_comparable::<CustomProduct, 
        <CustomProduct as tnuctipun::field_witnesses::HasField<customproduct_fields::BadCollection>>::Value, 
        String>();
}
