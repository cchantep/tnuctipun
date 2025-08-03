// This test verifies that private fields are not accessible by default (include_private = false)

use nessus::{FieldName, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

// Test struct with mixed visibility and default behavior (include_private = false)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct DefaultBehaviorTest {
    pub name: String,      // public field - should be included
    email: String,         // private field - should be skipped
    pub age: i32,          // public field - should be included
    internal_id: u64,      // private field - should be skipped
}

fn main() {
    // These should fail to compile because private fields are skipped by default:
    let _ = defaultbehaviortest_fields::Email::field_name();
    let _ = defaultbehaviortest_fields::InternalId::field_name();
}
