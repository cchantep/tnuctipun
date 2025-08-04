//! Shared test fixtures, mock types, and field implementations for filters tests

use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable};

// Define test structs
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub categories: Vec<String>,
    pub brand: String,
}

// Define nested structs for testing with_lookup function
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct ContactInfo {
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub home_address: Address,
    pub work_address: Address,
    pub contact: ContactInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct Company {
    pub name: String,
    pub address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub company: Company,
}
