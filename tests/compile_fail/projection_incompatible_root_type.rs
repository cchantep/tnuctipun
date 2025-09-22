// This test verifies that BasicProjectionBuilder::with_lookup rejects paths with incompatible Root type parameters

use tnuctipun::projection::empty;
use tnuctipun::path::Path;
use tnuctipun::{FieldWitnesses};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Address {
    pub street: String,
    pub city: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct User {
    pub name: String,
    pub home_address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Company {
    pub name: String,
    pub headquarters: Address,
}

fn main() {
    // Create a BasicProjectionBuilder for User
    let mut user_builder = empty::<User>();
    
    // Create a Path with Company as Root type parameter (incompatible with User builder)
    let company_path = Path::<company_fields::Headquarters, Company, Company>::new();
    let address_city_path = company_path.field::<address_fields::City>();
    
    // This should fail to compile because the path's Root type (Company) 
    // doesn't match the builder's type parameter (User)
    user_builder.with_lookup(
        |_path| address_city_path, // This path has Root=Company, but builder expects Root=User
        |nested| {
            nested.includes::<address_fields::City>();
        }
    );
}
