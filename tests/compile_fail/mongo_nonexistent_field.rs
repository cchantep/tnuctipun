// This test verifies that MongoComparable can't be used with a non-existent field

use tnuctipun::{FieldWitnesses, MongoComparable, HasField, FieldName};
use tnuctipun::mongo_comparable::MongoComparable as MongoComparableTrait;
use tnuctipun::HasField;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub Name: String,
    pub Price: f64,
}

// Define a field witness for a nonexistent field
#[allow(non_camel_case_types)]
struct Weight;

impl tnuctipun::field_witnesses::FieldName for Weight {
    fn field_name() -> &'static str { "Weight" }
}

fn main() {
    // This should fail because Product doesn't have a Weight field
    // The MongoComparable trait requires HasField to be implemented
    fn assert_implements_mongo_comparable<T, A, B>()
    where
        T: MongoComparableTrait<A, B>
    {}
    
    assert_implements_mongo_comparable::<Product, 
        <Product as tnuctipun::field_witnesses::HasField<Weight>>::Value, 
        f64>();
}
