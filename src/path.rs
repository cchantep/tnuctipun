use crate::field_witnesses::{FieldName, HasField};

/// A type-safe path builder for navigating nested document structures in MongoDB queries.
///
/// The `Path` struct represents a navigation path through nested document fields,
/// maintaining type safety at compile time. It tracks the field path as a series
/// of field names and ensures that only valid field navigations are allowed
/// through Rust's type system.
///
/// # Type Parameters
///
/// * `F` - The field name marker type (e.g., `user_fields::Address`)
/// * `T` - The struct type that contains the field `F`
/// * `Root` - The root document type, preserved through field navigation for type safety
///
/// # Usage
///
/// `Path` is primarily used internally by the filter builder system to construct
/// MongoDB field paths like `"address.city"` or `"user.profile.name"` in a type-safe manner.
/// It ensures that field navigation operations are valid at compile time.
///
/// # Example
///
/// ```rust
/// use nessus::path::Path;
/// use nessus::FieldWitnesses;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
/// struct Address {
///     street: String,
///     city: String,
/// }
///
/// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
/// struct User {
///     name: String,
///     address: Address,
/// }
///
/// // Path is typically used internally by the filter system
/// let path = Path::<user_fields::Address, User, User>::new();
/// let city_path = path.field::<address_fields::City>();
/// // This creates a path that represents "address.city"
/// ```
pub struct Path<F: FieldName, T: HasField<F>, Root> {
    /// The accumulated field path segments (e.g., ["user", "profile"])
    pub(crate) prefix: Vec<String>,

    /// Phantom data to maintain type information at compile time
    pub(crate) _marker: std::marker::PhantomData<(F, T, Root)>,
}

impl<F: FieldName, T: HasField<F>, Root> Path<F, T, Root> {
    /// Creates a new path for the given field name and struct type.
    ///
    /// This creates an empty path with no prefix segments. The path can then
    /// be extended using the `field` method to navigate deeper into nested structures.
    ///
    /// # Returns
    ///
    /// A new `Path` instance with an empty prefix, ready for field navigation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::path::Path;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User {
    ///     name: String,
    ///     age: i32,
    /// }
    ///
    /// // Create a new path for the User struct's name field
    /// let path = Path::<user_fields::Name, User, User>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            prefix: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Navigates to a nested field within the current path context.
    ///
    /// This method extends the current path by adding the current field to the prefix
    /// and then targeting a nested field within that field's type. This enables
    /// type-safe navigation through nested document structures.
    ///
    /// # Type Parameters
    ///
    /// * `G` - The field name marker type for the target nested field
    ///
    /// # Arguments
    ///
    /// * `self` - The current path instance (consumed by reference)
    ///
    /// # Returns
    ///
    /// A new `Path` instance targeting the nested field, with the current field
    /// added to the prefix path.
    ///
    /// # Type Safety
    ///
    /// This method enforces that:
    /// - The target field `G` must exist in the current field's value type (`T::Value`)
    /// - The navigation is only allowed if `T::Value` implements `HasField<G>`
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::path::Path;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct Address {
    ///     street: String,
    ///     city: String,
    ///     zip_code: String,
    /// }
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User {
    ///     name: String,
    ///     home_address: Address,
    /// }
    ///
    /// // Create a path starting from User's home_address field
    /// let address_path = Path::<user_fields::HomeAddress, User, User>::new();
    ///
    /// // Navigate to the city field within the address
    /// let city_path = address_path.field::<address_fields::City>();
    ///
    /// // This creates a path representing "home_address.city"
    /// // The prefix will contain ["home_address"] and target the City field
    /// // The Root type (User) is preserved for type safety with builders
    /// ```
    ///
    /// # Compile-Time Safety
    ///
    /// ```compile_fail
    /// // This would fail to compile because NonExistentField doesn't exist in Address
    /// let invalid_path = address_path.field::<address_fields::NonExistentField>();
    /// ```
    pub fn field<G: FieldName>(&self) -> Path<G, T::Value, Root>
    where
        T::Value: HasField<G>,
    {
        let mut prefix = self.prefix.clone();

        prefix.push(F::field_name().to_string());

        Path {
            prefix,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<F: FieldName, T: HasField<F>, Root> Default for Path<F, T, Root> {
    /// Creates a default `Path` instance with an empty prefix.
    ///
    /// This is equivalent to calling `Path::new()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::path::Path;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User {
    ///     name: String,
    /// }
    ///
    /// let path: Path<user_fields::Name, User, User> = Default::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<F: FieldName, T: HasField<F>, Root> Clone for Path<F, T, Root> {
    /// Creates a clone of the path with the same prefix and type information.
    ///
    /// This is useful when you need to create multiple navigation paths
    /// starting from the same base path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use nessus::path::Path;
    /// use nessus::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct Address {
    ///     street: String,
    ///     city: String,
    /// }
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User {
    ///     home_address: Address,
    /// }
    ///
    /// let base_path = Path::<user_fields::HomeAddress, User, User>::new();
    /// let street_path = base_path.clone().field::<address_fields::Street>();
    /// let city_path = base_path.field::<address_fields::City>();
    /// ```
    fn clone(&self) -> Self {
        Self {
            prefix: self.prefix.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test field marker types for testing
    struct TestField;
    impl FieldName for TestField {
        fn field_name() -> &'static str {
            "test_field"
        }
    }

    struct NestedField;
    impl FieldName for NestedField {
        fn field_name() -> &'static str {
            "nested_field"
        }
    }

    struct DeepField;
    impl FieldName for DeepField {
        fn field_name() -> &'static str {
            "deep_field"
        }
    }

    // Test struct types
    struct DeepValue;
    impl HasField<DeepField> for DeepValue {
        type Value = String;

        fn get_field(&self) -> &Self::Value {
            unimplemented!("Test struct")
        }
    }

    struct TestValue;
    impl HasField<NestedField> for TestValue {
        type Value = DeepValue;

        fn get_field(&self) -> &Self::Value {
            unimplemented!("Test struct")
        }
    }

    struct TestStruct;
    impl HasField<TestField> for TestStruct {
        type Value = TestValue;

        fn get_field(&self) -> &Self::Value {
            unimplemented!("Test struct")
        }
    }

    #[test]
    fn test_path_new() {
        let path = Path::<TestField, TestStruct, TestStruct>::new();

        assert!(path.prefix.is_empty());
    }

    #[test]
    fn test_path_default() {
        let path: Path<TestField, TestStruct, TestStruct> = Default::default();

        assert!(path.prefix.is_empty());
    }

    #[test]
    fn test_path_clone() {
        let mut path = Path::<TestField, TestStruct, TestStruct>::new();

        path.prefix.push("existing".to_string());

        let cloned = path.clone();

        assert_eq!(path.prefix, cloned.prefix);
    }

    #[test]
    fn test_path_field_navigation() {
        let path = Path::<TestField, TestStruct, TestStruct>::new();
        let nested_path = path.field::<NestedField>();

        assert_eq!(nested_path.prefix, vec!["test_field"]);
    }

    #[test]
    fn test_path_deep_navigation() {
        let path = Path::<TestField, TestStruct, TestStruct>::new();
        let nested_path = path.field::<NestedField>();
        let deep_path: Path<DeepField, DeepValue, TestStruct> = nested_path.field::<DeepField>();

        assert_eq!(deep_path.prefix, vec!["test_field", "nested_field"]);
    }
}
