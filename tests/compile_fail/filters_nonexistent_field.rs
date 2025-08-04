// This test verifies that using a nonexistent field with filter_builder will cause a compile error

use tnuctipun::{FieldWitnesses, MongoComparable};
use tnuctipun::filters::empty;
use tnuctipun::field_witnesses::FieldName;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub Name: String,
    pub Price: f64,
}

// Defining a field that doesn't exist in Product
struct Weight;
impl FieldName for Weight {
    fn field_name() -> &'static str {
        "Weight"
    }
}

fn main() {
    // This should fail to compile because the Product struct doesn't have a Weight field
    let mut builder = empty::<Product>();
    builder.eq::<Weight, _>(10.5);
}
