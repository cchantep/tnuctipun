use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct ExprUser {
    pub name: String,
    pub age: i32,
    pub score: i32,
}
