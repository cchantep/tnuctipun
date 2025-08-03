// This test verifies that UpdateBuilder::with_lookup rejects paths with incompatible Root type parameters

use nessus::updates::empty;
use nessus::{FieldWitnesses, path::Path};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct User {
    name: String,
    home_address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Company {
    name: String,
    headquarters: Address,
}

fn main() {
    // Create an UpdateBuilder for User
    let mut user_builder = empty::<User>();
    
    // Create a Path with Company as Root type parameter (incompatible with User builder)
    let company_path = Path::<company_fields::Headquarters, Company, Company>::new();
    let address_city_path = company_path.field::<address_fields::City>();
    
    // This should fail to compile because the path's Root type (Company) 
    // doesn't match the builder's type parameter (User)
    user_builder.with_lookup(
        |_path| address_city_path, // This path has Root=Company, but builder expects Root=User
        |mut nested| {
            nested.set::<address_fields::City, _>("San Francisco".to_string());
            nested
        }
    );
}
