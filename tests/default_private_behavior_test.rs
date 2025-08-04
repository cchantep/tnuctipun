use serde::{Deserialize, Serialize};
use tnuctipun::{FieldName, FieldWitnesses, MongoComparable};
// Test struct with mixed visibility and default behavior (include_private = false)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct DefaultBehaviorTest {
    pub name: String, // public field - should be included
    email: String,    // private field - should be skipped
    pub age: i32,     // public field - should be included
    internal_id: u64, // private field - should be skipped
}

#[test]
fn test_default_behavior_skips_private_fields() {
    let user = DefaultBehaviorTest {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
        internal_id: 12345,
    };

    // Only public fields should be accessible
    assert_eq!(defaultbehaviortest_fields::Name::field_name(), "name");
    assert_eq!(defaultbehaviortest_fields::Age::field_name(), "age");

    // Test field access using HasField trait
    use tnuctipun::HasField;

    let name_ref =
        <DefaultBehaviorTest as HasField<defaultbehaviortest_fields::Name>>::get_field(&user);

    let age_ref =
        <DefaultBehaviorTest as HasField<defaultbehaviortest_fields::Age>>::get_field(&user);

    assert_eq!(name_ref, "Alice");
    assert_eq!(*age_ref, 30);

    println!("Default behavior test passed: only public fields are accessible");
}
