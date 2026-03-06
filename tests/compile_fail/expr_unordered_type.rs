// This test verifies that expression ordering operators reject non-ordered types.

use serde::{Deserialize, Serialize};
use tnuctipun::expr;
use tnuctipun::{FieldWitnesses, MongoComparable};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub Name: String,
}

fn main() {
    // This must fail because String is comparable but not ordered for $gt.
    let b = expr::empty::<Product>();
    let left = b.select::<product_fields::Name>();
    let right = b.from("abc".to_string());

    let _ = b.gt(left, right);
}
