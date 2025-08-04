use crate::field_witnesses::{FieldName, HasField};
/// Macro to generate field witnesses (type-level field representations) for a struct
///
/// This macro automatically generates:
/// - Field marker structs for each field
/// - FieldName implementations for each field marker
/// - HasField implementations to access field values with type safety
///
/// # Examples
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use tnuctipun::DeclarativeFieldWitnesses;
/// use tnuctipun::mongo::type_safe_eq;
/// 
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct User {
///     name: String,
///     age: i32,
///     email: String,
/// }
///
/// // Generate field witnesses for the User struct
/// DeclarativeFieldWitnesses!(User, name: String, age: i32, email: String);
///
/// // Create type-safe MongoDB queries
/// let name_filter = type_safe_eq::<name, User, _>("John".to_string());
/// let age_filter = type_safe_eq::<age, User, _>(30);
/// 
/// // Filter guarantees at compile time:
/// // 1. Field 'name' exists in struct User
/// // 2. Field 'name' is of type String
/// // 3. The value "John" is compatible with String
/// ```
#[macro_export]
macro_rules! DeclarativeFieldWitnesses {
    ($struct_name:ident, $($field_name:ident: $field_type:ty),* $(,)?) => {
        // Generate field marker structs
        $(
            #[doc = concat!("Field witness for the `", stringify!($field_name), "` field of `", stringify!($struct_name), "`")]
            #[derive(Debug, Clone)]
            pub struct $field_name;

            // Implement FieldName for each field marker
            impl $crate::field_witnesses::FieldName for $field_name {
                fn field_name() -> &'static str {
                    stringify!($field_name)
                }
            }

            // Implement HasField for each field
            impl $crate::field_witnesses::HasField<$field_name> for $struct_name {
                type Output = $field_type;
                
                fn get_field(&self) -> &Self::Output {
                    &self.$field_name
                }
            }
        )*
    };
}
