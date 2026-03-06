// This test verifies that ordering filters reject non-ordered types.

use serde::{Deserialize, Serialize};
use tnuctipun::filters::empty;
use tnuctipun::{FieldWitnesses, MongoComparable};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub Name: String,
}

fn main() {
    // This must fail because String is comparable but not ordered for $gt.
    let mut builder = empty::<Product>();
    builder.gt::<product_fields::Name, _>("abc".to_string());
}
