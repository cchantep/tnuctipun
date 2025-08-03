// This example demonstrates how to control private field inclusion in field witnesses

use nessus::{FieldName, FieldWitnesses, HasField, MongoComparable};
use serde::{Deserialize, Serialize};

// Example 1: Default behavior - private fields are skipped
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserDefault {
    pub name: String, // Public field - included
    email: String,    // Private field - skipped by default
    pub age: i32,     // Public field - included
    internal_id: u64, // Private field - skipped by default
}

// Example 2: Include private fields explicitly
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(include_private = true)]
struct UserWithPrivate {
    pub name: String, // Public field - included
    email: String,    // Private field - included
    pub age: i32,     // Public field - included
    internal_id: u64, // Private field - included
}

// Example 3: Combine with other attributes
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
#[nessus(field_naming = "camelCase", include_private = true)]
struct UserCombined {
    pub user_name: String,  // Public field - included, named "userName"
    email_address: String,  // Private field - included, named "emailAddress"
    pub created_at: String, // Public field - included, named "createdAt"
    internal_data: String,  // Private field - included, named "internalData"
}

fn main() {
    // Example 1: Default behavior
    let user_default = UserDefault {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        age: 30,
        internal_id: 12345,
    };

    println!("=== Default Behavior (private fields skipped) ===");
    println!("Available fields:");
    println!("- name: {}", userdefault_fields::Name::field_name());
    println!("- age: {}", userdefault_fields::Age::field_name());

    // Access field values
    let name = <UserDefault as HasField<userdefault_fields::Name>>::get_field(&user_default);

    let age = <UserDefault as HasField<userdefault_fields::Age>>::get_field(&user_default);

    println!("Values: name='{name}', age={age}");

    // Example 2: Include private fields
    let user_with_private = UserWithPrivate {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        age: 25,
        internal_id: 67890,
    };

    println!("\n=== With Private Fields Included ===");
    println!("Available fields:");
    println!("- name: {}", userwithprivate_fields::Name::field_name());
    println!("- email: {}", userwithprivate_fields::Email::field_name());
    println!("- age: {}", userwithprivate_fields::Age::field_name());
    println!(
        "- internal_id: {}",
        userwithprivate_fields::InternalId::field_name()
    );

    // Access all field values including private ones
    let name =
        <UserWithPrivate as HasField<userwithprivate_fields::Name>>::get_field(&user_with_private);

    let email =
        <UserWithPrivate as HasField<userwithprivate_fields::Email>>::get_field(&user_with_private);

    let age =
        <UserWithPrivate as HasField<userwithprivate_fields::Age>>::get_field(&user_with_private);

    let internal_id = <UserWithPrivate as HasField<userwithprivate_fields::InternalId>>::get_field(
        &user_with_private,
    );

    println!("Values: name='{name}', email='{email}', age={age}, internal_id={internal_id}");

    // Example 3: Combined attributes
    let user_combined = UserCombined {
        user_name: "Charlie".to_string(),
        email_address: "charlie@example.com".to_string(),
        created_at: "2023-01-01".to_string(),
        internal_data: "secret".to_string(),
    };

    println!("\n=== Combined Attributes (camelCase + private fields) ===");
    println!("Available fields:");
    println!(
        "- user_name: {}",
        usercombined_fields::UserName::field_name()
    );
    println!(
        "- email_address: {}",
        usercombined_fields::EmailAddress::field_name()
    );
    println!(
        "- created_at: {}",
        usercombined_fields::CreatedAt::field_name()
    );
    println!(
        "- internal_data: {}",
        usercombined_fields::InternalData::field_name()
    );

    // Access field values with transformed names
    let user_name =
        <UserCombined as HasField<usercombined_fields::UserName>>::get_field(&user_combined);

    let email_address =
        <UserCombined as HasField<usercombined_fields::EmailAddress>>::get_field(&user_combined);

    let created_at =
        <UserCombined as HasField<usercombined_fields::CreatedAt>>::get_field(&user_combined);

    let internal_data =
        <UserCombined as HasField<usercombined_fields::InternalData>>::get_field(&user_combined);

    println!(
        "Values: user_name='{user_name}', email_address='{email_address}', created_at='{created_at}', internal_data='{internal_data}'"
    );

    println!("\n=== Summary ===");
    println!("1. Default: #[derive(FieldWitnesses)] - skips private fields");
    println!("2. Include private: #[nessus(include_private = true)] - includes all fields");
    println!("3. Can combine with other attributes like field_naming");
}
