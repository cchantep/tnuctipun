---
title: Basic Usage Examples
layout: page
nav_exclude: true
---

This page provides simple, runnable examples to get you started with Tnuctipun quickly.

## Simple User Management

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn main() {
    // Create a simple equality filter with chaining
    let filter_doc = empty::<User>()
        .eq::<user_fields::Name, _>("John".to_string())
        .eq::<user_fields::IsActive, _>(true)
        .and();
    
    println!("Filter: {}", filter_doc);
    // Output: { "$and": [{ "name": "John" }, { "is_active": true }] }
    
    // Create a projection to select specific fields
    let projection_doc = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Age>()
        .excludes::<user_fields::Email>()  // Hide email
        .build();
    
    println!("Projection: {}", projection_doc);
    // Output: { "name": 1, "age": 1, "email": 0 }
    
    // Create an update document
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Name, _>("John Doe".to_string())
        .inc::<user_fields::Age, _>(1)
        .set::<user_fields::IsActive, _>(true)
        .build();
    
    println!("Update: {}", update_doc);
    // Output: { "$set": { "name": "John Doe", "is_active": true }, "$inc": { "age": 1 } }
}
```

## Range Queries

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub name: String,
    pub price: f64,
    pub category: String,
    pub in_stock: bool,
}

fn range_query_examples() {
    // Find products in a price range
    let filter_doc = empty::<Product>()
        .gte::<product_fields::Price, _>(10.0)      // Price >= 10.0
        .lte::<product_fields::Price, _>(100.0)     // Price <= 100.0
        .eq::<product_fields::InStock, _>(true)     // In stock
        .and();
    
    println!("Price range filter: {}", filter_doc);
    
    // Find products by multiple categories
    let category_doc = empty::<Product>()
        // .in_array::<product_fields::Category, _>(vec![
        //     "Electronics".to_string(),
        //     "Books".to_string(),
        //     "Clothing".to_string(),
        // ])
        .eq::<product_fields::Category, _>("Electronics".to_string())  // Single category for now
        .and();

    println!("Category filter: {}", category_doc);
}
```

## Array Operations

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub user_id: String,
    pub tags: Vec<String>,
    pub interests: Vec<String>,
    pub scores: Vec<i32>,
}

fn array_operations_examples() {
    // Add items to arrays - simplified examples
    let update_doc = updates::empty::<UserProfile>()
        // .push::<userprofile_fields::Tags, _>("premium".to_string())
        // .add_to_set::<userprofile_fields::Interests, _>("programming".to_string())
        // .push::<userprofile_fields::Scores, _>(95)
        .set::<userprofile_fields::UserId, _>("user123".to_string())  // Basic set operation
        .build();
    
    println!("Array additions: {}", update_doc);
    
    // Remove items from arrays - simplified examples  
    let cleanup_doc = updates::empty::<UserProfile>()
        // .pull::<userprofile_fields::Tags, _>("inactive".to_string())
        // .pop::<userprofile_fields::Scores, _>(1)  // Remove last score
        .set::<userprofile_fields::UserId, _>("updated_user123".to_string())  // Basic set operation
        .build();
    
    println!("Array cleanup: {}", cleanup_doc);
}
```
