// This test verifies that ordering filters reject string slices when only
// MongoComparable evidence exists but MongoOrdered is missing.

use serde::{Deserialize, Serialize};
use tnuctipun::filters::empty;
use tnuctipun::mongo_comparable::MongoComparable;
use tnuctipun::FieldWitnesses;

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Product {
    pub Name: String,
}

impl MongoComparable<String, &'static str> for Product {}

fn main() {
    // This must fail because MongoOrdered<String, &str> is not implemented.
    let mut builder = empty::<Product>();
    builder.gt::<product_fields::Name, _>("abc");
}
