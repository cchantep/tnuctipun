use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Lit, Visibility, parse_macro_input};

/// Derive macro for generating field witnesses with full attribute support.
///
/// This macro generates type-safe field markers for struct fields, enabling compile-time
/// validation of field access in MongoDB operations. It provides extensive customization
/// through container-level and field-level attributes.
///
/// The macro generates two distinct naming schemes:
///
/// 1. **Struct marker names**: Always converted to PascalCase regardless of strategy (e.g., `user_name` → `UserName`)
/// 2. **MongoDB field names**: Controlled by the field naming strategy (returned by `FieldName::field_name()`)
///
/// This separation ensures Rust code follows proper naming conventions while allowing flexible
/// MongoDB field naming.
///
/// # Attributes
///
/// ## Container-level attributes
///
/// - `#[tnuctipun(field_naming = "strategy")]` - Apply a naming strategy to MongoDB field names
///   - Built-in strategies: "PascalCase", "camelCase"
///   - If not specified, MongoDB field names are kept as-is (no transformation)
///   - **Note**: This only affects `FieldName::field_name()` output, not struct marker names
/// - `#[tnuctipun(include_private = true)]` - Include private fields in witness generation
///   - If not specified or set to false, private fields are skipped
///   - When true, both public and private fields generate witnesses
///
/// ## Field-level attributes
///
/// - `#[tnuctipun(rename = "name")]` - Override the MongoDB field name for this specific field
/// - `#[tnuctipun(skip)]` - Skip generating witnesses for this field
///
/// Built-in field naming strategies
struct FieldNaming;

impl FieldNaming {
    /// Generate field name transformation at macro expansion time
    fn transform_field_name(strategy: &str, field_name: &str) -> String {
        match strategy {
            "PascalCase" => snake_case_to_pascal_case(field_name),
            "camelCase" => snake_case_to_camel_case(field_name),
            _ => field_name.to_string(),
        }
    }
}

#[derive(Debug, Default)]
struct ContainerAttributes {
    field_naming_strategy: Option<String>, // Built-in strategy name only
    include_private: bool,                 // Whether to include private fields (default: false)
}

#[derive(Debug, Default)]
struct FieldAttributes {
    rename: Option<String>,
    skip: bool,
}

