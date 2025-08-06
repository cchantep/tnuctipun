//! Tests for regex filter operations

use super::test_fixtures::*;
use bson::doc;
use tnuctipun::filters::empty;

#[test]
fn test_basic_regex_filter() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("Laptop", None)
        .and();

    let expected = doc! {
        "name": { "$regex": "Laptop" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_case_insensitive_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("laptop", Some("i"))
        .and();

    let expected = doc! {
        "name": { "$regex": "laptop", "$options": "i" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_partial_match_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>(".*Gaming.*", None)
        .and();

    let expected = doc! {
        "name": { "$regex": ".*Gaming.*" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_starts_with_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("^Samsung", None)
        .and();

    let expected = doc! {
        "name": { "$regex": "^Samsung" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_ends_with_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("Pro$", None)
        .and();

    let expected = doc! {
        "name": { "$regex": "Pro$" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiline_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("^Product", Some("m"))
        .and();

    let expected = doc! {
        "name": { "$regex": "^Product", "$options": "m" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_case_insensitive_multiline_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("laptop", Some("im"))
        .and();

    let expected = doc! {
        "name": { "$regex": "laptop", "$options": "im" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_email_validation_regex() {
    // Test using a simple string field instead of nested
    let result = empty::<Product>()
        .regex::<product_fields::Name>(r"^[\w\.-]+@[\w\.-]+\.\w+$", None)
        .and();

    let expected = doc! {
        "name": { "$regex": r"^[\w\.-]+@[\w\.-]+\.\w+$" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_digits_only_regex() {
    let result = empty::<Product>()
        .regex::<product_fields::Id>(r"^\d+$", None)
        .and();

    let expected = doc! {
        "id": { "$regex": r"^\d+$" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_regex_filters() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("laptop", Some("i"))
        .regex::<product_fields::Brand>("^Dell", None)
        .and();

    let expected = doc! {
        "$and": [
            { "name": { "$regex": "laptop", "$options": "i" } },
            { "brand": { "$regex": "^Dell" } }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_with_other_filters() {
    let result = empty::<Product>()
        .regex::<product_fields::Name>("Gaming", None)
        .gt::<product_fields::Price, _>(500.0)
        .eq::<product_fields::Stock, _>(10)
        .and();

    let expected = doc! {
        "$and": [
            { "name": { "$regex": "Gaming" } },
            { "price": { "$gt": 500.0 } },
            { "stock": 10 }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_escape_special_characters() {
    // Test that special regex characters are handled correctly
    let result = empty::<Product>()
        .regex::<product_fields::Name>(r"C\+\+", None)
        .and();

    let expected = doc! {
        "name": { "$regex": r"C\+\+" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_with_nested_field() {
    let result = empty::<User>()
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.regex::<address_fields::City>("new york", Some("i")),
        )
        .and();

    let expected = doc! {
        "home_address.city": { "$regex": "new york", "$options": "i" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_single_clause_optimization() {
    // Test that single regex clause doesn't get wrapped in $and
    let result = empty::<Product>()
        .regex::<product_fields::Name>("MacBook", None)
        .and();

    let expected = doc! {
        "name": { "$regex": "MacBook" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_unicode_regex() {
    // Test Unicode support in regex patterns
    let result = empty::<Product>()
        .regex::<product_fields::Name>("Café", None)
        .and();

    let expected = doc! {
        "name": { "$regex": "Café" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_word_boundaries() {
    // Test word boundary regex patterns
    let result = empty::<Product>()
        .regex::<product_fields::Name>(r"\bLaptop\b", None)
        .and();

    let expected = doc! {
        "name": { "$regex": r"\bLaptop\b" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_dotall_option() {
    // Test the 's' option (dotall - allows . to match newlines)
    let result = empty::<Product>()
        .regex::<product_fields::Name>("start.*end", Some("s"))
        .and();

    let expected = doc! {
        "name": { "$regex": "start.*end", "$options": "s" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_extended_option() {
    // Test the 'x' option (extended - ignores whitespace and allows comments)
    let result = empty::<Product>()
        .regex::<product_fields::Name>(r"lap top", Some("x"))
        .and();

    let expected = doc! {
        "name": { "$regex": r"lap top", "$options": "x" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_all_options_combined() {
    // Test combining multiple regex options
    let result = empty::<Product>()
        .regex::<product_fields::Name>("^product.*details$", Some("imsx"))
        .and();

    let expected = doc! {
        "name": { "$regex": "^product.*details$", "$options": "imsx" }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_regex_empty_options() {
    // Test that empty options string doesn't add $options field
    let result = empty::<Product>()
        .regex::<product_fields::Name>("test", Some(""))
        .and();

    let expected = doc! {
        "name": { "$regex": "test" }
    };

    assert_eq!(result, expected);
}
