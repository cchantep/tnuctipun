use nessus::{FieldName, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

// Test struct with mixed visibility and include_private = true
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = true)]
struct UserWithPrivate {
    pub name: String, // public field
    email: String,    // private field - should be included
    pub age: i32,     // public field
    internal_id: u64, // private field - should be included
}

// Test struct with mixed visibility and include_private = false (default)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserWithoutPrivate {
    pub name: String, // public field
    email: String,    // private field - should be skipped
    pub age: i32,     // public field
    internal_id: u64, // private field - should be skipped
}

// Test struct with all private fields and include_private = true
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = true)]
struct AllPrivateIncluded {
    name: String,
    age: i32,
    email: String,
}

// Test struct with all private fields and include_private = false (default)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct AllPrivateSkipped {
    name: String,
    age: i32,
    email: String,
}

// Test struct with all public fields (include_private should not matter)
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = false)]
struct AllPublic {
    pub name: String,
    pub age: i32,
    pub email: String,
}

#[test]
fn test_include_private_true() {
    let user = UserWithPrivate {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
        internal_id: 12345,
    };

    // All fields should be accessible (both public and private)
    assert_eq!(userwithprivate_fields::Name::field_name(), "name");
    assert_eq!(userwithprivate_fields::Email::field_name(), "email");
    assert_eq!(userwithprivate_fields::Age::field_name(), "age");
    assert_eq!(
        userwithprivate_fields::InternalId::field_name(),
        "internal_id"
    );

    // Test field access using HasField trait
    use nessus::HasField;

    let name_ref = <UserWithPrivate as HasField<userwithprivate_fields::Name>>::get_field(&user);

    let email_ref = <UserWithPrivate as HasField<userwithprivate_fields::Email>>::get_field(&user);

    let age_ref = <UserWithPrivate as HasField<userwithprivate_fields::Age>>::get_field(&user);

    let internal_id_ref =
        <UserWithPrivate as HasField<userwithprivate_fields::InternalId>>::get_field(&user);

    assert_eq!(name_ref, "Alice");
    assert_eq!(email_ref, "alice@example.com");
    assert_eq!(*age_ref, 30);
    assert_eq!(*internal_id_ref, 12345);
}

#[test]
fn test_include_private_false_default() {
    let user = UserWithoutPrivate {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        age: 25,
        internal_id: 67890,
    };

    // Only public fields should be accessible
    assert_eq!(userwithoutprivate_fields::Name::field_name(), "name");
    assert_eq!(userwithoutprivate_fields::Age::field_name(), "age");

    // Test field access using HasField trait
    use nessus::HasField;

    let name_ref =
        <UserWithoutPrivate as HasField<userwithoutprivate_fields::Name>>::get_field(&user);

    let age_ref =
        <UserWithoutPrivate as HasField<userwithoutprivate_fields::Age>>::get_field(&user);

    assert_eq!(name_ref, "Bob");
    assert_eq!(*age_ref, 25);

    // These should not compile if uncommented (private fields should be skipped):
    // assert_eq!(userwithoutprivate_fields::Email::field_name(), "email");
    // assert_eq!(userwithoutprivate_fields::InternalId::field_name(), "internal_id");
}

#[test]
fn test_all_private_fields_included() {
    let user = AllPrivateIncluded {
        name: "Charlie".to_string(),
        age: 35,
        email: "charlie@example.com".to_string(),
    };

    // All private fields should be accessible when include_private = true
    assert_eq!(allprivateincluded_fields::Name::field_name(), "name");
    assert_eq!(allprivateincluded_fields::Age::field_name(), "age");
    assert_eq!(allprivateincluded_fields::Email::field_name(), "email");

    // Test field access using HasField trait
    use nessus::HasField;

    let name_ref =
        <AllPrivateIncluded as HasField<allprivateincluded_fields::Name>>::get_field(&user);

    let age_ref =
        <AllPrivateIncluded as HasField<allprivateincluded_fields::Age>>::get_field(&user);

    let email_ref =
        <AllPrivateIncluded as HasField<allprivateincluded_fields::Email>>::get_field(&user);

    assert_eq!(name_ref, "Charlie");
    assert_eq!(*age_ref, 35);
    assert_eq!(email_ref, "charlie@example.com");
}

#[test]
fn test_all_public_fields() {
    let user = AllPublic {
        name: "David".to_string(),
        age: 40,
        email: "david@example.com".to_string(),
    };

    // All public fields should be accessible regardless of include_private setting
    assert_eq!(allpublic_fields::Name::field_name(), "name");
    assert_eq!(allpublic_fields::Age::field_name(), "age");
    assert_eq!(allpublic_fields::Email::field_name(), "email");

    // Test field access using HasField trait
    use nessus::HasField;

    let name_ref = <AllPublic as HasField<allpublic_fields::Name>>::get_field(&user);
    let age_ref = <AllPublic as HasField<allpublic_fields::Age>>::get_field(&user);
    let email_ref = <AllPublic as HasField<allpublic_fields::Email>>::get_field(&user);

    assert_eq!(name_ref, "David");
    assert_eq!(*age_ref, 40);
    assert_eq!(email_ref, "david@example.com");
}

#[test]
fn test_combine_with_other_attributes() {
    // Test that include_private works with other attributes like field_naming
    #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    #[nessus(field_naming = "camelCase", include_private = true)]
    struct CombinedAttributesTest {
        pub user_name: String, // public field
        email_address: String, // private field - should be included with camelCase naming
    }

    let user = CombinedAttributesTest {
        user_name: "Eve".to_string(),
        email_address: "eve@example.com".to_string(),
    };

    // Both fields should be accessible with camelCase naming
    assert_eq!(
        combinedattributestest_fields::UserName::field_name(),
        "userName"
    );
    assert_eq!(
        combinedattributestest_fields::EmailAddress::field_name(),
        "emailAddress"
    );

    // Test field access using HasField trait
    use nessus::HasField;

    let name_ref =
        <CombinedAttributesTest as HasField<combinedattributestest_fields::UserName>>::get_field(
            &user,
        );

    let email_ref = <CombinedAttributesTest as HasField<
        combinedattributestest_fields::EmailAddress,
    >>::get_field(&user);

    assert_eq!(name_ref, "Eve");
    assert_eq!(email_ref, "eve@example.com");
}