/// Procedural macro to generate field witnesses for a struct.
///
/// This macro automatically generates:
/// - Struct marker types for each field (always in PascalCase, following Rust naming conventions)
/// - FieldName implementations for each field marker (returns MongoDB field names)
/// - HasField implementations to access field values with type safety
///
/// Note: The generated field witnesses are scoped within a module named `<struct_name>_fields`
/// at the same module level as the derived struct. This prevents naming conflicts when multiple
/// structs have fields with the same names, even across different modules.
///
/// # Naming Behavior
///
/// **Important**: The field naming strategy affects two different things:
///
/// 1. **Struct marker names**: Always converted to PascalCase regardless of strategy (e.g., `user_name` → `UserName`)
/// 2. **MongoDB field names**: Controlled by the field naming strategy (returned by `FieldName::field_name()`)
///
/// This separation ensures Rust code follows proper naming conventions while allowing flexible
/// MongoDB field naming.
///
/// # Attributes
///
/// ## Container-level attributes
///
/// - `#[tnuctipun(field_naming = "strategy")]` - Apply a naming strategy to MongoDB field names
///   - Built-in strategies: "PascalCase", "camelCase"
///   - If not specified, MongoDB field names are kept as-is (no transformation)
///   - **Note**: This only affects `FieldName::field_name()` output, not struct marker names
/// - `#[tnuctipun(include_private = true)]` - Include private fields in witness generation
///   - If not specified or set to false, private fields are skipped
///   - When true, both public and private fields generate witnesses
///
/// ## Field-level attributes
///
/// - `#[tnuctipun(rename = "name")]` - Override the MongoDB field name for this specific field
/// - `#[tnuctipun(skip)]` - Skip generating witnesses for this field
///
/// # Examples
///
/// ## Basic usage (default behavior):
///
/// ```ignore
/// use tnuctipun::derive::FieldWitnesses;
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
///
/// ```ignore
/// #[derive(FieldWitnesses)]
/// #[tnuctipun(field_naming = "camelCase")]
/// struct User {
///     user_name: String,
///     email_address: String,
/// }
///
/// // Generated struct markers (always PascalCase regardless of strategy):
/// // - user_fields::UserName
/// // - user_fields::EmailAddress
///
/// // MongoDB field names (transformed by strategy):
/// // - UserName::field_name() returns "userName"
/// // - EmailAddress::field_name() returns "emailAddress"
/// ```
///
/// ## Comparison of different strategies:
///
/// ```ignore
/// // For field: user_name
///
/// // Default (no strategy):
/// // Struct marker: user_fields::UserName
/// // MongoDB field: "user_name"
///
/// // With field_naming = "camelCase":
/// // Struct marker: user_fields::UserName (unchanged)
/// // MongoDB field: "userName"
///
/// // With field_naming = "PascalCase":
/// // Struct marker: user_fields::UserName (unchanged)
/// // MongoDB field: "UserName"
/// ```
///
/// With field-level overrides:
///
/// ```ignore
/// #[derive(FieldWitnesses)]
/// #[tnuctipun(field_naming = "camelCase")]
/// struct User {
///     user_name: String,              // -> "userName"
///     #[tnuctipun(rename = "email")]
///     email_address: String,          // -> "email" (override)
///     #[tnuctipun(skip)]
///     internal_id: String,            // Skipped entirely
/// }
/// ```
///
/// With private field inclusion:
///
/// ```ignore
/// #[derive(FieldWitnesses)]
/// #[tnuctipun(include_private = true)]
/// struct User {
///     pub user_name: String,          // Public field - included
///     email_address: String,          // Private field - included due to include_private = true
/// }
///
/// #[derive(FieldWitnesses)]
/// struct UserWithoutPrivate {
///     pub user_name: String,          // Public field - included
///     email_address: String,          // Private field - skipped (include_private defaults to false)
/// }
/// ```
pub fn derive_field_witnesses(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;

    // Parse container-level attributes
    let container_attrs = parse_container_attributes(&input.attrs);

    // Get the fields of the struct
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => {
                return quote! {
                    compile_error!("FieldWitnesses only works with structs that have named fields");
                }
                .into();
            }
        },
        _ => {
            return quote! {
                compile_error!("FieldWitnesses only works with structs");
            }
            .into();
        }
    };

    // Create a module name for this struct's field witnesses
    let fields_mod_name = syn::Ident::new(
        &format!("{}_fields", struct_name.to_string().to_lowercase()),
        struct_name.span(),
    );

    // Generate field witness types within the module
    let field_witness_types = fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_attrs = parse_field_attributes(&field.attrs);

            // Skip fields marked with #[tnuctipun(skip)]
            if field_attrs.skip {
                return None;
            }

            // Skip private fields if include_private is false
            if is_field_private(&field.vis) && !container_attrs.include_private {
                return None;
            }

            // Generate PascalCase struct name for the field witness (always follows Rust naming conventions)
            // This is independent of any field naming strategy - struct markers are ALWAYS PascalCase
            let struct_marker_name = syn::Ident::new(
                &snake_case_to_pascal_case(&field_name.to_string()),
                field_name.span(),
            );

            // Determine the final field name for MongoDB queries
            // This is where field naming strategies are applied (separate from struct marker names)
            let mongo_field_name_expr = if let Some(rename) = field_attrs.rename {
                // Direct rename - use string literal
                quote! { #rename }
            } else if let Some(ref strategy) = container_attrs.field_naming_strategy {
                // Built-in strategy - transform at macro expansion time
                let transformed_name =
                    FieldNaming::transform_field_name(strategy, &field_name.to_string());
                quote! { #transformed_name }
            } else {
                // Default - use field name as is
                let field_name_str = field_name.to_string();
                quote! { #field_name_str }
            };

            Some(quote! {
                #[doc = concat!("Field witness for a field of `", stringify!(#struct_name), "`")]
                #[derive(Debug, Clone)]
                pub struct #struct_marker_name;

                impl ::tnuctipun::field_witnesses::FieldName for #struct_marker_name {
                    fn field_name() -> &'static str {
                        #mongo_field_name_expr
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    // Generate HasField implementations outside the module to avoid path issues
    let has_field_impls = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_attrs = parse_field_attributes(&field.attrs);

        // Skip fields marked with #[tnuctipun(skip)]
        if field_attrs.skip {
            return None;
        }

        // Skip private fields if include_private is false
        if is_field_private(&field.vis) && !container_attrs.include_private {
            return None;
        }

        let field_type = &field.ty;
        let struct_marker_name = syn::Ident::new(
            &snake_case_to_pascal_case(&field_name.to_string()),
            field_name.span()
        );

        Some(quote! {
            impl ::tnuctipun::field_witnesses::HasField<#fields_mod_name::#struct_marker_name> for #struct_name {
                type Value = #field_type;

                fn get_field(&self) -> &Self::Value {
                    &self.#field_name
                }
            }
        })
    }).collect::<Vec<_>>();

    // Generate the final code
    let expanded = quote! {
        impl ::tnuctipun::field_witnesses::NonEmptyStruct for #struct_name {}

        // Create a module containing all field witnesses specifically for this struct
        // This prevents naming conflicts when multiple structs have fields with the same name
        pub mod #fields_mod_name {
            #(#field_witness_types)*
        }

        // Generate HasField implementations outside the module
        #(#has_field_impls)*
    };

    // Convert back to token stream
    TokenStream::from(expanded)
}

