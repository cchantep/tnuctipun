/// # Field Witnesses
///
/// This module provides type-level field references for Rust structs, enabling compile-time
/// verification of field existence and type correctness. Field witnesses are particularly
/// useful when working with external systems like MongoDB, where field names are often
/// represented as strings.
///
/// ## Using the Derive Macro (Recommended)
///
/// The simplest way to generate field witnesses is to use the derive macro:
///
/// ```rust
/// use tnuctipun::FieldWitnesses;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
/// struct User {
///     Name: String,
///     Age: i32,
/// }
/// ```
///
/// The derive macro automatically inspects your struct and generates all necessary
/// field witnesses without requiring you to list the fields and types again.
///
/// ## Compile-Time Guarantees
///
/// Using field witnesses provides the following guarantees at compile time:
///
/// 1. The specified field exists in the struct
/// 2. The field has the expected type
/// 3. Operations on the field use compatible value types
///
/// This eliminates runtime errors caused by typos in field names or type mismatches.
/// Trait to get field value from struct
pub trait HasField<F: FieldName> {
    /// The value type of the field
    type Value;

    /// Get a reference to the field value
    fn get_field(&self) -> &Self::Value;
}

/// Examples of compile-time type safety:
///
/// ```compile_fail
/// use tnuctipun::field_witnesses::HasField;
///
/// // Define a simple struct with an i32 age field
/// struct User { age: i32 }
///
/// // Define a field witness for age
/// struct age;
///
/// // Implement HasField
/// impl HasField<age> for User {
///     type Value = i32;
///     fn get_field(&self) -> &Self::Value { &self.age }
/// }
///
/// // This will fail to compile: age is i32, not String
/// let user = User { age: 42 };
/// let age_ref: &String = <User as HasField<age>>::get_field(&user);
/// ```
/// Trait to map field types to their string names
pub trait FieldName {
    /// Returns the string name of the field
    fn field_name() -> &'static str;
}

/// Evidence that a type is a non-empty struct.
/// It's implemented for structs that have at least one field and are using the `FieldWitnesses` derive macro.
///
/// ```rust
/// use tnuctipun::{FieldWitnesses, NonEmptyStruct};
///
/// #[derive(Debug, Clone, FieldWitnesses)]
/// struct User {
///     name: String,
///     age: i32,
/// }
/// /// impl NonEmptyStruct for User {}
/// ```
///
pub trait NonEmptyStruct {}

/// Example showing compile error when trying to access a nonexistent field:
///
/// ```compile_fail
/// use tnuctipun::{HasField, FieldName};
///
/// // Define a simple struct
/// struct User { name: String }
///
/// // Define field witness for existing field
/// struct name;
/// impl FieldName for name {
///     fn field_name() -> &'static str { "name" }
/// }
///
/// // Implement HasField for name
/// impl HasField<name> for User {
///     type Value = String;
///     fn get_field(&self) -> &Self::Value { &self.name }
/// }
///
/// // Define field witness for nonexistent field
/// struct age;
/// impl FieldName for age {
///     fn field_name() -> &'static str { "age" }
/// }
///
/// // This implementation will fail because User doesn't have an age field
/// impl HasField<age> for User {
///     type Value = i32;
///     fn get_field(&self) -> &Self::Value { &self.age }
/// }
/// ```
#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    // Test struct with the derive macro
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestUser {
        name: String,
        age: i32,
        email: String,
    }

    // Manual implementation of field witnesses for TestUser
    #[allow(non_camel_case_types)]
    pub struct test_name;
    impl FieldName for test_name {
        fn field_name() -> &'static str {
            "name"
        }
    }

    #[allow(non_camel_case_types)]
    pub struct test_age;
    impl FieldName for test_age {
        fn field_name() -> &'static str {
            "age"
        }
    }

    #[allow(non_camel_case_types)]
    pub struct test_email;
    impl FieldName for test_email {
        fn field_name() -> &'static str {
            "email"
        }
    }

    // Manual implementation of HasField for TestUser
    impl HasField<test_name> for TestUser {
        type Value = String;

        fn get_field(&self) -> &Self::Value {
            &self.name
        }
    }

    impl HasField<test_age> for TestUser {
        type Value = i32;

        fn get_field(&self) -> &Self::Value {
            &self.age
        }
    }

    impl HasField<test_email> for TestUser {
        type Value = String;

        fn get_field(&self) -> &Self::Value {
            &self.email
        }
    }

    #[test]
    fn test_field_name_trait() {
        // Test that the FieldName trait works correctly
        assert_eq!(test_name::field_name(), "name");
        assert_eq!(test_age::field_name(), "age");
        assert_eq!(test_email::field_name(), "email");
    }

    #[test]
    fn test_has_field_trait_manually_implemented() {
        // Create a test user with manual field witness implementation
        let user = TestUser {
            name: "John Doe".to_string(),
            age: 30,
            email: "john.doe@example.com".to_string(),
        };

        // Test using HasField trait with type parameters
        let name_ref: &String = HasField::<test_name>::get_field(&user);
        let age_ref: &i32 = HasField::<test_age>::get_field(&user);
        let email_ref: &String = HasField::<test_email>::get_field(&user);

        assert_eq!(name_ref, &"John Doe".to_string());
        assert_eq!(age_ref, &30);
        assert_eq!(email_ref, &"john.doe@example.com".to_string());
    }

    #[test]
    fn test_type_safety() {
        // This test verifies compile-time type safety
        // The following would cause compile errors if uncommented:

        // Error: age field is i32, not String
        // let _: HasField<test_age, Value = String> = ();

        // Error: nonexistent field
        // struct test_nonexistent;
        // impl FieldName for test_nonexistent {
        //     fn field_name() -> &'static str { "nonexistent" }
        // }
        // let _: HasField<test_nonexistent> = ();

        // These tests pass by not causing compile errors
        let user = TestUser {
            name: "Test".to_string(),
            age: 42,
            email: "test@example.com".to_string(),
        };

        // Test compile-time type safety for name field
        let name_ref: &String = <TestUser as HasField<test_name>>::get_field(&user);

        assert_eq!(name_ref, "Test");

        // Test compile-time type safety for age field
        let age_ref: &i32 = <TestUser as HasField<test_age>>::get_field(&user);

        assert_eq!(*age_ref, 42);

        // Test compile-time type safety for email field
        let email_ref: &String = <TestUser as HasField<test_email>>::get_field(&user);

        assert_eq!(email_ref, "test@example.com");
    }
}
