// This test verifies that using a nonexistent field with filter_builder will cause a compile error

use nessus::filters::empty;
use nessus::field_witnesses::FieldName;
use nessus::{FieldWitnesses, MongoComparable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    Name: String,
    Price: f64,
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
