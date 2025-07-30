// This example demonstrates the field witness scoping solution

use nessus::{FieldName, FieldWitnesses, HasField};
use serde::{Deserialize, Serialize};

// Two structs with the same field names - this should not conflict
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Product {
    name: String,
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct User {
    name: String,
    id: String,
}

fn main() {
    // Create instances
    let product = Product {
        name: "Laptop".to_string(),
        id: "PROD-123".to_string(),
    };

    let user = User {
        name: "John Doe".to_string(),
        id: "USER-456".to_string(),
    };

    // This works without conflicts
    println!("Product: {:?}", product);
    println!("User: {:?}", user);

    // The field witnesses are generated properly, scoped within modules
    println!("Product name field: {}", product_fields::Name::field_name());
    println!("User name field: {}", user_fields::Name::field_name());

    // Field access also works - using the trait method correctly
    let product_name = <Product as HasField<product_fields::Name>>::get_field(&product);
    let user_name = <User as HasField<user_fields::Name>>::get_field(&user);

    println!("Product name value: {}", product_name);
    println!("User name value: {}", user_name);
}
