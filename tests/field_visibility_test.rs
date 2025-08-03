use nessus::{FieldName, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

// Test struct with private fields (no pub modifier) and include_private = true
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = true)]
struct PrivateFieldsTest {
    name: String, // private field - included
    age: i32,     // private field - included
}

// Test struct with public fields (with pub modifier)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct PublicFieldsTest {
    pub name: String, // public field
    pub age: i32,     // public field
}

// Test struct with mixed visibility and include_private = true
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = true)]
pub struct MixedVisibilityTest {
    name: String,      // private field - included
    pub age: i32,      // public field
    pub email: String, // public field
    internal_id: u64,  // private field - included
}

#[test]
fn test_field_visibility_with_include_private() {
    // This should compile and work fine with include_private = true
    let private_test = PrivateFieldsTest {
        name: "Alice".to_string(),
        age: 30,
    };

    let public_test = PublicFieldsTest {
        name: "Bob".to_string(),
        age: 25,
    };

    let mixed_test = MixedVisibilityTest {
        name: "Charlie".to_string(),
        age: 35,
        email: "charlie@example.com".to_string(),
        internal_id: 12345,
    };

    // Test that field witnesses are generated for all fields when include_private = true
    assert_eq!(privatefieldstest_fields::Name::field_name(), "name");
    assert_eq!(privatefieldstest_fields::Age::field_name(), "age");

    assert_eq!(publicfieldstest_fields::Name::field_name(), "name");
    assert_eq!(publicfieldstest_fields::Age::field_name(), "age");

    assert_eq!(mixedvisibilitytest_fields::Name::field_name(), "name");
    assert_eq!(mixedvisibilitytest_fields::Age::field_name(), "age");
    assert_eq!(mixedvisibilitytest_fields::Email::field_name(), "email");
    assert_eq!(
        mixedvisibilitytest_fields::InternalId::field_name(),
        "internal_id"
    );

    println!("All tests compiled successfully!");
    println!("Private fields test: {private_test:?}");
    println!("Public fields test: {public_test:?}");
    println!("Mixed visibility test: {mixed_test:?}");
}
