// Minimal test to verify field name conflict resolution works

use tnuctipun::{FieldName, FieldWitnesses, HasField};
// Define two structs with the same field name to test conflict resolution
#[derive(FieldWitnesses)]
struct Product {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub id: String,
}

#[derive(FieldWitnesses)]
struct User {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub id: String,
}

#[test]
fn test_field_name_conflict_resolution() {
    // This test verifies that multiple structs can have the same field names
    // without causing "name defined multiple times" compile errors

    // The field witnesses are now scoped within their respective modules
    // product_fields::Name and user_fields::Name are separate types

    // Test that field names work correctly
    assert_eq!(product_fields::Name::field_name(), "name");
    assert_eq!(product_fields::Id::field_name(), "id");
    assert_eq!(user_fields::Name::field_name(), "name");
    assert_eq!(user_fields::Id::field_name(), "id");

    // Test that the types are actually different
    // This should compile without conflicts because they're in different modules
    let _product_name_type: product_fields::Name = product_fields::Name;
    let _user_name_type: user_fields::Name = user_fields::Name;

    // These are different types even though they have the same field name
    // If there were conflicts, this test wouldn't even compile
}

#[test]
fn test_field_witnesses_still_work() {
    let product = Product {
        name: "Test Product".to_string(),
        id: "prod-123".to_string(),
    };

    let user = User {
        name: "John Doe".to_string(),
        id: "user-456".to_string(),
    };

    // Test HasField trait works with scoped field witnesses
    let product_name: &String = <Product as HasField<product_fields::Name>>::get_field(&product);
    let product_id: &String = <Product as HasField<product_fields::Id>>::get_field(&product);
    let user_name: &String = <User as HasField<user_fields::Name>>::get_field(&user);
    let user_id: &String = <User as HasField<user_fields::Id>>::get_field(&user);

    assert_eq!(product_name, "Test Product");
    assert_eq!(product_id, "prod-123");
    assert_eq!(user_name, "John Doe");
    assert_eq!(user_id, "user-456");
}

#[test]
fn test_different_types_prevent_mixups() {
    // This test verifies that you can't accidentally use the wrong field witness type
    let product = Product {
        name: "Test Product".to_string(),
        id: "prod-123".to_string(),
    };

    // This should compile because we're using the correct field witness type
    let _correct_field = <Product as HasField<product_fields::Name>>::get_field(&product);

    // Note: If we tried to use user_fields::Name with Product, it would be a compile error:
    // let _wrong_field = <Product as HasField<user_fields::Name>>::get_field(&product); // ERROR!

    // This demonstrates that the module scoping prevents not just conflicts but also wrong usage
}
