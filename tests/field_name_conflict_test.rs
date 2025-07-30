use nessus::{FieldName, FieldWitnesses, HasField};
use serde::{Deserialize, Serialize};

/// This test verifies that multiple structs with the same field names
/// can coexist without naming conflicts.
///
/// Before the fix: This would fail to compile with errors like:
/// "error[E0428]: the name `Name` is defined multiple times"
///
/// After the fix: Field witnesses are scoped in separate modules,
/// so `product_fields::Name` and `user_fields::Name` are different types.

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Product {
    name: String,
    id: String,
    price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct User {
    name: String, // Same field name as Product::name
    id: String,   // Same field name as Product::id
    age: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Company {
    name: String, // Same field name as Product::Name and User::Name
    id: String,   // Same field name as Product::Id and User::Id
    revenue: f64,
}

#[test]
fn test_no_field_name_conflicts() {
    // Create instances of all structs
    let product = Product {
        name: "Laptop".to_string(),
        id: "prod-123".to_string(),
        price: 999.99,
    };

    let user = User {
        name: "John Doe".to_string(),
        id: "user-456".to_string(),
        age: 30,
    };

    let company = Company {
        name: "Tech Corp".to_string(),
        id: "comp-789".to_string(),
        revenue: 1_000_000.0,
    };

    // Verify that field witnesses are properly scoped
    assert_eq!(product_fields::Name::field_name(), "name");
    assert_eq!(user_fields::Name::field_name(), "name");
    assert_eq!(company_fields::Name::field_name(), "name");

    assert_eq!(product_fields::Id::field_name(), "id");
    assert_eq!(user_fields::Id::field_name(), "id");
    assert_eq!(company_fields::Id::field_name(), "id");

    // Verify that HasField works correctly with scoped field witnesses
    let product_name = <Product as HasField<product_fields::Name>>::get_field(&product);
    let user_name = <User as HasField<user_fields::Name>>::get_field(&user);
    let company_name = <Company as HasField<company_fields::Name>>::get_field(&company);

    assert_eq!(product_name, "Laptop");
    assert_eq!(user_name, "John Doe");
    assert_eq!(company_name, "Tech Corp");

    // Verify that different field witness types are actually different
    // This ensures the types are properly scoped and not conflicting
    let product_id = <Product as HasField<product_fields::Id>>::get_field(&product);
    let user_id = <User as HasField<user_fields::Id>>::get_field(&user);
    let company_id = <Company as HasField<company_fields::Id>>::get_field(&company);

    assert_eq!(product_id, "prod-123");
    assert_eq!(user_id, "user-456");
    assert_eq!(company_id, "comp-789");
}

#[test]
fn test_field_witnesses_are_different_types() {
    // This test verifies that field witnesses with the same name from different
    // structs are actually different types. This would fail to compile if they
    // were the same type.

    use std::any::TypeId;

    // These should all be different TypeIds, proving they're different types
    let product_name_type = TypeId::of::<product_fields::Name>();
    let user_name_type = TypeId::of::<user_fields::Name>();
    let company_name_type = TypeId::of::<company_fields::Name>();

    assert_ne!(product_name_type, user_name_type);
    assert_ne!(product_name_type, company_name_type);
    assert_ne!(user_name_type, company_name_type);

    let product_id_type = TypeId::of::<product_fields::Id>();
    let user_id_type = TypeId::of::<user_fields::Id>();
    let company_id_type = TypeId::of::<company_fields::Id>();

    assert_ne!(product_id_type, user_id_type);
    assert_ne!(product_id_type, company_id_type);
    assert_ne!(user_id_type, company_id_type);
}

#[test]
fn test_unique_fields_still_work() {
    // Verify that fields unique to each struct work correctly
    let product = Product {
        name: "Smartphone".to_string(),
        id: "prod-999".to_string(),
        price: 599.99,
    };

    let user = User {
        name: "Jane Smith".to_string(),
        id: "user-888".to_string(),
        age: 25,
    };

    let company = Company {
        name: "StartupCo".to_string(),
        id: "comp-777".to_string(),
        revenue: 500_000.0,
    };

    // Test unique fields for each struct
    assert_eq!(product_fields::Price::field_name(), "price");
    assert_eq!(user_fields::Age::field_name(), "age");
    assert_eq!(company_fields::Revenue::field_name(), "revenue");

    let product_price = <Product as HasField<product_fields::Price>>::get_field(&product);
    let user_age = <User as HasField<user_fields::Age>>::get_field(&user);
    let company_revenue = <Company as HasField<company_fields::Revenue>>::get_field(&company);

    assert_eq!(*product_price, 599.99);
    assert_eq!(*user_age, 25);
    assert_eq!(*company_revenue, 500_000.0);
}
