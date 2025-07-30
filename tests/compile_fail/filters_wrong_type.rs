// This test verifies that using the wrong type with filter_builder will cause a compile error

use nessus::filters::empty;
use nessus::{FieldWitnesses, MongoComparable};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    Price: f64,
}

fn main() {
    // This should fail to compile because we're trying to use a string for a f64 field
    let mut builder = empty::<Product>();
    builder.eq::<product_fields::Price, _>("wrong type".to_string());
}
