---
title: Updating Documents
layout: page
nav_order: 4
parent: User Guide
---

# Updating Documents

This guide covers how to use Tnuctipun to build type-safe update operations for MongoDB. You'll learn to create update documents from simple field assignments to complex operations involving arrays, nested objects, and conditional updates.

## Table of Contents

- [Basic Update Operations](#basic-update-operations)
- [Field Operations](#field-operations)
- [Array Operations](#array-operations)
- [Complex Updates](#complex-updates)
- [Conditional Updates](#conditional-updates)
- [Integration with MongoDB Updates](#integration-with-mongodb-updates)
- [Update Patterns](#update-patterns)

## Basic Update Operations

### Setting Field Values

The most common update operation is setting field values using `set`:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

fn basic_set_operations() {
    // Single field update
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Name, _>("John Doe".to_string())
        .build();
    // Result: { "$set": { "name": "John Doe" } }
    
    // Multiple field updates
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Name, _>("Jane Smith".to_string())
        .set::<user_fields::Email, _>("jane@example.com".to_string())
        .set::<user_fields::IsActive, _>(true)
        .build();
    // Result: {
    //   "$set": {
    //     "name": "Jane Smith",
    //     "email": "jane@example.com",
    //     "is_active": true
    //   }
    // }
}
```

### Unsetting Fields

Use `unset` to remove fields from documents:

```rust
fn unset_operations() {
    let update_doc = updates::empty::<User>()
        .unset::<user_fields::LastLogin>()  // Remove last_login field
        .unset::<user_fields::Email>()      // Remove email field
        .build();
    // Result: { "$unset": { "last_login": "", "email": "" } }
}
```

### Mixed Set and Unset

```rust
fn mixed_set_unset() {
    let update_doc = updates::empty::<User>()
        .set::<user_fields::IsActive, _>(false)      // Deactivate user
        .unset::<user_fields::LastLogin>()           // Clear login timestamp
        .set::<user_fields::Email, _>("archived@example.com".to_string())  // Archive email
        .build();
    // Result: {
    //   "$set": { "is_active": false, "email": "archived@example.com" },
    //   "$unset": { "last_login": "" }
    // }
}
```

## Field Operations

### Increment and Decrement

Use `inc` for numeric field operations:

```rust
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserStats {
    pub user_id: String,
    pub login_count: i32,
    pub points: i64,
    pub balance: f64,
    pub last_active: chrono::DateTime<chrono::Utc>,
}

fn increment_operations() {
    // Increment values
    let update_doc = updates::empty::<UserStats>()
        .inc::<user_stats_fields::LoginCount, _>(1)     // Increment login count
        .inc::<user_stats_fields::Points, _>(100)       // Add 100 points
        .inc::<user_stats_fields::Balance, _>(25.50)    // Add to balance
        .build();
    // Result: {
    //   "$inc": {
    //     "login_count": 1,
    //     "points": 100,
    //     "balance": 25.50
    //   }
    // }
    
    // Decrement (negative increment)
    let update_doc = updates::empty::<UserStats>()
        .inc::<user_stats_fields::Points, _>(-50)       // Subtract 50 points
        .inc::<user_stats_fields::Balance, _>(-10.00)   // Subtract from balance
        .build();
    // Result: {
    //   "$inc": {
    //     "points": -50,
    //     "balance": -10.00
    //   }
    // }
}
```

### Multiplication

Use `mul` to multiply field values:

```rust
fn multiplication_operations() {
    let update_doc = updates::empty::<UserStats>()
        .mul::<user_stats_fields::Points, _>(2)        // Double the points
        .mul::<user_stats_fields::Balance, _>(1.1)     // Apply 10% bonus
        .build();
    // Result: {
    //   "$mul": {
    //     "points": 2,
    //     "balance": 1.1
    //   }
    // }
}
```

### Min and Max Operations

Use `min` and `max` to set fields to minimum or maximum values:

```rust
fn min_max_operations() {
    let update_doc = updates::empty::<UserStats>()
        .min::<user_stats_fields::Points, _>(0)        // Ensure points never go below 0
        .max::<user_stats_fields::LoginCount, _>(1000) // Cap login count at 1000
        .build();
    // Result: {
    //   "$min": { "points": 0 },
    //   "$max": { "login_count": 1000 }
    // }
}
```

## Array Operations

### Working with Array Fields

```rust
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub user_id: String,
    pub tags: Vec<String>,
    pub favorite_colors: Vec<String>,
    pub login_history: Vec<bson::DateTime>,
    pub scores: Vec<i32>,
}

fn array_operations() {
    // Add items to arrays
    let update_doc = updates::empty::<UserProfile>()
        .push::<user_profile_fields::Tags, _>("premium".to_string())
        .push::<user_profile_fields::FavoriteColors, _>("blue".to_string())
        .build();
    // Result: {
    //   "$push": {
    //     "tags": "premium",
    //     "favorite_colors": "blue"
    //   }
    // }
    
    // Add multiple items to arrays (multiple push calls)
    let update_doc = updates::empty::<UserProfile>()
        .push::<user_profile_fields::Tags, _>("premium".to_string())
        .push::<user_profile_fields::Tags, _>("verified".to_string())
        .push::<user_profile_fields::Tags, _>("active".to_string())
        .build();
    // Result: {
    //   "$push": {
    //     "tags": "premium",  // Note: Multiple $push operations
    //     "tags": "verified", 
    //     "tags": "active"
    //   }
    // }
}
```

### Array Removal Operations

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub tags: Vec<String>,
    pub favorite_colors: Vec<String>,
    pub login_history: Vec<bson::DateTime>,
    pub scores: Vec<i32>,
}

fn array_removal() {
    // Remove specific values from arrays
    let update_doc = updates::empty::<UserProfile>()
        .pull::<user_profile_fields::Tags, _>("inactive".to_string())
        .pull::<user_profile_fields::FavoriteColors, _>("red".to_string())
        .build();
    // Result: {
    //   "$pull": {
    //     "tags": "inactive",
    //     "favorite_colors": "red"
    //   }
    // }
    
    // Remove multiple values
    let update_doc = updates::empty::<UserProfile>()
        .pull_all::<user_profile_fields::Tags, _>(vec![
            "inactive".to_string(),
            "suspended".to_string()
        ])
        .build();
    // Result: {
    //   "$pullAll": {
    //     "tags": ["inactive", "suspended"]
    //   }
    // }
    
    // Remove first or last element
    let update_doc = updates::empty::<UserProfile>()
        .pop::<user_profile_fields::LoginHistory, _>(-1)  // Remove first element
        .pop::<user_profile_fields::Scores, _>(1)         // Remove last element
        .build();
    // Result: {
    //   "$pop": {
    //     "login_history": -1,
    //     "scores": 1
    //   }
    // }
}
```

### Add to Set (Unique Arrays)

Use `add_to_set` to add items only if they don't already exist:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub tags: Vec<String>,
}

fn add_to_set_operations() {
    // Add single unique item
    let update_doc = updates::empty::<UserProfile>()
        .add_to_set::<user_profile_fields::Tags, _>("verified".to_string())
        .build();
    // Result: { "$addToSet": { "tags": "verified" } }
    
    // Add multiple unique items (using multiple calls)
    let update_doc = updates::empty::<UserProfile>()
        .add_to_set::<user_profile_fields::Tags, _>("premium".to_string())
        .add_to_set::<user_profile_fields::Tags, _>("verified".to_string())
        .add_to_set::<user_profile_fields::Tags, _>("active".to_string())
        .build();
    // Result: {
    //   "$addToSet": {
    //     "tags": "premium",  // Note: Multiple addToSet operations
    //     "tags": "verified", 
    //     "tags": "active"
    //   }
    // }
}
```

## Complex Updates

### Combined Update Operations

For complex update scenarios, combine multiple operations:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub is_active: bool,
    pub last_login: Option<bson::DateTime>,
    pub login_count: i32,
}

fn complex_nested_updates() {
    let mut main_update = updates::empty::<User>();
    
    // Update basic fields
    main_update.set::<user_fields::Name, _>("John Smith".to_string());
    main_update.set::<user_fields::IsActive, _>(true);
    main_update.set::<user_fields::LastLogin, _>(Some(bson::DateTime::now()));
    main_update.inc::<user_fields::LoginCount, _>(1);
    
    let update_doc = main_update.build();
}
```

### Field-Specific Complex Updates

Apply multiple operations to build complex updates:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserStats {
    pub points: i64,
    pub balance: f64,
}

fn field_specific_updates() {
    let mut update_builder = updates::empty::<UserStats>();
    
    // Apply multiple operations in sequence
    update_builder
        .inc::<user_stats_fields::Points, _>(100)      // Add 100 points
        .mul::<user_stats_fields::Balance, _>(1.1)     // Apply 10% bonus
        .set::<user_stats_fields::Points, _>(1000);    // Cap points at 1000
    
    let update_doc = update_builder.build();
}
```

## Conditional Updates

### Building Updates Based on Runtime Conditions

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub is_active: bool,
    pub last_login: Option<bson::DateTime>,
}

fn conditional_update_building(
    new_name: Option<String>,
    increment_age: bool,
    deactivate: bool,
    reset_login: bool
) {
    let mut update_builder = updates::empty::<User>();
    
    // Conditionally set name
    if let Some(name) = new_name {
        update_builder.set::<user_fields::Name, _>(name);
    }
    
    // Conditionally increment age
    if increment_age {
        update_builder.inc::<user_fields::Age, _>(1);
    }
    
    // Conditionally deactivate user
    if deactivate {
        update_builder.set::<user_fields::IsActive, _>(false);
        update_builder.unset::<user_fields::LastLogin>();
    }
    
    // Conditionally reset login
    if reset_login {
        update_builder.unset::<user_fields::LastLogin>();
    }
    
    let update_doc = update_builder.build();
}
```

### Profile Update Example

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub last_login: Option<bson::DateTime>,
}

fn update_user_profile(
    user_update: UserProfileUpdate,
) -> bson::Document {
    let mut update_builder = updates::empty::<User>();
    
    // Update provided fields
    if let Some(name) = user_update.name {
        update_builder.set::<user_fields::Name, _>(name);
    }
    
    if let Some(email) = user_update.email {
        update_builder.set::<user_fields::Email, _>(email);
    }
    
    if let Some(age) = user_update.age {
        update_builder.set::<user_fields::Age, _>(age);
    }
    
    // Always update the last modified timestamp
    update_builder.set::<user_fields::LastLogin, _>(
        Some(bson::DateTime::now())
    );
    
    update_builder.build()
}

#[derive(Debug)]
struct UserProfileUpdate {
    name: Option<String>,
    email: Option<String>,
    age: Option<i32>,
}
```

## Integration with MongoDB Updates

### Basic Update Operations

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};
use mongodb::{Collection, options::UpdateOptions};
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub last_login: Option<bson::DateTime>,
}

async fn update_single_user(
    collection: &Collection<User>,
    user_id: &str,
    new_name: String,
    increment_age: bool
) -> mongodb::error::Result<mongodb::results::UpdateResult> {
    
    // Build filter to find the user
    let filter = doc! { "_id": user_id };
    
    // Build type-safe update
    let mut update_builder = updates::empty::<User>();

    update_builder.set::<user_fields::Name, _>(new_name);
    
    if increment_age {
        update_builder.inc::<user_fields::Age, _>(1);
    }
    
    update_builder.set::<user_fields::LastLogin, _>(
        Some(bson::DateTime::now())
    );
    
    let update_doc = update_builder.build();
    
    // Execute update
    let result = collection.update_one(filter, update_doc, None).await?;

    Ok(result)
}
```

### Bulk Updates

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub is_active: bool,
    pub last_login: Option<bson::DateTime>,
    pub login_count: i32,
}

async fn bulk_activate_users(
    collection: &Collection<User>,
    user_ids: Vec<String>
) -> mongodb::error::Result<mongodb::results::UpdateResult> {
    
    // Filter for multiple users
    let filter = doc! { "_id": { "$in": user_ids } };
    
    // Build activation update
    let update_doc = updates::empty::<User>()
        .set::<user_fields::IsActive, _>(true)
        .set::<user_fields::LastLogin, _>(Some(bson::DateTime::now()))
        .inc::<user_fields::LoginCount, _>(1)  // Assuming this field exists
        .build();
    
    // Execute bulk update
    let result = collection.update_many(filter, update_doc, None).await?;

    Ok(result)
}
```

### Upsert Operations

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};
use mongodb::{Collection, options::UpdateOptions};
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserStats {
    pub user_id: String,
    pub points: i64,
    pub last_active: bson::DateTime,
}

async fn upsert_user_stats(
    collection: &Collection<UserStats>,
    user_id: String,
    points_to_add: i64
) -> mongodb::error::Result<mongodb::results::UpdateResult> {
    
    let filter = doc! { "user_id": &user_id };
    
    // Build upsert update
    let update_doc = updates::empty::<UserStats>()
        .set::<user_stats_fields::UserId, _>(user_id)            // Set on insert
        .inc::<user_stats_fields::Points, _>(points_to_add)      // Increment existing
        .set::<user_stats_fields::LastActive, _>(bson::DateTime::now())  // Always update
        .build();
    
    // Configure upsert options
    let options = UpdateOptions::builder()
        .upsert(true)
        .build();
    
    let result = collection.update_one(filter, update_doc, options).await?;
    
    Ok(result)
}
```

### Replace vs Update

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
    pub last_login: Option<bson::DateTime>,
}

async fn replace_vs_update_example(collection: &Collection<User>) {
    let user_id = "user123";
    
    // Update: Modify specific fields (recommended)
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Name, _>("New Name".to_string())
        .inc::<user_fields::Age, _>(1)
        .build();
    
    let filter = doc! { "_id": user_id };
    let _update_result = collection.update_one(filter.clone(), update_doc, None).await;
    
    // Replace: Replace entire document (use with caution)
    let new_user = User {
        name: "Replaced User".to_string(),
        age: 30,
        email: "new@example.com".to_string(),
        is_active: true,
        last_login: Some(bson::DateTime::now()),
    };
    
    let _replace_result = collection.replace_one(filter, new_user, None).await;
}
```

## Update Patterns

### Atomic Counters

```rust
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserStats {
    pub user_id: String,
    pub login_count: i32,
    pub points: i64,
    pub last_active: bson::DateTime,
}

async fn atomic_counter_pattern(
    collection: &Collection<UserStats>,
    user_id: &str,
    points: i64
) -> mongodb::error::Result<Option<UserStats>> {
    
    let filter = doc! { "user_id": user_id };
    
    let update_doc = updates::empty::<UserStats>()
        .inc::<user_stats_fields::Points, _>(points)
        .inc::<user_stats_fields::LoginCount, _>(1)
        .set::<user_stats_fields::LastActive, _>(bson::DateTime::now())
        .build();
    
    // Use find_one_and_update for atomic read-modify-write
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();
    
    let result = collection
        .find_one_and_update(filter, update_doc, options)
        .await?;
    
    Ok(result)
}
```

### Optimistic Updates

```rust
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub version: i32,
}

async fn optimistic_update_pattern(
    collection: &Collection<User>,
    user_id: &str,
    expected_version: i32,
    new_name: String
) -> mongodb::error::Result<bool> {
    
    // Include version in filter for optimistic locking
    let filter = doc! { 
        "_id": user_id,
        "version": expected_version
    };
    
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Name, _>(new_name)
        .inc::<user_fields::Version, _>(1)  // Increment version
        .build();
    
    let result = collection.update_one(filter, update_doc, None).await?;
    
    // Return true if document was actually updated
    Ok(result.modified_count > 0)
}
```

### Conditional Set Pattern

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub email: String,
    pub last_login: Option<bson::DateTime>,
}

async fn conditional_set_pattern(
    collection: &Collection<User>,
    user_id: &str,
    new_email: String
) -> mongodb::error::Result<mongodb::results::UpdateResult> {
    
    // Only update if email is not already set
    let filter = doc! { 
        "_id": user_id,
        "email": { "$exists": false }
    };
    
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Email, _>(new_email)
        .set::<user_fields::LastLogin, _>(Some(bson::DateTime::now()))
        .build();
    
    let result = collection.update_one(filter, update_doc, None).await?;

    Ok(result)
}
```

## Update Operators Reference

This table provides a quick reference for all available update operators and methods in Tnuctipun:

| Operator | Method | Description | Section |
|----------|--------|-------------|---------|
| **Field Setting** | | | |
| `$set` | `.set()` | Sets the value of a field in a document | [Setting Field Values](#setting-field-values) |
| `$unset` | `.unset()` | Removes the specified field from a document | [Unsetting Fields](#unsetting-fields) |
| **Numeric Operations** | | | |
| `$inc` | `.inc()` | Increments the value of a field by a specified amount | [Increment and Decrement](#increment-and-decrement) |
| `$mul` | `.mul()` | Multiplies the value of a field by a specified number | [Multiplication](#multiplication) |
| `$min` | `.min()` | Only updates if the specified value is less than the existing value | [Min and Max Operations](#min-and-max-operations) |
| `$max` | `.max()` | Only updates if the specified value is greater than the existing value | [Min and Max Operations](#min-and-max-operations) |
| **Array Operations** | | | |
| `$push` | `.push()` | Adds an item to an array | [Working with Array Fields](#working-with-array-fields) |
| `$push` | `.push_each()` | Adds multiple items to an array | [Working with Array Fields](#working-with-array-fields) |
| `$pull` | `.pull()` | Removes all instances of a value from an array | [Array Removal Operations](#array-removal-operations) |
| `$pullAll` | `.pull_all()` | Removes multiple values from an array | [Array Removal Operations](#array-removal-operations) |
| `$pop` | `.pop()` | Removes the first or last item from an array | [Array Removal Operations](#array-removal-operations) |
| `$addToSet` | `.add_to_set()` | Adds a value to an array only if it doesn't already exist | [Add to Set (Unique Arrays)](#add-to-set-unique-arrays) |
| `$addToSet` | `.add_to_set_each()` | Adds multiple unique values to an array | [Add to Set (Unique Arrays)](#add-to-set-unique-arrays) |
| **Building** | | | |
| - | `.build()` | Builds the final update document | [Setting Field Values](#setting-field-values) |
| **Advanced** | | | |
| - | `.with_lookup()` | Creates nested update builders for complex update logic | [Nested Field Updates with `with_lookup`](#nested-field-updates-with-with_lookup) |
| - | `.with_fields()` | Applies multiple operations to a specific field | [Field-Specific Complex Updates with `with_fields`](#field-specific-complex-updates-with-with_fields) |

### Usage Patterns

- **Simple updates**: Use method chaining with `.set()`, `.inc()`, etc., then call `.build()`
- **Numeric operations**: Use `.inc()` for counters, `.mul()` for scaling, `.min()`/`.max()` for bounds
- **Array modifications**: Use `.push()` for adding, `.pull()` for removing, `.add_to_set()` for unique items
- **Complex nested logic**: Use `.with_lookup()` for nested update expressions
- **Field-specific operations**: Use `.with_fields()` to apply multiple operators to one field
- **Dynamic updates**: Use mutable update builders when operations depend on runtime parameters

## Best Practices

1. **Use Appropriate Operations**: Choose the right update operator (`$set`, `$inc`, `$push`, etc.)
2. **Atomic Updates**: Prefer single update operations over multiple separate updates
3. **Type Safety**: Leverage compile-time validation for field names and types
4. **Conditional Building**: Build updates dynamically based on runtime conditions
5. **Version Management**: Consider implementing versioning for optimistic locking
6. **Error Handling**: Always handle potential update errors appropriately
7. **Reference Table**: Use the [Update Operators Reference](#update-operators-reference) above for quick lookup

## Next Steps

Now that you've mastered update operations:

- [**Derive Macros**](05-derive-macros.md) - Understand the macro system in detail
- [**Advanced Topics**](06-advanced-topics.md) - Explore complex scenarios and best practices
