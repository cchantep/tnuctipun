// Test that the MongoComparable derive macro fails with a non-struct type

use nessus::{FieldWitnesses, MongoComparable};
use nessus::mongo_comparable;
use nessus::field_witnesses;
use serde::{Deserialize, Serialize};

// Try to apply MongoComparable to an enum (should fail)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub enum InvalidType {
    A(String),
    B(i32),
}

fn main() {
    // This should not compile
}
