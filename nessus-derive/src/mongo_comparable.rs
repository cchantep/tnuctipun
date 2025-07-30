use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

// This function implements the MongoComparable derive macro
// It automatically implements the MongoComparable trait for a struct
// with different field types, providing evidence that fields of specific types
// can be compared with values of specific types in MongoDB queries.
pub fn derive_mongo_comparable(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get fields from the struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("DeriveMongoComparable only works on structs with named fields"),
        },
        _ => panic!("DeriveMongoComparable only works on structs"),
    };

    // Create a vector to store all our implementations
    let mut impls = Vec::new();
    let mut implemented_types = HashSet::new();

    // Process each field and generate appropriate implementations
    for field in fields {
        let field_type = &field.ty;
        let field_tname = type_to_string(field_type);

        // For any field type T, implement MongoComparable<T, T>
        let self_impl_key = format!("{}_{}", field_tname, field_tname);

        if !implemented_types.contains(&self_impl_key) {
            implemented_types.insert(self_impl_key);

            // DEBUG: println!("{} ==> {:?}: {}", name, &field.ident, field_tname);

            impls.push(quote! {
                impl nessus::mongo_comparable::MongoComparable<#field_type, #field_type> for #name {}
            });
        }

        // Generate MongoComparable implementations based on hardcoded type compatibility rules
        if let Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();

                // Generate the compatible types based on hardcoded rules
                let compatible_types = get_compatible_types_for(&type_name);

                for compatible_type_str in compatible_types {
                    let impl_key = format!("{}_{}", type_name, compatible_type_str);

                    if !implemented_types.contains(&impl_key) {
                        implemented_types.insert(impl_key.clone());

                        // For most types, we can directly use the string as an identifier
                        // Only handle special cases explicitly
                        let compatible_type = if compatible_type_str == "DateTime" {
                            quote! { chrono::DateTime<chrono::Utc> }
                        } else {
                            // Parse the string into an identifier and use it directly
                            let ident = syn::Ident::new(
                                &compatible_type_str,
                                proc_macro2::Span::call_site(),
                            );
                            quote! { #ident }
                        };

                        impls.push(quote! {
                            impl nessus::mongo_comparable::MongoComparable<#field_type, #compatible_type> for #name {}
                        });
                    }
                }
            }
        }

        // Now handle special cases for additional type compatibility
        if let Type::Path(type_path) = field_type {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();

                // Case 1: Option<T> field
                if type_name == "Option" {
                    if let Some(inner_type) = extract_generic_arg(field_type) {
                        // Implement MongoComparable<Option<T>, T>
                        let impl_key = format!(
                            "Option_{}_{}",
                            type_to_string(field_type),
                            type_to_string(&inner_type)
                        );

                        if !implemented_types.contains(&impl_key) {
                            implemented_types.insert(impl_key);
                            impls.push(quote! {
                                impl nessus::mongo_comparable::MongoComparable<#field_type, #inner_type> for #name {}
                            });
                        }

                        // Additionally implement MongoComparable<Option<T>, Option<U>> for all U compatible with T
                        if let Type::Path(inner_type_path) = &inner_type {
                            if let Some(inner_segment) = inner_type_path.path.segments.last() {
                                let inner_type_name = inner_segment.ident.to_string();
                                let compatible_types = get_compatible_types_for(&inner_type_name);

                                for compatible_type_str in compatible_types {
                                    let impl_key = format!(
                                        "Option<{}>_Option<{}>",
                                        inner_type_name, compatible_type_str
                                    );

                                    if !implemented_types.contains(&impl_key) {
                                        implemented_types.insert(impl_key.clone());

                                        // For most types, we can directly use the string as an identifier
                                        // Only handle special cases explicitly
                                        let compatible_type = if compatible_type_str == "DateTime" {
                                            quote! { chrono::DateTime<chrono::Utc> }
                                        } else {
                                            // Parse the string into an identifier and use it directly
                                            let ident = syn::Ident::new(
                                                &compatible_type_str,
                                                proc_macro2::Span::call_site(),
                                            );
                                            quote! { #ident }
                                        };

                                        impls.push(quote! {
                                            impl nessus::mongo_comparable::MongoComparable<#field_type, Option<#compatible_type>> for #name {}
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                // Case 2: IntoIterator<Item=I> field
                else if is_into_iterator_type(&type_name) {
                    if let Some(item_type) = extract_generic_arg(field_type) {
                        // Implement MongoComparable<C, I> where C: IntoIterator<Item=I>
                        let impl_key = format!(
                            "{}_{}",
                            type_to_string(field_type),
                            type_to_string(&item_type)
                        );

                        if !implemented_types.contains(&impl_key) {
                            implemented_types.insert(impl_key);
                            impls.push(quote! {
                                impl nessus::mongo_comparable::MongoComparable<#field_type, #item_type> for #name {}
                            });
                        }
                    }
                }
            }
        }
    }

    // Generate the final token stream with all implementations
    let expanded = quote! {
        #(#impls)*
    };

    TokenStream::from(expanded)
}

// Helper function to get compatible types based on hardcoded MongoDB type compatibility rules
fn get_compatible_types_for(type_name: &str) -> Vec<String> {
    match type_name {
        // Hardcoded MongoDB type compatibility rules
        "i32" => vec!["i16".to_string()],
        "i64" => vec!["i16".to_string(), "i32".to_string()],
        "f64" => vec![
            "i16".to_string(),
            "i32".to_string(),
            "i64".to_string(),
            "f32".to_string(),
        ],
        "char" => vec!["String".to_string()],
        "DateTime" => vec!["i64".to_string()],
        _ => vec![],
    }
}

// Helper function to check if a type is a collection that implements IntoIterator
fn is_into_iterator_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "Vec"
            | "HashSet"
            | "BTreeSet"
            | "LinkedList"
            | "VecDeque"
            | "BinaryHeap"
            | "HashMap"
            | "BTreeMap"
    )
}

// Helper function to extract the generic argument from a type (Option<T> or Collection<T>)
fn extract_generic_arg(ty: &Type) -> Option<Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                    return Some(inner_type.clone());
                }
            }
        }
    }
    None
}

