//! Tests for nested field operations using with_lookup and with_field

use super::test_fixtures::*;
use nessus::updates::empty;

// Tests for with_lookup function
#[test]
fn test_with_lookup_single_nested_field() {
    // Test updating a single nested field
    let result = empty::<User>()
        .with_lookup::<UserHomeAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |nested| {
                nested.set::<AddressCity, _>("New York".to_string());
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "home_address.city": "New York"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_multiple_nested_fields() {
    // Test updating multiple nested fields within the same nested object
    let result = empty::<User>()
        .with_lookup::<UserHomeAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |nested| {
                nested.set::<AddressCity, _>("San Francisco".to_string());
            },
        )
        .with_lookup::<UserHomeAddress, _, AddressZipCode, Address, _>(
            |path| path.field::<AddressZipCode>(),
            |nested| {
                nested.set::<AddressZipCode, _>("94102".to_string());
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "home_address.city": "San Francisco",
            "home_address.zip_code": "94102"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_different_operations() {
    // Test using different MongoDB operators within nested fields
    let result = empty::<User>()
        .with_lookup::<UserHomeAddress, _, AddressCountry, Address, _>(
            |path| path.field::<AddressCountry>(),
            |nested| {
                nested.set::<AddressCountry, _>("USA".to_string());
            },
        )
        .with_lookup::<UserContact, _, ContactEmail, ContactInfo, _>(
            |path| path.field::<ContactEmail>(),
            |nested| {
                nested.unset::<ContactEmail>();
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "home_address.country": "USA"
        },
        "$unset": {
            "contact.email": bson::Bson::Null
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_multiple_nested_objects() {
    // Test updating different nested objects within the same parent
    let result = empty::<User>()
        .with_lookup::<UserHomeAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |nested| {
                nested.set::<AddressCity, _>("Boston".to_string());
            },
        )
        .with_lookup::<UserWorkAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |nested| {
                nested.set::<AddressCity, _>("Cambridge".to_string());
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "home_address.city": "Boston",
            "work_address.city": "Cambridge"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_mixed_with_regular_updates() {
    // Test combining nested updates with regular field updates
    let result = empty::<User>()
        .set::<UserName, _>("John Doe".to_string())
        .with_lookup::<UserContact, _, ContactEmail, ContactInfo, _>(
            |path| path.field::<ContactEmail>(),
            |nested| {
                nested.set::<ContactEmail, _>("john@example.com".to_string());
            },
        )
        .inc::<UserAge, _>(1)
        .build();

    let expected = bson::doc! {
        "$set": {
            "name": "John Doe",
            "contact.email": "john@example.com"
        },
        "$inc": {
            "age": 1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_deep_nesting() {
    // Test deeply nested structures (Employee -> Company -> Address)
    let result = empty::<Employee>()
        .with_lookup::<EmployeeCompany, _, CompanyName, Company, _>(
            |path| path.field::<CompanyName>(),
            |nested| {
                nested.set::<CompanyName, _>("Tech Corp".to_string());
            },
        )
        .with_lookup::<EmployeeCompany, _, CompanyAddress, Company, _>(
            |path| path.field::<CompanyAddress>(),
            |nested| {
                nested.with_lookup::<CompanyAddress, _, AddressCity, Address, _>(
                    |path| path.field::<AddressCity>(),
                    |deeply_nested| {
                        deeply_nested.set::<AddressCity, _>("Seattle".to_string());
                    },
                );
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "company.name": "Tech Corp",
            "company.address.city": "Seattle"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_multiple_operations_same_nested_context() {
    // Test multiple operations within the same nested context
    let result = empty::<User>()
        .with_lookup::<UserHomeAddress, _, AddressStreet, Address, _>(
            |path| path.field::<AddressStreet>(),
            |nested| {
                nested
                    .set::<AddressStreet, _>("123 Main St".to_string())
                    .set::<AddressCity, _>("Portland".to_string())
                    .set::<AddressZipCode, _>("97201".to_string())
                    .set::<AddressCountry, _>("USA".to_string());
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "home_address.street": "123 Main St",
            "home_address.city": "Portland",
            "home_address.zip_code": "97201",
            "home_address.country": "USA"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_complex_operations() {
    // Test nested fields with complex MongoDB operators
    let result = empty::<User>()
        .with_lookup::<UserHomeAddress, _, AddressStreet, Address, _>(
            |path| path.field::<AddressStreet>(),
            |nested| {
                nested
                    .set::<AddressStreet, _>("456 Oak Ave".to_string())
                    .set::<AddressCity, _>("Denver".to_string());
            },
        )
        .with_lookup::<UserContact, _, ContactEmail, ContactInfo, _>(
            |path| path.field::<ContactEmail>(),
            |nested| {
                nested.set::<ContactEmail, _>("user@denver.com".to_string());
            },
        )
        .build();

    let expected = bson::doc! {
        "$set": {
            "home_address.street": "456 Oak Ave",
            "home_address.city": "Denver",
            "contact.email": "user@denver.com"
        }
    };

    assert_eq!(result, expected);
}

// Tests for with_field function (convenience method using identity)
#[test]
fn test_with_field_simple_update() {
    // Test with_field for simple field update
    let result = empty::<User>()
        .with_field::<UserName, _>(|nested| {
            nested.set::<UserName, _>("Alice Smith".to_string());
        })
        .build();

    let expected = bson::doc! {
        "$set": {
            "name": "Alice Smith"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_field_multiple_operations() {
    // Test with_field with multiple operations on the same field context
    let result = empty::<User>()
        .with_field::<UserAge, _>(|nested| {
            nested
                .set::<UserName, _>("Bob Johnson".to_string())
                .inc::<UserAge, _>(5);
        })
        .build();

    let expected = bson::doc! {
        "$set": {
            "name": "Bob Johnson"
        },
        "$inc": {
            "age": 5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_field_vs_direct_comparison() {
    // Test that with_field produces the same result as direct field operations
    let mut builder1 = empty::<User>();
    let mut builder2 = empty::<User>();

    // Using with_field
    builder1.with_field::<UserName, _>(|nested| {
        nested.set::<UserName, _>("Test User".to_string());
    });

    // Using direct field operation
    builder2.set::<UserName, _>("Test User".to_string());

    assert_eq!(builder1.build(), builder2.build());
}

#[test]
fn test_with_field_combined_operations() {
    // Test with_field combined with other operations
    let result = empty::<User>()
        .with_field::<UserId, _>(|nested| {
            nested.set::<UserId, _>("user-123".to_string());
        })
        .inc::<UserAge, _>(10)
        .with_field::<UserName, _>(|nested| {
            nested.set::<UserName, _>("Combined User".to_string());
        })
        .build();

    let expected = bson::doc! {
        "$set": {
            "id": "user-123",
            "name": "Combined User"
        },
        "$inc": {
            "age": 10
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_mixed_with_field_and_with_lookup() {
    // Test combining with_field and with_lookup in the same query
    let result = empty::<User>()
        .with_field::<UserId, _>(|nested| {
            nested.set::<UserId, _>("mixed-user-456".to_string());
        })
        .with_lookup::<UserContact, _, ContactPhone, ContactInfo, _>(
            |path| path.field::<ContactPhone>(),
            |nested| {
                nested.set::<ContactPhone, _>("+1-555-0123".to_string());
            },
        )
        .with_field::<UserAge, _>(|nested| {
            nested.inc::<UserAge, _>(1);
        })
        .build();

    let expected = bson::doc! {
        "$set": {
            "id": "mixed-user-456",
            "contact.phone": "+1-555-0123"
        },
        "$inc": {
            "age": 1
        }
    };

    assert_eq!(result, expected);
}
