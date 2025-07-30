use proc_macro::TokenStream;

mod field_witnesses;
mod mongo_comparable;

/// Procedural macro to generate field witnesses for a struct.
///
/// This macro automatically generates:
/// - Struct marker types for each field (always in PascalCase following Rust conventions)
/// - FieldName implementations for each field marker (returns MongoDB field names)
/// - HasField implementations to access field values with type safety
///
/// Note: The generated field witnesses are scoped within a module named `<struct_name>_fields`
/// at the same module level as the derived struct. This prevents naming conflicts when multiple
/// structs have fields with the same names, even across different modules.
///
/// # Important: Naming Behavior
///
/// The field naming strategy only affects MongoDB field names returned by `FieldName::field_name()`.
/// Struct marker names are always converted to PascalCase regardless of the naming strategy.
///
/// # Attributes
///
/// ## Container-level attributes
///
/// - `#[nessus(field_naming = "strategy")]` - Apply a naming strategy to MongoDB field names only
///   - Built-in strategies: "PascalCase", "camelCase"
///
/// ## Field-level attributes
///
/// - `#[nessus(rename = "name")]` - Override the MongoDB field name for this specific field
/// - `#[nessus(skip)]` - Skip generating witnesses for this field
///
/// # Examples
///
/// ## Basic usage (default behavior):
/// ```ignore
/// use nessus_derive::FieldWitnesses;
///
/// #[derive(FieldWitnesses)]
/// struct User {
///     user_name: String,
///     email_address: String,
/// }
///
/// // Generated struct markers (always PascalCase):
/// // - user_fields::UserName
/// // - user_fields::EmailAddress
///
/// // MongoDB field names (kept as-is by default):
/// // - UserName::field_name() returns "user_name"
/// // - EmailAddress::field_name() returns "email_address"
/// ```
///
/// ## With field naming strategy:
/// ```ignore
/// #[derive(FieldWitnesses)]
/// #[nessus(field_naming = "camelCase")]
/// struct User {
///     user_name: String,
///     email_address: String,
/// }
///
/// // Generated struct markers (unchanged - always PascalCase):
/// // - user_fields::UserName
/// // - user_fields::EmailAddress
///
/// // MongoDB field names (transformed by strategy):
/// // - UserName::field_name() returns "userName"
/// // - EmailAddress::field_name() returns "emailAddress"
/// ```
///
/// With field-level overrides:
///
/// ```ignore
/// #[derive(FieldWitnesses)]
/// #[nessus(field_naming = "camelCase")]
/// struct User {
///     user_name: String,              // -> "userName"
///     #[nessus(rename = "email")]
///     email_address: String,          // -> "email" (override)
///     #[nessus(skip)]
///     internal_id: String,            // Skipped entirely
/// }
/// ```
///
/// Multiple structs with same field names don't conflict:
///
/// ```ignore
/// mod admin {
///     #[derive(FieldWitnesses)]
///     struct User {
///         name: String,
///         permissions: Vec<String>,
///     }
///     // Generates: admin::user_fields::Name (different from the above)
/// }
/// ```
///
/// The macro generates a module containing field witness types for type-safe field access.
/// For a struct named `User`, the generated module will be `user_fields` containing
/// field witness types for each field in the struct.
#[proc_macro_derive(FieldWitnesses, attributes(nessus))]
pub fn derive_field_witnesses(input: TokenStream) -> TokenStream {
    field_witnesses::derive_field_witnesses(input)
}

/// Procedural macro to generate MongoComparable implementations for a struct.
///
/// This macro automatically generates implementations of the MongoComparable trait
/// for each field in the struct, enabling MongoDB query operations.
///
/// Note: This macro requires FieldWitnesses to also be derived on the same struct
/// to generate the necessary field markers and trait implementations.
///
/// Example:
/// ```ignore
/// use nessus_derive::{FieldWitnesses, MongoComparable};
///
/// #[derive(FieldWitnesses, MongoComparable)]
/// struct User {
///     name: String,
///     age: i32,
///     tags: Vec<String>
/// }
///
/// // The macro generates MongoComparable implementations like:
/// // impl MongoComparable<String, String> for User {}
/// // impl MongoComparable<i32, i32> for User {}  
/// // impl MongoComparable<Vec<String>, String> for User {}
/// // And many other compatible type combinations...
/// ```
///
/// The macro generates trait implementations that enable type-safe MongoDB operations
/// by providing evidence that specific field types can be compared with specific value types.
#[proc_macro_derive(MongoComparable)]
pub fn derive_mongo_comparable(input: TokenStream) -> TokenStream {
    mongo_comparable::derive_mongo_comparable(input)
}