// Helper function to convert a Type to a string for HashSet keys
fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let type_name = segment.ident.to_string();

                // Handle generic types like Option<T>, Vec<T>, etc.
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let generic_args: Vec<String> = args
                        .args
                        .iter()
                        .filter_map(|arg| {
                            if let syn::GenericArgument::Type(inner_type) = arg {
                                Some(type_to_string(inner_type))
                            } else {
                                None
                            }
                        })
                        .collect();

                    if !generic_args.is_empty() {
                        format!("{}<{}>", type_name, generic_args.join(", "))
                    } else {
                        type_name
                    }
                } else {
                    type_name
                }
            } else {
                format!("{:?}", ty) // Fallback to debug format
            }
        }
        _ => format!("{:?}", ty), // For non-path types, use debug format
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_type_to_string_simple_types() {
        // Test simple types
        let string_type: Type = parse_quote!(String);

        assert_eq!(type_to_string(&string_type), "String");

        let i32_type: Type = parse_quote!(i32);

        assert_eq!(type_to_string(&i32_type), "i32");

        let bool_type: Type = parse_quote!(bool);

        assert_eq!(type_to_string(&bool_type), "bool");
    }

    #[test]
    fn test_type_to_string_generic_types() {
        // Test Option<T>
        let option_string: Type = parse_quote!(Option<String>);

        assert_eq!(type_to_string(&option_string), "Option<String>");

        let option_i32: Type = parse_quote!(Option<i32>);

        assert_eq!(type_to_string(&option_i32), "Option<i32>");

        // Test Vec<T>
        let vec_string: Type = parse_quote!(Vec<String>);

        assert_eq!(type_to_string(&vec_string), "Vec<String>");

        // Test nested generics
        let vec_option_string: Type = parse_quote!(Vec<Option<String>>);

        assert_eq!(type_to_string(&vec_option_string), "Vec<Option<String>>");

        let option_vec_i32: Type = parse_quote!(Option<Vec<i32>>);

        assert_eq!(type_to_string(&option_vec_i32), "Option<Vec<i32>>");
    }

    #[test]
    fn test_type_to_string_multiple_generics() {
        // Test types with multiple generic parameters
        let hashmap: Type = parse_quote!(HashMap<String, i32>);

        assert_eq!(type_to_string(&hashmap), "HashMap<String, i32>");

        let result: Type = parse_quote!(Result<String, Error>);

        assert_eq!(type_to_string(&result), "Result<String, Error>");
    }

    #[test]
    fn test_type_to_string_qualified_paths() {
        // Test fully qualified paths
        let chrono_datetime: Type = parse_quote!(chrono::DateTime<chrono::Utc>);

        assert_eq!(type_to_string(&chrono_datetime), "DateTime<Utc>");

        let std_vec: Type = parse_quote!(std::vec::Vec<String>);

        assert_eq!(type_to_string(&std_vec), "Vec<String>");
    }

    #[test]
    fn test_extract_generic_arg_simple() {
        // Test Option<T>
        let option_string: Type = parse_quote!(Option<String>);
        let extracted = extract_generic_arg(&option_string);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "String");

        let option_i32: Type = parse_quote!(Option<i32>);
        let extracted = extract_generic_arg(&option_i32);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "i32");

        // Test Vec<T>
        let vec_string: Type = parse_quote!(Vec<String>);
        let extracted = extract_generic_arg(&vec_string);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "String");
    }

    #[test]
    fn test_extract_generic_arg_nested() {
        // Test nested generics - should extract the outermost generic
        let vec_option_string: Type = parse_quote!(Vec<Option<String>>);
        let extracted = extract_generic_arg(&vec_option_string);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "Option<String>");

        let option_vec_i32: Type = parse_quote!(Option<Vec<i32>>);
        let extracted = extract_generic_arg(&option_vec_i32);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "Vec<i32>");
    }

    #[test]
    fn test_extract_generic_arg_no_generics() {
        // Test types without generics
        let string_type: Type = parse_quote!(String);
        let extracted = extract_generic_arg(&string_type);

        assert!(extracted.is_none());

        let i32_type: Type = parse_quote!(i32);
        let extracted = extract_generic_arg(&i32_type);

        assert!(extracted.is_none());

        let bool_type: Type = parse_quote!(bool);
        let extracted = extract_generic_arg(&bool_type);

        assert!(extracted.is_none());
    }

    #[test]
    fn test_extract_generic_arg_multiple_generics() {
        // Test types with multiple generic parameters - should extract the first one
        let hashmap: Type = parse_quote!(HashMap<String, i32>);
        let extracted = extract_generic_arg(&hashmap);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "String");

        let result: Type = parse_quote!(Result<String, Error>);
        let extracted = extract_generic_arg(&result);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "String");
    }

    #[test]
    fn test_extract_generic_arg_qualified_paths() {
        // Test fully qualified paths with generics
        let std_vec: Type = parse_quote!(std::vec::Vec<String>);
        let extracted = extract_generic_arg(&std_vec);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "String");

        let chrono_datetime: Type = parse_quote!(chrono::DateTime<chrono::Utc>);
        let extracted = extract_generic_arg(&chrono_datetime);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "Utc");
    }

    #[test]
    fn test_extract_generic_arg_collections() {
        // Test various collection types
        let hashset: Type = parse_quote!(HashSet<i32>);
        let extracted = extract_generic_arg(&hashset);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "i32");

        let btreeset: Type = parse_quote!(BTreeSet<String>);
        let extracted = extract_generic_arg(&btreeset);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "String");

        let vecdeque: Type = parse_quote!(VecDeque<bool>);
        let extracted = extract_generic_arg(&vecdeque);

        assert!(extracted.is_some());
        assert_eq!(type_to_string(&extracted.unwrap()), "bool");
    }

    #[test]
    fn test_is_into_iterator_type_collections() {
        // Test vector types
        assert!(is_into_iterator_type("Vec"));
        assert!(is_into_iterator_type("VecDeque"));

        // Test set types
        assert!(is_into_iterator_type("HashSet"));
        assert!(is_into_iterator_type("BTreeSet"));

        // Test list types
        assert!(is_into_iterator_type("LinkedList"));

        // Test heap types
        assert!(is_into_iterator_type("BinaryHeap"));

        // Test map types
        assert!(is_into_iterator_type("HashMap"));
        assert!(is_into_iterator_type("BTreeMap"));
    }

    #[test]
    fn test_is_into_iterator_type_non_collections() {
        // Test primitive types
        assert!(!is_into_iterator_type("String"));
        assert!(!is_into_iterator_type("i32"));
        assert!(!is_into_iterator_type("i64"));
        assert!(!is_into_iterator_type("f64"));
        assert!(!is_into_iterator_type("bool"));
        assert!(!is_into_iterator_type("char"));

        // Test Option (not an iterator itself)
        assert!(!is_into_iterator_type("Option"));

        // Test Result (not an iterator)
        assert!(!is_into_iterator_type("Result"));

        // Test custom types
        assert!(!is_into_iterator_type("MyStruct"));
        assert!(!is_into_iterator_type("User"));
        assert!(!is_into_iterator_type("DateTime"));
    }

    #[test]
    fn test_is_into_iterator_type_edge_cases() {
        // Test empty string
        assert!(!is_into_iterator_type(""));

        // Test similar but not exact matches
        assert!(!is_into_iterator_type("Vector"));
        assert!(!is_into_iterator_type("MyVec"));
        assert!(!is_into_iterator_type("VecLike"));
        assert!(!is_into_iterator_type("HashSetLike"));

        // Test case sensitivity
        assert!(!is_into_iterator_type("vec"));
        assert!(!is_into_iterator_type("hashset"));
        assert!(!is_into_iterator_type("HASHSET"));
    }

    #[test]
    fn test_get_compatible_types_for_numeric_widening() {
        // Test i32 can accept smaller integers
        let i32_compatible = get_compatible_types_for("i32");

        assert_eq!(i32_compatible, vec!["i16"]);

        // Test i64 can accept smaller integers
        let i64_compatible = get_compatible_types_for("i64");

        assert_eq!(i64_compatible, vec!["i16", "i32"]);

        // Test f64 can accept all smaller numeric types
        let f64_compatible = get_compatible_types_for("f64");

        assert_eq!(f64_compatible, vec!["i16", "i32", "i64", "f32"]);
    }

    #[test]
    fn test_get_compatible_types_for_char_string() {
        // Test that char is compatible with String
        let char_compatible = get_compatible_types_for("char");

        assert_eq!(char_compatible, vec!["String"]);
    }

    #[test]
    fn test_get_compatible_types_for_datetime() {
        // Test DateTime compatibility with timestamp
        let datetime_compatible = get_compatible_types_for("DateTime");

        assert_eq!(datetime_compatible, vec!["i64"]);
    }

    #[test]
    fn test_get_compatible_types_for_no_compatibility() {
        // Test types that have no defined compatibility rules
        assert_eq!(get_compatible_types_for("String"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("bool"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("Vec"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("Option"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("HashMap"), Vec::<String>::new());
        assert_eq!(
            get_compatible_types_for("MyCustomType"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_get_compatible_types_for_smaller_types() {
        // Test that smaller types don't have upward compatibility (as expected)
        assert_eq!(get_compatible_types_for("i16"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("f32"), Vec::<String>::new());
    }

    #[test]
    fn test_get_compatible_types_for_edge_cases() {
        // Test edge cases
        assert_eq!(get_compatible_types_for(""), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("unknown"), Vec::<String>::new());

        // Test case sensitivity
        assert_eq!(get_compatible_types_for("I32"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("datetime"), Vec::<String>::new());
        assert_eq!(get_compatible_types_for("CHAR"), Vec::<String>::new());
    }
}