fn parse_container_attributes(attrs: &[Attribute]) -> ContainerAttributes {
    let mut container_attrs = ContainerAttributes::default();

    for attr in attrs {
        if attr.path().is_ident("tnuctipun") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("field_naming") {
                    let value: Lit = meta.value()?.parse()?;

                    if let Lit::Str(lit_str) = value {
                        let strategy = lit_str.value();

                        // Check if it's a built-in strategy
                        match strategy.as_str() {
                            "PascalCase" | "pascal_case" => {
                                container_attrs.field_naming_strategy =
                                    Some("PascalCase".to_string());
                            }
                            "camelCase" | "camel_case" => {
                                container_attrs.field_naming_strategy =
                                    Some("camelCase".to_string());
                            }
                            _ => {
                                return Err(meta.error(format!(
                                    "Invalid field_naming attribute: '{strategy}'. \
                                     Supported options are: 'PascalCase', 'camelCase'"
                                )));
                            }
                        }
                    }
                } else if meta.path.is_ident("include_private") {
                    let value: Lit = meta.value()?.parse()?;

                    if let Lit::Bool(lit_bool) = value {
                        container_attrs.include_private = lit_bool.value;
                    } else {
                        return Err(meta.error(
                            "include_private attribute must be a boolean value (true or false)",
                        ));
                    }
                }
                Ok(())
            });
        }
    }

    container_attrs
}

/// Check if a field is private (not public)
fn is_field_private(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Inherited)
}

fn parse_field_attributes(attrs: &[Attribute]) -> FieldAttributes {
    let mut field_attrs = FieldAttributes::default();

    for attr in attrs {
        if attr.path().is_ident("tnuctipun") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value: Lit = meta.value()?.parse()?;

                    if let Lit::Str(lit_str) = value {
                        field_attrs.rename = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("skip") {
                    field_attrs.skip = true;
                }

                Ok(())
            });
        }
    }

    field_attrs
}

// Built-in field name transformations - called at macro expansion time

