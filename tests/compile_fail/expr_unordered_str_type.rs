// This test verifies that expression ordering operators reject string slices
// when only MongoComparable evidence exists but MongoOrdered is missing.

use serde::{Deserialize, Serialize};
use tnuctipun::expr;
use tnuctipun::mongo_comparable::MongoComparable;
use tnuctipun::FieldWitnesses;

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Product {
    pub Name: String,
}

impl MongoComparable<String, &'static str> for Product {}

fn main() {
    // This must fail because MongoOrdered<String, &str> is not implemented.
    let b = expr::empty::<Product>();
    let left = b.select::<product_fields::Name>();
    let right = b.from("abc");

    let _ = b.gt(left, right);
}
