// Simple standalone example to prove field name conflicts are resolved
use nessus::{FieldName, FieldWitnesses};

#[derive(FieldWitnesses)]
#[allow(dead_code)] // This struct is used only for field witness generation in this example
struct Product {
    name: String,
    id: String,
}

#[derive(FieldWitnesses)]
#[allow(dead_code)] // This struct is used only for field witness generation in this example
struct User {
    name: String, // Same field name as Product - this would cause conflicts before our fix
    id: String,   // Same field name as Product - this would cause conflicts before our fix
}

fn main() {
    println!("Field name conflict resolution test");

    // Before our fix, this would fail to compile with "name is defined multiple times" errors
    // After our fix, these are in separate modules: product_fields::Name vs user_fields::Name

    println!("Product name field: {}", product_fields::Name::field_name());
    println!("User name field: {}", user_fields::Name::field_name());

    println!("SUCCESS: No field name conflicts!");
}