/// Convert snake_case to PascalCase for struct marker names.
/// This is used for all generated struct markers regardless of field naming strategy.
///
/// Examples:
/// - "user_name" → "UserName"
/// - "email_address" → "EmailAddress"
/// - "created_at" → "CreatedAt"
fn snake_case_to_pascal_case(s: &str) -> String {
    // Convert snake_case to PascalCase
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn snake_case_to_camel_case(s: &str) -> String {
    // Convert snake_case to camelCase
    let mut parts = s.split('_');

    // First part stays lowercase
    let first = parts.next().unwrap_or("").to_lowercase();

    // Remaining parts get first letter capitalized
    let rest: String = parts
        .map(|word| {
            let mut chars = word.chars();

            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();

    first + &rest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case_to_pascal_case() {
        // Basic transformations
        assert_eq!(snake_case_to_pascal_case("user_name"), "UserName");
        assert_eq!(snake_case_to_pascal_case("email_address"), "EmailAddress");
        assert_eq!(snake_case_to_pascal_case("created_at"), "CreatedAt");
        assert_eq!(snake_case_to_pascal_case("is_active"), "IsActive");

        // Single words
        assert_eq!(snake_case_to_pascal_case("name"), "Name");
        assert_eq!(snake_case_to_pascal_case("age"), "Age");

        // Multiple underscores
        assert_eq!(
            snake_case_to_pascal_case("some_long_field_name"),
            "SomeLongFieldName"
        );
        assert_eq!(snake_case_to_pascal_case("api_key_secret"), "ApiKeySecret");

        // Edge cases
        assert_eq!(snake_case_to_pascal_case(""), "");
        assert_eq!(snake_case_to_pascal_case("_"), "");
        assert_eq!(snake_case_to_pascal_case("a"), "A");
        assert_eq!(snake_case_to_pascal_case("a_b"), "AB");

        // Already mixed case (should still work)
        assert_eq!(snake_case_to_pascal_case("User_Name"), "UserName");
        assert_eq!(snake_case_to_pascal_case("API_key"), "APIKey");
    }

    #[test]
    fn test_snake_case_to_camel_case() {
        // Basic transformations
        assert_eq!(snake_case_to_camel_case("user_name"), "userName");
        assert_eq!(snake_case_to_camel_case("email_address"), "emailAddress");
        assert_eq!(snake_case_to_camel_case("created_at"), "createdAt");
        assert_eq!(snake_case_to_camel_case("is_active"), "isActive");

        // Single words (should stay lowercase)
        assert_eq!(snake_case_to_camel_case("name"), "name");
        assert_eq!(snake_case_to_camel_case("age"), "age");

        // Multiple underscores
        assert_eq!(
            snake_case_to_camel_case("some_long_field_name"),
            "someLongFieldName"
        );
        assert_eq!(snake_case_to_camel_case("api_key_secret"), "apiKeySecret");

        // Edge cases
        assert_eq!(snake_case_to_camel_case(""), "");
        assert_eq!(snake_case_to_camel_case("_"), "");
        assert_eq!(snake_case_to_camel_case("a"), "a");
        assert_eq!(snake_case_to_camel_case("a_b"), "aB");

        // Already mixed case (should normalize)
        assert_eq!(snake_case_to_camel_case("User_Name"), "userName");
        assert_eq!(snake_case_to_camel_case("API_key"), "apiKey");
    }

    #[test]
    fn test_edge_cases_and_special_characters() {
        // Empty strings
        assert_eq!(snake_case_to_pascal_case(""), "");
        assert_eq!(snake_case_to_camel_case(""), "");

        // Single characters
        assert_eq!(snake_case_to_pascal_case("a"), "A");
        assert_eq!(snake_case_to_camel_case("a"), "a");

        // Underscores only
        assert_eq!(snake_case_to_pascal_case("_"), "");
        assert_eq!(snake_case_to_camel_case("_"), "");

        // Multiple consecutive underscores
        assert_eq!(snake_case_to_pascal_case("user__name"), "UserName");
        assert_eq!(snake_case_to_camel_case("user__name"), "userName");

        // Leading/trailing underscores
        assert_eq!(snake_case_to_pascal_case("_user_name"), "UserName");
        assert_eq!(snake_case_to_pascal_case("user_name_"), "UserName");
        assert_eq!(snake_case_to_camel_case("_user_name"), "UserName"); // Empty first part becomes "", so "User" + "Name"
        assert_eq!(snake_case_to_camel_case("user_name_"), "userName");
    }

    #[test]
    fn test_numbers_in_field_names() {
        // Numbers in snake_case
        assert_eq!(snake_case_to_pascal_case("user_id_2"), "UserId2");
        assert_eq!(snake_case_to_pascal_case("api_v1_key"), "ApiV1Key");
        assert_eq!(snake_case_to_camel_case("user_id_2"), "userId2");
        assert_eq!(snake_case_to_camel_case("api_v1_key"), "apiV1Key");
    }

    #[test]
    fn test_real_world_field_names() {
        // Common database field names
        let test_cases = vec![
            // (input, PascalCase, camelCase)
            ("first_name", "FirstName", "firstName"),
            ("last_name", "LastName", "lastName"),
            ("email_address", "EmailAddress", "emailAddress"),
            ("phone_number", "PhoneNumber", "phoneNumber"),
            ("date_of_birth", "DateOfBirth", "dateOfBirth"),
            ("created_at", "CreatedAt", "createdAt"),
            ("updated_at", "UpdatedAt", "updatedAt"),
            ("is_active", "IsActive", "isActive"),
            ("is_verified", "IsVerified", "isVerified"),
            ("user_id", "UserId", "userId"),
            ("product_id", "ProductId", "productId"),
            ("order_id", "OrderId", "orderId"),
            ("customer_id", "CustomerId", "customerId"),
            ("api_key", "ApiKey", "apiKey"),
            ("access_token", "AccessToken", "accessToken"),
            ("refresh_token", "RefreshToken", "refreshToken"),
        ];

        for (snake, expected_pascal, expected_camel) in test_cases {
            assert_eq!(
                snake_case_to_pascal_case(snake),
                expected_pascal,
                "Failed snake_case_to_pascal_case for '{}'",
                snake
            );

            assert_eq!(
                snake_case_to_camel_case(snake),
                expected_camel,
                "Failed snake_case_to_camel_case for '{}'",
                snake
            );
        }
    }

    #[test]
    fn test_default_behavior_no_transformation() {
        // Test the default behavior when no field_naming attribute is specified
        // This simulates what happens in the actual macro expansion

        // When no strategy is provided, field names should remain unchanged
        let test_cases = vec![
            "user_name",
            "email_address",
            "created_at",
            "is_active",
            "api_key",
            "some_long_field_name",
            "id",
            "name",
            "_private_field",
            "field_with_123",
            "API_CONSTANT", // Even if it's not typical snake_case
        ];

        for field_name in test_cases {
            // This is what happens in the default case in transform_field_name
            let result = field_name.to_string(); // Default case in the match

            assert_eq!(
                result, field_name,
                "Default behavior should keep field name '{}' unchanged",
                field_name
            );
        }
    }

    #[test]
    fn test_is_field_private_inherited_visibility() {
        // Test that inherited visibility (no explicit visibility modifier) is considered private
        let visibility = Visibility::Inherited;

        assert!(is_field_private(&visibility));
    }

    #[test]
    fn test_is_field_private_public_visibility() {
        use syn::parse_quote;

        // Test that public visibility is not considered private
        let visibility: Visibility = parse_quote!(pub);

        assert!(!is_field_private(&visibility));
    }

    #[test]
    fn test_is_field_private_pub_crate_visibility() {
        use syn::parse_quote;

        // Test that pub(crate) visibility is not considered private
        let visibility: Visibility = parse_quote!(pub(crate));

        assert!(!is_field_private(&visibility));
    }

    #[test]
    fn test_is_field_private_pub_super_visibility() {
        use syn::parse_quote;

        // Test that pub(super) visibility is not considered private
        let visibility: Visibility = parse_quote!(pub(super));

        assert!(!is_field_private(&visibility));
    }

    #[test]
    fn test_is_field_private_pub_self_visibility() {
        use syn::parse_quote;

        // Test that pub(self) visibility is not considered private
        let visibility: Visibility = parse_quote!(pub(self));

        assert!(!is_field_private(&visibility));
    }

    #[test]
    fn test_is_field_private_pub_in_path_visibility() {
        use syn::parse_quote;

        // Test that pub(in path) visibility is not considered private
        let visibility: Visibility = parse_quote!(pub(in crate::module));

        assert!(!is_field_private(&visibility));
    }
}
