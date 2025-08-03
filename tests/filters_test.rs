use nessus_derive::MongoComparable;

use nessus::FieldWitnesses;
use nessus::filters::empty;
use serde::{Deserialize, Serialize};

// Define test structs
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    id: String,
    name: String,
    price: f64,
    stock: i32,
    categories: Vec<String>,
}

// Define nested structs for testing with_lookup function
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
    country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct ContactInfo {
    email: String,
    phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    id: String,
    name: String,
    age: i32,
    home_address: Address,
    work_address: Address,
    contact: ContactInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Company {
    name: String,
    address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Employee {
    id: String,
    name: String,
    company: Company,
}

#[test]
fn test_equality_filter() {
    let mut builder = empty::<Product>();
    builder.eq::<product_fields::Name, _>("Smartphone".to_string());

    let expected_name = bson::doc! { "name": "Smartphone" };
    assert_eq!(builder.clauses(), &vec![expected_name.clone()]);

    builder.eq::<product_fields::Categories, _>("Electronics".to_string());
    let expected_categories = bson::doc! { "categories": "Electronics" };
    assert_eq!(builder.clauses(), &vec![expected_name, expected_categories]);
}

#[test]
fn test_greater_than_filter() {
    let mut builder = empty::<Product>();

    builder.gt::<product_fields::Price, _>(500.0);

    let expected = bson::doc! { "price": { "$gt": 500.0 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);

    let mut builder2 = empty::<Product>();

    builder2.gt::<product_fields::Categories, _>(vec!["Electronics".to_string()]);

    let expected_categories = bson::doc! { "categories": { "$gt": ["Electronics"] } };

    assert_eq!(builder2.clauses(), &vec![expected_categories.clone()]);

    // Even if Price field is f64, we can use an integer
    // This should work because f64 can be compared with i32
    let mut builder3 = empty::<Product>();

    builder3.gt::<product_fields::Price, _>(1000);

    let expected_int = bson::doc! { "price": { "$gt": 1000 } };

    assert_eq!(builder3.clauses(), &vec![expected_int.clone()]);
}

#[test]
fn test_less_than_filter() {
    let mut builder = empty::<Product>();

    builder.lt::<product_fields::Stock, _>(20);

    let expected = bson::doc! { "stock": { "$lt": 20 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_in_filter() {
    let mut builder = empty::<Product>();

    builder.r#in::<product_fields::Id, _>(vec!["prod-123".to_string(), "prod-456".to_string()]);

    let expected = bson::doc! { "id": { "$in": ["prod-123", "prod-456"] } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_ne_filter() {
    let mut builder = empty::<Product>();

    builder.ne::<product_fields::Name, _>("Laptop".to_string());

    let expected = bson::doc! { "name": { "$ne": "Laptop" } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_greater_than_equal_filter() {
    let mut builder = empty::<Product>();

    builder.gte::<product_fields::Price, _>(1000.0);

    let expected = bson::doc! { "price": { "$gte": 1000.0 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_less_than_equal_filter() {
    let mut builder = empty::<Product>();

    builder.lte::<product_fields::Price, _>(1000.0);

    let expected = bson::doc! { "price": { "$lte": 1000.0 } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_exists_filter() {
    let mut builder = empty::<Product>();

    builder.exists::<product_fields::Categories>(true);

    let expected = bson::doc! { "categories": { "$exists": true } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

#[test]
fn test_nin_filter() {
    // For a field that's a Vec<String>, we need to create a Vec<Vec<String>>
    let category1 = vec!["Electronics".to_string()];
    let category2 = vec!["Books".to_string()];
    let categories = vec![category1, category2];

    let mut builder = empty::<Product>();

    builder.nin::<product_fields::Categories, _>(categories);

    let expected = bson::doc! { "categories": { "$nin": [["Electronics"], ["Books"]] } };

    assert_eq!(builder.clauses(), &vec![expected.clone()]);
}

// Test compile-time safety
#[test]
fn test_compile_time_safety() {
    // Note: Compile-time type safety tests are handled by trybuild tests in compile_fail/ directory:
    // - filters_wrong_type.rs: Tests that using the wrong type causes a compile error
    // - filters_nonexistent_field.rs: Tests that using a nonexistent field causes a compile error

    // Verify correct usage works
    let mut builder = empty::<Product>();

    builder.eq::<product_fields::Price, _>(599.99);

    let expected_price_filter = bson::doc! { "price": 599.99 };

    assert_eq!(builder.clauses(), &vec![expected_price_filter.clone()]);
}

#[test]
fn test_and_function_empty_builder() {
    // Test with empty builder - should return empty document
    let builder = empty::<Product>();
    let result = builder.and();
    let expected = bson::doc! {};

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_single_clause() {
    // Test with single clause - should return the clause directly (no $and wrapper)
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Smartphone".to_string())
        .and();
    
    let expected = bson::doc! { "name": "Smartphone" };
    assert_eq!(result, expected);
}

#[test]
fn test_and_function_multiple_clauses() {
    // Test with multiple clauses - should wrap in $and
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Smartphone".to_string())
        .eq::<product_fields::Price, _>(599.99)
        .and();
        
    let expected = bson::doc! {
        "$and": [
            { "name": "Smartphone" },
            { "price": 599.99 }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_complex_builder_chain() {
    // Test with a complex chain of different filter types
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Gaming Laptop".to_string())
        .eq::<product_fields::Stock, _>(10)
        .and();

    let expected = bson::doc! {
        "$and": [
            { "name": "Gaming Laptop" },
            { "stock": 10 }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_and_function_three_clauses() {
    // Test with three clauses to ensure proper array construction
    let result = empty::<Product>()
        .eq::<product_fields::Name, _>("Tablet".to_string())
        .eq::<product_fields::Price, _>(299.99)
        .eq::<product_fields::Stock, _>(5)
        .and();

    let expected = bson::doc! {
        "$and": [
            { "name": "Tablet" },
            { "price": 299.99 },
            { "stock": 5 }
        ]
    };

    assert_eq!(result, expected);
}

// Tests for with_lookup function
#[test]
fn test_with_lookup_single_field() {
    // Test filtering on a single nested field
    let mut builder = empty::<User>();

    builder.with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
        |path| path.field::<address_fields::City>(),
        |nested| nested.eq::<address_fields::City, _>("New York".to_string()),
    );

    let expected = bson::doc! { "home_address.city": "New York" };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_with_lookup_multiple_fields() {
    // Test filtering on multiple nested fields within the same nested object
    let mut builder = empty::<User>();

    builder
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.eq::<address_fields::City, _>("San Francisco".to_string()),
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::ZipCode, Address, _>(
            |path| path.field::<address_fields::ZipCode>(),
            |nested| nested.eq::<address_fields::ZipCode, _>("94102".to_string()),
        );

    let expected_city = bson::doc! { "home_address.city": "San Francisco" };
    let expected_zip = bson::doc! { "home_address.zip_code": "94102" };

    assert_eq!(builder.clauses(), &vec![expected_city, expected_zip]);
}

#[test]
fn test_with_lookup_different_operators() {
    // Test using different MongoDB operators within nested fields
    let mut builder = empty::<User>();

    builder
        .with_lookup::<user_fields::HomeAddress, _, address_fields::Country, Address, _>(
            |path| path.field::<address_fields::Country>(),
            |nested| nested.eq::<address_fields::Country, _>("USA".to_string()),
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.ne::<address_fields::City, _>("Los Angeles".to_string()),
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::ZipCode, Address, _>(
            |path| path.field::<address_fields::ZipCode>(),
            |nested| nested.exists::<address_fields::ZipCode>(true),
        );

    let expected_country = bson::doc! { "home_address.country": "USA" };
    let expected_city = bson::doc! { "home_address.city": { "$ne": "Los Angeles" } };
    let expected_zip_exists = bson::doc! { "home_address.zip_code": { "$exists": true } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_country, expected_city, expected_zip_exists]
    );
}

#[test]
fn test_with_lookup_multiple_nested_objects() {
    // Test filtering on different nested objects within the same parent
    let mut builder = empty::<User>();

    builder
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.eq::<address_fields::City, _>("Boston".to_string()),
        )
        .with_lookup::<user_fields::WorkAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.eq::<address_fields::City, _>("Cambridge".to_string()),
        );

    let expected_home = bson::doc! { "home_address.city": "Boston" };
    let expected_work = bson::doc! { "work_address.city": "Cambridge" };

    assert_eq!(builder.clauses(), &vec![expected_home, expected_work]);
}

#[test]
fn test_with_lookup_mixed_with_regular_filters() {
    // Test combining nested filters with regular field filters
    let mut builder = empty::<User>();

    builder
        .eq::<user_fields::Name, _>("John Doe".to_string())
        .with_lookup::<user_fields::Contact, _, contactinfo_fields::Email, ContactInfo, _>(
            |path| path.field::<contactinfo_fields::Email>(),
            |nested| nested.eq::<contactinfo_fields::Email, _>("john@example.com".to_string()),
        )
        .gt::<user_fields::Age, _>(25);

    let expected_name = bson::doc! { "name": "John Doe" };
    let expected_email = bson::doc! { "contact.email": "john@example.com" };
    let expected_age = bson::doc! { "age": { "$gt": 25 } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_name, expected_email, expected_age]
    );
}

#[test]
fn test_with_lookup_deep_nesting() {
    // Test deeply nested structures (Company -> Address)
    let mut builder = empty::<Employee>();

    builder
        .with_lookup::<employee_fields::Company, _, company_fields::Name, Company, _>(
            |path| path.field::<company_fields::Name>(),
            |nested| nested.eq::<company_fields::Name, _>("Tech Corp".to_string()),
        )
        .with_lookup::<employee_fields::Company, _, company_fields::Address, Company, _>(
            |path| path.field::<company_fields::Address>(),
            |nested| {
                nested
                    .with_lookup::<company_fields::Address, _, address_fields::City, Address, _>(
                        |path| path.field::<address_fields::City>(),
                        |deeply_nested| {
                            deeply_nested.eq::<address_fields::City, _>("Seattle".to_string())
                        },
                    )
                    .with_lookup::<company_fields::Address, _, address_fields::Country, Address, _>(
                        |path| path.field::<address_fields::Country>(),
                        |deeply_nested| {
                            deeply_nested.eq::<address_fields::Country, _>("USA".to_string())
                        },
                    )
            },
        );

    let expected_company_name = bson::doc! { "company.name": "Tech Corp" };
    let expected_company_city = bson::doc! { "company.address.city": "Seattle" };
    let expected_company_country = bson::doc! { "company.address.country": "USA" };

    assert_eq!(
        builder.clauses(),
        &vec![
            expected_company_name,
            expected_company_city,
            expected_company_country
        ]
    );
}

#[test]
fn test_with_lookup_and_function() {
    // Test that nested fields work correctly with the and() function
    let result = empty::<User>()
        .eq::<user_fields::Name, _>("Alice Smith".to_string())
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.eq::<address_fields::City, _>("Portland".to_string()),
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::ZipCode, Address, _>(
            |path| path.field::<address_fields::ZipCode>(),
            |nested| nested.eq::<address_fields::ZipCode, _>("97201".to_string()),
        )
        .and();

    let expected = bson::doc! {
        "$and": [
            { "name": "Alice Smith" },
            { "home_address.city": "Portland" },
            { "home_address.zip_code": "97201" }
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_single_clause_and_function() {
    // Test that a single nested clause works correctly with and() function
    let result = empty::<User>()
        .with_lookup::<user_fields::Contact, _, contactinfo_fields::Phone, ContactInfo, _>(
            |path| path.field::<contactinfo_fields::Phone>(),
            |nested| nested.eq::<contactinfo_fields::Phone, _>("+1-555-0123".to_string()),
        )
        .and();
        
    let expected = bson::doc! { "contact.phone": "+1-555-0123" };
    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_complex_operators() {
    // Test nested fields with complex MongoDB operators like $in and $nin
    let mut builder = empty::<User>();

    builder
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| {
                nested.r#in::<address_fields::City, _>(vec![
                    "New York".to_string(),
                    "Boston".to_string(),
                    "Chicago".to_string(),
                ])
            },
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::Country, Address, _>(
            |path| path.field::<address_fields::Country>(),
            |nested| {
                nested.nin::<address_fields::Country, _>(vec![
                    "Canada".to_string(),
                    "Mexico".to_string(),
                ])
            },
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::ZipCode, Address, _>(
            |path| path.field::<address_fields::ZipCode>(),
            |nested| nested.gte::<address_fields::ZipCode, _>("10000".to_string()),
        )
        .with_lookup::<user_fields::HomeAddress, _, address_fields::ZipCode, Address, _>(
            |path| path.field::<address_fields::ZipCode>(),
            |nested| nested.lte::<address_fields::ZipCode, _>("99999".to_string()),
        );

    let expected_city_in =
        bson::doc! { "home_address.city": { "$in": ["New York", "Boston", "Chicago"] } };
    let expected_country_nin =
        bson::doc! { "home_address.country": { "$nin": ["Canada", "Mexico"] } };
    let expected_zip_gte = bson::doc! { "home_address.zip_code": { "$gte": "10000" } };
    let expected_zip_lte = bson::doc! { "home_address.zip_code": { "$lte": "99999" } };

    assert_eq!(
        builder.clauses(),
        &vec![
            expected_city_in,
            expected_country_nin,
            expected_zip_gte,
            expected_zip_lte
        ]
    );
}

#[test]
fn test_or_function_simple_equality() {
    // Test OR with simple equality conditions
    let mut builder = empty::<Product>();
    let names = vec!["Laptop", "Smartphone", "Tablet"];

    builder.or::<product_fields::Name, _, _>(names, |filter, name| {
        filter.eq::<product_fields::Name, _>(name.to_string())
    });

    let expected = bson::doc! {
        "$or": [
            { "name": "Laptop" },
            { "name": "Smartphone" },
            { "name": "Tablet" }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_with_price_ranges() {
    // Test OR with complex conditions (price ranges)
    let mut builder = empty::<Product>();
    let price_ranges = vec![(0.0, 100.0), (500.0, 1000.0), (2000.0, 5000.0)];

    builder.or::<product_fields::Price, _, _>(price_ranges, |filter, (min, max)| {
        filter
            .gte::<product_fields::Price, _>(min)
            .lte::<product_fields::Price, _>(max)
    });

    // The implementation flattens multiple clauses from each iteration into the $or array
    let expected = bson::doc! {
        "$or": [
            { "price": { "$gte": 0.0 } },
            { "price": { "$lte": 100.0 } },
            { "price": { "$gte": 500.0 } },
            { "price": { "$lte": 1000.0 } },
            { "price": { "$gte": 2000.0 } },
            { "price": { "$lte": 5000.0 } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_with_different_operators() {
    // Test OR with different MongoDB operators
    let mut builder = empty::<Product>();
    let search_criteria = vec!["Electronics", "Books", "Clothing"];

    builder.or::<product_fields::Categories, _, _>(search_criteria, |filter, category| {
        filter
            .eq::<product_fields::Categories, _>(vec![category.to_string()])
            .exists::<product_fields::Name>(true)
    });

    // The implementation flattens multiple clauses from each iteration into the $or array
    let expected = bson::doc! {
        "$or": [
            { "categories": ["Electronics"] },
            { "name": { "$exists": true } },
            { "categories": ["Books"] },
            { "name": { "$exists": true } },
            { "categories": ["Clothing"] },
            { "name": { "$exists": true } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_empty_input() {
    // Test OR with empty input - should add empty $or array
    let mut builder = empty::<Product>();
    let empty_vec: Vec<String> = vec![];

    builder.or::<product_fields::Name, _, _>(empty_vec, |filter, name| {
        filter.eq::<product_fields::Name, _>(name)
    });

    let expected = bson::doc! { "$or": [] };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_single_item() {
    // Test OR with single item
    let mut builder = empty::<Product>();
    let single_name = vec!["Laptop"];

    builder.or::<product_fields::Name, _, _>(single_name, |filter, name| {
        filter.eq::<product_fields::Name, _>(name.to_string())
    });

    let expected = bson::doc! {
        "$or": [
            { "name": "Laptop" }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_chained_with_other_conditions() {
    // Test OR chained with other filter conditions
    let mut builder = empty::<Product>();

    // First add a regular condition
    builder.gt::<product_fields::Stock, _>(0);

    // Then add OR condition
    let categories = vec!["Electronics", "Books"];
    builder.or::<product_fields::Categories, _, _>(categories, |filter, category| {
        filter.eq::<product_fields::Categories, _>(vec![category.to_string()])
    });

    // Add another regular condition
    builder.lte::<product_fields::Price, _>(1000.0);

    let expected_stock = bson::doc! { "stock": { "$gt": 0 } };
    let expected_or = bson::doc! {
        "$or": [
            { "categories": ["Electronics"] },
            { "categories": ["Books"] }
        ]
    };
    let expected_price = bson::doc! { "price": { "$lte": 1000.0 } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_stock, expected_or, expected_price]
    );
}

#[test]
fn test_or_function_with_lookup_fields() {
    // Test OR with nested field access
    let mut builder = empty::<User>();
    let cities = vec!["New York", "Boston", "Chicago"];

    builder.or::<user_fields::HomeAddress, _, _>(cities, |filter, city| {
        filter.with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.eq::<address_fields::City, _>(city.to_string()),
        )
    });

    let expected = bson::doc! {
        "$or": [
            { "home_address.city": "New York" },
            { "home_address.city": "Boston" },
            { "home_address.city": "Chicago" }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_complex_nested_conditions() {
    // Test OR with complex nested conditions
    let mut builder = empty::<User>();
    let age_ranges = vec![(18, 25), (30, 40), (50, 65)];

    builder.or::<user_fields::Age, _, _>(age_ranges, |filter, (min_age, max_age)| {
        filter
            .gte::<user_fields::Age, _>(min_age)
            .lte::<user_fields::Age, _>(max_age)
            .with_lookup::<user_fields::Contact, _, contactinfo_fields::Email, ContactInfo, _>(
                |path| path.field::<contactinfo_fields::Email>(),
                |nested| nested.exists::<contactinfo_fields::Email>(true),
            )
    });

    // The implementation flattens multiple clauses from each iteration into the $or array
    let expected = bson::doc! {
        "$or": [
            { "age": { "$gte": 18 } },
            { "age": { "$lte": 25 } },
            { "contact.email": { "$exists": true } },
            { "age": { "$gte": 30 } },
            { "age": { "$lte": 40 } },
            { "contact.email": { "$exists": true } },
            { "age": { "$gte": 50 } },
            { "age": { "$lte": 65 } },
            { "contact.email": { "$exists": true } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_and_final_result() {
    // Test OR with and() function to get final MongoDB query
    let names = vec!["Laptop", "Smartphone"];

    let result = empty::<Product>()
        .gt::<product_fields::Price, _>(100.0)
        .or::<product_fields::Name, _, _>(names, |filter, name| {
            filter.eq::<product_fields::Name, _>(name.to_string())
        })
        .and();
        
    let expected = bson::doc! {
        "$and": [
            { "price": { "$gt": 100.0 } },
            { "$or": [
                { "name": "Laptop" },
                { "name": "Smartphone" }
            ]}
        ]
    };

    assert_eq!(result, expected);
}

#[test]
fn test_or_function_realistic_usage() {
    // Test OR with realistic usage where each closure generates a single condition
    let mut builder = empty::<Product>();
    let category_filters = vec!["Electronics", "Books", "Clothing"];

    builder.or::<product_fields::Categories, _, _>(category_filters, |filter, category| {
        // Each iteration should ideally generate a single meaningful condition
        filter.eq::<product_fields::Categories, _>(vec![category.to_string()])
    });

    let expected = bson::doc! {
        "$or": [
            { "categories": ["Electronics"] },
            { "categories": ["Books"] },
            { "categories": ["Clothing"] }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_or_function_with_mixed_single_and_multiple_clauses() {
    // Test that demonstrates mixing single and multiple clause generations
    let mut builder = empty::<Product>();
    let mixed_conditions = vec![("simple", 100.0), ("range", 500.0)];

    builder.or::<product_fields::Price, _, _>(
        mixed_conditions,
        |filter, (condition_type, value)| match condition_type {
            "simple" => filter.eq::<product_fields::Price, _>(value),
            "range" => filter
                .gte::<product_fields::Price, _>(value)
                .lte::<product_fields::Price, _>(value * 2.0),
            _ => filter,
        },
    );

    // Shows how multiple clauses from "range" condition get flattened
    let expected = bson::doc! {
        "$or": [
            { "price": 100.0 },
            { "price": { "$gte": 500.0 } },
            { "price": { "$lte": 1000.0 } }
        ]
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_single_equality() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()));

    let expected = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_numeric_equality() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.eq(500.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$eq": 500.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_integer_field() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Stock, _>(|op| op.eq(10));

    let expected = bson::doc! {
        "stock": {
            "$not": {
                "stock": { "$eq": 10 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_not_filter_chained_with_other_filters() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .gt::<product_fields::Price, _>(100.0);

    let expected_not = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };
    let expected_gt = bson::doc! { "price": { "$gt": 100.0 } };

    assert_eq!(builder.clauses(), &vec![expected_not, expected_gt]);
}

#[test]
fn test_not_filter_multiple_not_operations() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .not::<product_fields::Price, _>(|op| op.eq(500.0));

    let expected_not_name = bson::doc! {
        "name": {
            "$not": {
                "name": { "$eq": "Smartphone" }
            }
        }
    };
    let expected_not_price = bson::doc! {
        "price": {
            "$not": {
                "price": { "$eq": 500.0 }
            }
        }
    };

    assert_eq!(
        builder.clauses(),
        &vec![expected_not_name, expected_not_price]
    );
}

#[test]
fn test_not_filter_with_and_combination() {
    let filter = empty::<Product>()
        .not::<product_fields::Name, _>(|op| op.eq("Smartphone".to_string()))
        .eq::<product_fields::Categories, _>("Electronics".to_string())
        .and();

    let expected = bson::doc! {
        "$and": [
            {
                "name": {
                    "$not": {
                        "name": { "$eq": "Smartphone" }
                    }
                }
            },
            { "categories": "Electronics" }
        ]
    };

    assert_eq!(filter, expected);
}

#[test]
fn test_not_filter_type_compatibility() {
    let mut builder = empty::<Product>();

    // Test that we can use i32 for f64 price field due to MongoComparable
    builder.not::<product_fields::Price, _>(|op| {
        op.eq(100) // i32 instead of f64
    });

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$eq": 100 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

// Tests for OperationBuilder functions
#[test]
fn test_operation_builder_gt() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.gt(100.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gt": 100.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_gte() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.gte(50.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gte": 50.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_lt() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Stock, _>(|op| op.lt(5));

    let expected = bson::doc! {
        "stock": {
            "$not": {
                "stock": { "$lt": 5 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_lte() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.lte(200.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$lte": 200.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_in() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Categories, _>(|op| {
        op.r#in(vec!["Electronics".to_string(), "Books".to_string()])
    });

    let expected = bson::doc! {
        "categories": {
            "$not": {
                "categories": { "$in": ["Electronics", "Books"] }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_nin() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Categories, _>(|op| {
        op.nin(vec!["Clothing".to_string(), "Shoes".to_string()])
    });

    let expected = bson::doc! {
        "categories": {
            "$not": {
                "categories": { "$nin": ["Clothing", "Shoes"] }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_exists() {
    let mut builder = empty::<User>();

    builder.not::<user_fields::Contact, _>(|op| op.exists(true));

    let expected = bson::doc! {
        "contact": {
            "$not": {
                "contact": { "$exists": true }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_exists_false() {
    let mut builder = empty::<User>();

    builder.not::<user_fields::Contact, _>(|op| op.exists(false));

    let expected = bson::doc! {
        "contact": {
            "$not": {
                "contact": { "$exists": false }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_chaining_multiple_operations() {
    let mut builder = empty::<Product>();

    builder.not::<product_fields::Price, _>(|op| op.gte(100.0).lte(500.0));

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gte": 100.0, "$lte": 500.0 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_type_compatibility_gt() {
    let mut builder = empty::<Product>();

    // Test that we can use i32 for f64 price field due to MongoComparable
    builder.not::<product_fields::Price, _>(|op| {
        op.gt(100) // i32 instead of f64
    });

    let expected = bson::doc! {
        "price": {
            "$not": {
                "price": { "$gt": 100 }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_type_compatibility_in() {
    let mut builder = empty::<User>();

    // Test that we can use Vec<i32> for i32 age field
    builder.not::<user_fields::Age, _>(|op| op.r#in(vec![20, 30, 40]));

    let expected = bson::doc! {
        "age": {
            "$not": {
                "age": { "$in": [20, 30, 40] }
            }
        }
    };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_operation_builder_complex_chain() {
    let mut builder = empty::<Product>();

    builder
        .not::<product_fields::Price, _>(|op| op.gt(50.0).lt(1000.0))
        .eq::<product_fields::Name, _>("Laptop".to_string());

    let expected_not = bson::doc! {
        "price": {
            "$not": {
                "price": {
                    "$gt": 50.0,
                    "$lt": 1000.0
                }
            }
        }
    };
    let expected_eq = bson::doc! { "name": "Laptop" };

    assert_eq!(builder.clauses(), &vec![expected_not, expected_eq]);
}

// Tests for with_field function (convenience method using identity)
#[test]
fn test_with_field_simple_exists() {
    // Test with_field for simple field existence check
    let mut builder = empty::<User>();

    builder.with_field::<user_fields::HomeAddress, _>(|nested| {
        nested.exists::<user_fields::HomeAddress>(true)
    });

    let expected = bson::doc! { "home_address": { "$exists": true } };

    assert_eq!(builder.clauses(), &vec![expected]);
}

#[test]
fn test_with_field_combined_operations() {
    // Test with_field with multiple operations on the same field context
    let mut builder = empty::<User>();

    builder.with_field::<user_fields::Age, _>(|nested| {
        nested
            .gte::<user_fields::Age, _>(21)
            .lte::<user_fields::Age, _>(65)
    });

    let expected_gte = bson::doc! { "age": { "$gte": 21 } };
    let expected_lte = bson::doc! { "age": { "$lte": 65 } };

    assert_eq!(builder.clauses(), &vec![expected_gte, expected_lte]);
}

#[test]
fn test_with_field_vs_direct_comparison() {
    // Test that with_field produces the same result as direct field operations
    let mut builder1 = empty::<Product>();
    let mut builder2 = empty::<Product>();

    // Using with_field
    builder1.with_field::<product_fields::Name, _>(|nested| {
        nested.eq::<product_fields::Name, _>("Laptop".to_string())
    });

    // Using direct field operation
    builder2.eq::<product_fields::Name, _>("Laptop".to_string());

    assert_eq!(builder1.clauses(), builder2.clauses());
}

// Tests for deeper nested paths using with_lookup
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct DetailedAddress {
    street: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
    location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct PersonalInfo {
    first_name: String,
    last_name: String,
    date_of_birth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct DetailedUser {
    id: String,
    personal_info: PersonalInfo,
    home_address: DetailedAddress,
    work_address: DetailedAddress,
}

#[test]
fn test_with_lookup_three_level_nesting() {
    // Test navigating through three levels: User -> Address -> Location
    let mut builder = empty::<DetailedUser>();

    builder.with_lookup::<detaileduser_fields::HomeAddress, _, detailedaddress_fields::Location, DetailedAddress, _>(
        |path| path.field::<detailedaddress_fields::Location>(),
        |nested| {
            nested.with_lookup::<detailedaddress_fields::Location, _, location_fields::Latitude, Location, _>(
                |path| path.field::<location_fields::Latitude>(),
                |deeply_nested| {
                    deeply_nested.gte::<location_fields::Latitude, _>(40.0)
                }
            ).with_lookup::<detailedaddress_fields::Location, _, location_fields::Longitude, Location, _>(
                |path| path.field::<location_fields::Longitude>(),
                |deeply_nested| {
                    deeply_nested.lte::<location_fields::Longitude, _>(-70.0)
                }
            )
        }
    );

    let expected_lat = bson::doc! { "home_address.location.latitude": { "$gte": 40.0 } };
    let expected_lng = bson::doc! { "home_address.location.longitude": { "$lte": -70.0 } };

    assert_eq!(builder.clauses(), &vec![expected_lat, expected_lng]);
}

#[test]
fn test_with_lookup_complex_deep_structure() {
    // Test complex filtering on deeply nested structures
    let mut builder = empty::<DetailedUser>();

    builder
        .with_lookup::<detaileduser_fields::PersonalInfo, _, personalinfo_fields::FirstName, PersonalInfo, _>(
            |path| path.field::<personalinfo_fields::FirstName>(),
            |nested| {
                nested.eq::<personalinfo_fields::FirstName, _>("John".to_string())
            }
        )
        .with_lookup::<detaileduser_fields::HomeAddress, _, detailedaddress_fields::City, DetailedAddress, _>(
            |path| path.field::<detailedaddress_fields::City>(),
            |nested| {
                nested.eq::<detailedaddress_fields::City, _>("New York".to_string())
            }
        )
        .with_lookup::<detaileduser_fields::WorkAddress, _, detailedaddress_fields::Location, DetailedAddress, _>(
            |path| path.field::<detailedaddress_fields::Location>(),
            |nested| {
                nested.with_lookup::<detailedaddress_fields::Location, _, location_fields::Latitude, Location, _>(
                    |path| path.field::<location_fields::Latitude>(),
                    |deeply_nested| {
                        deeply_nested.gt::<location_fields::Latitude, _>(41.0)
                    }
                )
            }
        );

    let expected_name = bson::doc! { "personal_info.first_name": "John" };
    let expected_home_city = bson::doc! { "home_address.city": "New York" };
    let expected_work_lat = bson::doc! { "work_address.location.latitude": { "$gt": 41.0 } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_name, expected_home_city, expected_work_lat]
    );
}

#[test]
fn test_with_lookup_four_level_deep_path() {
    // Test the deepest possible nesting we can reasonably create
    #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    struct Metadata {
        created_by: String,
        accuracy: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    struct EnhancedLocation {
        latitude: f64,
        longitude: f64,
        metadata: Metadata,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    struct SuperDetailedAddress {
        street: String,
        city: String,
        enhanced_location: EnhancedLocation,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    struct UltraDetailedUser {
        id: String,
        name: String,
        address: SuperDetailedAddress,
    }

    let mut builder = empty::<UltraDetailedUser>();

    // Navigate through four levels: User -> Address -> Location -> Metadata
    builder.with_lookup::<ultradetaileduser_fields::Address, _, superdetailedaddress_fields::EnhancedLocation, SuperDetailedAddress, _>(
        |path| path.field::<superdetailedaddress_fields::EnhancedLocation>(),
        |nested| {
            nested.with_lookup::<superdetailedaddress_fields::EnhancedLocation, _, enhancedlocation_fields::Metadata, EnhancedLocation, _>(
                |path| path.field::<enhancedlocation_fields::Metadata>(),
                |deeply_nested| {
                    deeply_nested.with_lookup::<enhancedlocation_fields::Metadata, _, metadata_fields::CreatedBy, Metadata, _>(
                        |path| path.field::<metadata_fields::CreatedBy>(),
                        |ultra_nested| {
                            ultra_nested.eq::<metadata_fields::CreatedBy, _>("GPS_SYSTEM".to_string())
                        }
                    ).with_lookup::<enhancedlocation_fields::Metadata, _, metadata_fields::Accuracy, Metadata, _>(
                        |path| path.field::<metadata_fields::Accuracy>(),
                        |ultra_nested| {
                            ultra_nested.gte::<metadata_fields::Accuracy, _>(0.95)
                        }
                    )
                }
            )
        }
    );

    let expected_created_by =
        bson::doc! { "address.enhanced_location.metadata.created_by": "GPS_SYSTEM" };
    let expected_accuracy =
        bson::doc! { "address.enhanced_location.metadata.accuracy": { "$gte": 0.95 } };

    assert_eq!(
        builder.clauses(),
        &vec![expected_created_by, expected_accuracy]
    );
}

#[test]
fn test_mixed_with_field_and_with_lookup() {
    // Test combining with_field and with_lookup in the same query
    let mut builder = empty::<DetailedUser>();

    builder
        .with_field::<detaileduser_fields::Id, _>(|nested| {
            nested.exists::<detaileduser_fields::Id>(true)
        })
        .with_lookup::<detaileduser_fields::PersonalInfo, _, personalinfo_fields::LastName, PersonalInfo, _>(
            |path| path.field::<personalinfo_fields::LastName>(),
            |nested| {
                nested.ne::<personalinfo_fields::LastName, _>("Unknown".to_string())
            }
        )
        .with_field::<detaileduser_fields::HomeAddress, _>(|nested| {
            nested.exists::<detaileduser_fields::HomeAddress>(true)
        });

    let expected_id_exists = bson::doc! { "id": { "$exists": true } };
    let expected_last_name = bson::doc! { "personal_info.last_name": { "$ne": "Unknown" } };
    let expected_address_exists = bson::doc! { "home_address": { "$exists": true } };

    assert_eq!(
        builder.clauses(),
        &vec![
            expected_id_exists,
            expected_last_name,
            expected_address_exists
        ]
    );
}

// Tests for trait implementations and default constructors
#[test]
fn test_filter_builder_from_trait() {
    // Test From<FilterBuilder<T>> for bson::Document trait implementation
    let mut builder = empty::<Product>();

    builder
        .eq::<product_fields::Name, _>("Test Product".to_string())
        .gt::<product_fields::Price, _>(100.0);

    // Use Into to convert FilterBuilder to bson::Document
    let doc: bson::Document = builder.into();

    let expected = bson::doc! {
        "$and": [
            { "name": "Test Product" },
            { "price": { "$gt": 100.0 } }
        ]
    };

    assert_eq!(doc, expected);
}

#[test]
fn test_filter_builder_from_trait_single_clause() {
    // Test From trait with single clause (no $and wrapper)
    let mut builder = empty::<Product>();

    builder.eq::<product_fields::Name, _>("Single Item".to_string());

    let doc: bson::Document = builder.into();
    let expected = bson::doc! { "name": "Single Item" };

    assert_eq!(doc, expected);
}

#[test]
fn test_filter_builder_from_trait_empty() {
    // Test From trait with empty builder
    let builder = empty::<Product>();

    let doc: bson::Document = builder.into();
    let expected = bson::doc! {};

    assert_eq!(doc, expected);
}

#[test]
fn test_filter_builder_default() {
    // Test Default trait implementation for FilterBuilder<T>
    use nessus::filters::FilterBuilder;

    let builder: FilterBuilder<Product> = Default::default();

    // Default builder should have empty clauses
    assert_eq!(builder.clauses(), &Vec::<bson::Document>::new());

    // Converting to document should give empty doc
    let doc: bson::Document = builder.into();
    assert_eq!(doc, bson::doc! {});
}

#[test]
fn test_filter_builder_default_usage() {
    // Test using Default in practical scenarios
    use nessus::filters::FilterBuilder;

    let mut builder: FilterBuilder<User> = Default::default();

    builder.eq::<user_fields::Name, _>("Default User".to_string());

    let expected = bson::doc! { "name": "Default User" };
    assert_eq!(builder.clauses(), &vec![expected.clone()]);

    let doc: bson::Document = builder.into();
    assert_eq!(doc, expected);
}

#[test]
fn test_operation_builder_default() {
    // Test Default trait implementation for OperationBuilder<F, T>
    use nessus::filters::OperationBuilder;

    let op_builder: OperationBuilder<product_fields::Name, Product> = Default::default();

    // Default OperationBuilder should build to empty operations on the field
    let doc = op_builder.build();

    // Since no operations were added, it should be empty - but the field name should still be present
    // Based on the implementation, an empty OperationBuilder should return an empty document
    let expected = bson::doc! {};
    assert_eq!(doc, expected);
}

#[test]
fn test_operation_builder_default_with_operations() {
    // Test Default OperationBuilder with added operations
    use nessus::filters::OperationBuilder;

    let op_builder: OperationBuilder<product_fields::Price, Product> = Default::default();

    let final_builder = op_builder.gt(100.0).lte(500.0);
    let doc = final_builder.build();

    let expected = bson::doc! {
        "price": {
            "$gt": 100.0,
            "$lte": 500.0
        }
    };

    assert_eq!(doc, expected);
}
