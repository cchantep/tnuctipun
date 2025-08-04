use serde::{Deserialize, Serialize};
use tnuctipun::{FieldName, FieldWitnesses, HasField, NonEmptyStruct};
// Test struct with the derive macro
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
pub struct DeriveUser {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
    pub score: f64,
}

#[test]
fn test_derive_field_witnesses_macro() {
    // Create a user with the derive macro
    let user = DeriveUser {
        name: "Jane Doe".to_string(),
        age: 28,
        email: "jane.doe@example.com".to_string(),
        is_active: true,
        score: 95.5,
    };

    // Check that the struct implements NonEmptyStruct at compile time
    fn _assert_implements_non_empty_struct<T: NonEmptyStruct>() {}
    _assert_implements_non_empty_struct::<DeriveUser>();

    // Test field access using auto-generated field witnesses
    // The derive macro generates types with the same names as the fields in a scoped module
    let name_ref = <DeriveUser as HasField<deriveuser_fields::Name>>::get_field(&user);
    let age_ref = <DeriveUser as HasField<deriveuser_fields::Age>>::get_field(&user);
    let email_ref = <DeriveUser as HasField<deriveuser_fields::Email>>::get_field(&user);
    let is_active_ref = <DeriveUser as HasField<deriveuser_fields::IsActive>>::get_field(&user);
    let score_ref = <DeriveUser as HasField<deriveuser_fields::Score>>::get_field(&user);

    // Verify values
    assert_eq!(name_ref, "Jane Doe");
    assert_eq!(*age_ref, 28);
    assert_eq!(email_ref, "jane.doe@example.com");
    assert!(*is_active_ref);
    assert_eq!(*score_ref, 95.5);

    // Test field name generation
    assert_eq!(deriveuser_fields::Name::field_name(), "name");
    assert_eq!(deriveuser_fields::Age::field_name(), "age");
    assert_eq!(deriveuser_fields::Email::field_name(), "email");
    assert_eq!(deriveuser_fields::IsActive::field_name(), "is_active");
    assert_eq!(deriveuser_fields::Score::field_name(), "score");
}
