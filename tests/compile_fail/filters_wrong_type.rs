// This test verifies that using the wrong type with filter_builder will cause a compile error

use tnuctipun::{FieldWitnesses, MongoComparable};
use tnuctipun::filters::empty;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub Price: f64,
}

fn main() {
    // This should fail to compile because we're trying to use a string for a f64 field
    let mut builder = empty::<Product>();
    builder.eq::<product_fields::Price, _>("wrong type".to_string());
}
