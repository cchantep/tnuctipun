use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct ExprUser {
    pub name: String,
    pub age: i32,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
pub struct ExprAddress {
    pub city: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
pub struct ExprUserWithAddress {
    pub name: String,
    pub address: ExprAddress,
}
