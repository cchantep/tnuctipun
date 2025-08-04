//! Shared test fixtures for projection tests

use serde::{Deserialize, Serialize};
use tnuctipun::FieldWitnesses;

#[derive(Deserialize, Serialize, FieldWitnesses)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: u32,
}

#[derive(Deserialize, Serialize, FieldWitnesses)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: String,
}

#[derive(Deserialize, Serialize, FieldWitnesses)]
pub struct Profile {
    pub bio: String,
    pub location: String,
    pub avatar_url: String,
}
