---
title: Updating Documents
layout: page
nav_exclude: true
---

# Updating Documents

This guide covers how to use Tnuctipun to build type-safe update operations for MongoDB. You'll learn to create update documents from simple field assignments to complex operations involving arrays, nested objects, and conditional updates.

## Table of Contents

- [Basic Update Operations](#basic-update-operations)
- [Field Operations](#field-operations)
- [Array Operations](#array-operations)
  - [Working with Array Fields](#working-with-array-fields)
  - [Array Removal Operations](#array-removal-operations)
  - [Add to Set (Unique Arrays)](#add-to-set-unique-arrays)
  - [Batch Array Operations](#batch-array-operations)
- [Complex Updates](#complex-updates)
- [Conditional Updates](#conditional-updates)

## Basic Update Operations

### Setting Field Values

The most common update operation is setting field values using `set`:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct BasicSetOperations {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
    pub last_login: Option<bson::DateTime>,
    pub login_count: i32,
}

fn basic_set_operations() {
    // Single field update
    let update_doc = updates::empty::<BasicSetOperations>()
        .set::<basicsetoperations_fields::Name, _>("John Doe".to_string())
        .build();
    // Result: { "$set": { "name": "John Doe" } }
    
    // Multiple field updates
    let update_doc = updates::empty::<BasicSetOperations>()
        .set::<basicsetoperations_fields::Name, _>("Jane Smith".to_string())
        .set::<basicsetoperations_fields::Email, _>("jane@example.com".to_string())
        .set::<basicsetoperations_fields::IsActive, _>(true)
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
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UnsetOperations {
    pub name: String,
    pub email: String,
    pub last_login: Option<bson::DateTime>,
}

fn unset_operations() {
    let update_doc = updates::empty::<UnsetOperations>()
        .unset::<unsetoperations_fields::LastLogin>()  // Remove last_login field
        .unset::<unsetoperations_fields::Email>()      // Remove email field
        .build();
    // Result: { "$unset": { "last_login": "", "email": "" } }
}
```

### Mixed Set and Unset

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct MixedSetUnset {
    pub is_active: bool,
    pub email: String,
    pub last_login: Option<bson::DateTime>,
}

fn mixed_set_unset() {
    let update_doc = updates::empty::<MixedSetUnset>()
        .set::<mixedsetunset_fields::IsActive, _>(false)      // Deactivate user
        .unset::<mixedsetunset_fields::LastLogin>()           // Clear login timestamp
        .set::<mixedsetunset_fields::Email, _>("archived@example.com".to_string())  // Archive email
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
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserStatsInc {
    pub user_id: String,
    pub login_count: i32,
    pub points: i64,
    pub balance: f64,
    pub last_active: bson::DateTime,
}

fn increment_operations() {
    // Increment values
    let update_doc = updates::empty::<UserStatsInc>()
        .inc::<userstatsinc_fields::LoginCount, _>(1)     // Increment login count
        .inc::<userstatsinc_fields::Points, _>(100)       // Add 100 points
        .inc::<userstatsinc_fields::Balance, _>(25.50)    // Add to balance
        .build();
    // Result: {
    //   "$inc": {
    //     "login_count": 1,
    //     "points": 100,
    //     "balance": 25.50
    //   }
    // }
    
    // Decrement (negative increment)
    let update_doc = updates::empty::<UserStatsInc>()
        .inc::<userstatsinc_fields::Points, _>(-50)       // Subtract 50 points
        .inc::<userstatsinc_fields::Balance, _>(-10.00)   // Subtract from balance
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
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserStatsMul {
    pub user_id: String,
    pub login_count: i32,
    pub points: i64,
    pub balance: f64,
    pub last_active: bson::DateTime,
}

fn multiplication_operations() {
    let update_doc = updates::empty::<UserStatsMul>()
        .mul::<userstatsmul_fields::Points, _>(2)        // Double the points
        .mul::<userstatsmul_fields::Balance, _>(1.1)     // Apply 10% bonus
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

Use `max` and `min` for comparison-based field updates:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct GameStats {
    pub player_id: String,
    pub high_score: i32,
    pub low_score: i32,
    pub best_time: f64,
    pub worst_time: f64,
}

fn min_max_operations() {
    // Update high score only if new score is higher
    let update_doc = updates::empty::<GameStats>()
        .max::<gamestats_fields::HighScore, _>(1500)     // Only update if 1500 > current high_score
        .min::<gamestats_fields::LowScore, _>(100)       // Only update if 100 < current low_score
        .build();
    // Result: {
    //   "$max": { "high_score": 1500 },
    //   "$min": { "low_score": 100 }
    // }
    
    // Performance tracking with time bounds
    let update_doc = updates::empty::<GameStats>()
        .min::<gamestats_fields::BestTime, _>(45.5)      // Update best time if faster
        .max::<gamestats_fields::WorstTime, _>(120.0)    // Update worst time if slower
        .build();
    // Result: {
    //   "$min": { "best_time": 45.5 },
    //   "$max": { "worst_time": 120.0 }
    // }
}

## Array Operations

### Working with Array Fields

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct ArrayOperations {
    pub user_id: String,
    pub tags: Vec<String>,
    pub favorite_colors: Vec<String>,
    pub login_history: Vec<bson::DateTime>,
    pub scores: Vec<i32>,
}

fn array_operations() {
    // Add items to arrays
    let update_doc = updates::empty::<ArrayOperations>()
        .push::<arrayoperations_fields::Tags, _>("premium".to_string())
        .push::<arrayoperations_fields::FavoriteColors, _>("blue".to_string())
        .build();
    // Result: {
    //   "$push": {
    //     "tags": "premium",
    //     "favorite_colors": "blue"
    //   }
    // }
    
    // Add multiple items to arrays (multiple push calls)
    let update_doc = updates::empty::<ArrayOperations>()
        .push::<arrayoperations_fields::Tags, _>("premium".to_string())
        .push::<arrayoperations_fields::Tags, _>("verified".to_string())
        .push::<arrayoperations_fields::Tags, _>("active".to_string())
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
        .pull::<userprofile_fields::Tags, _>("inactive".to_string())
        .pull::<userprofile_fields::FavoriteColors, _>("red".to_string())
        .build();
    // Result: {
    //   "$pull": {
    //     "tags": "inactive",
    //     "favorite_colors": "red"
    //   }
    // }
    
    // Remove multiple values
    let update_doc = updates::empty::<UserProfile>()
        .pull_all::<userprofile_fields::Tags, _>(vec![
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
        .pop::<userprofile_fields::LoginHistory>(updates::PopStrategy::First)  // Remove first element
        .pop::<userprofile_fields::Scores>(updates::PopStrategy::Last)         // Remove last element
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

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub tags: Vec<String>,
}

fn add_to_set_operations() {
    // Add single unique item
    let update_doc = updates::empty::<UserProfile>()
        .add_to_set::<userprofile_fields::Tags, _>("verified".to_string())
        .build();
    // Result: { "$addToSet": { "tags": "verified" } }
    
    // Add multiple unique items (using multiple calls)
    let update_doc = updates::empty::<UserProfile>()
        .add_to_set::<userprofile_fields::Tags, _>("premium".to_string())
        .add_to_set::<userprofile_fields::Tags, _>("verified".to_string())
        .add_to_set::<userprofile_fields::Tags, _>("active".to_string())
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

### Batch Array Operations

For efficient bulk array operations, use `push_each` and `add_to_set_each` to add multiple values in a single MongoDB operation:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub tags: Vec<String>,
    pub favorite_colors: Vec<String>,
    pub login_history: Vec<bson::DateTime>,
    pub scores: Vec<i32>,
}

fn batch_array_operations() {
    // Add multiple items to arrays efficiently
    let new_tags = vec!["premium".to_string(), "verified".to_string(), "active".to_string()];
    let new_colors = vec!["blue".to_string(), "green".to_string(), "red".to_string()];
    
    let update_doc = updates::empty::<UserProfile>()
        .push_each::<userprofile_fields::Tags, _, _, _>(new_tags)
        .push_each::<userprofile_fields::FavoriteColors, _, _, _>(new_colors)
        .build();
    // Result: {
    //   "$push": {
    //     "tags": { "$each": ["premium", "verified", "active"] },
    //     "favorite_colors": { "$each": ["blue", "green", "red"] }
    //   }
    // }
    
    // Add multiple unique items efficiently
    let unique_tags = vec!["premium".to_string(), "verified".to_string()];
    let unique_scores = vec![100, 200, 300];
    
    let update_doc = updates::empty::<UserProfile>()
        .add_to_set_each::<userprofile_fields::Tags, _, _>(unique_tags)
        .add_to_set_each::<userprofile_fields::Scores, _, _>(unique_scores)
        .build();
    // Result: {
    //   "$addToSet": {
    //     "tags": { "$each": ["premium", "verified"] },
    //     "scores": { "$each": [100, 200, 300] }
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
    let update_doc = updates::empty::<User>()
        .set::<user_fields::Name, _>("John Smith".to_string())
        .set::<user_fields::IsActive, _>(true)
        .set::<user_fields::LastLogin, _>(Some(bson::DateTime::now()))
        .inc::<user_fields::LoginCount, _>(1)
        .build();
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
    pub high_score: i32,
    pub min_balance: f64,
}

fn field_specific_updates() {
    // Apply multiple operations in sequence
    let update_doc = updates::empty::<UserStats>()
        .inc::<userstats_fields::Points, _>(100)         // Add 100 points
        .mul::<userstats_fields::Balance, _>(1.1)        // Apply 10% bonus
        .max::<userstats_fields::HighScore, _>(1000)     // Update high score if 1000 is higher
        .min::<userstats_fields::MinBalance, _>(0.0)     // Ensure balance doesn't go below 0
        .build();
    
    // Result: {
    //   "$inc": { "points": 100 },
    //   "$mul": { "balance": 1.1 },
    //   "$max": { "high_score": 1000 },
    //   "$min": { "min_balance": 0.0 }
    // }
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
| `$max` | `.max()` | Updates a field only if the specified value is greater than the existing value | [Min and Max Operations](#min-and-max-operations) |
| `$min` | `.min()` | Updates a field only if the specified value is less than the existing value | [Min and Max Operations](#min-and-max-operations) |
| **Array Operations** | | | |
| `$push` | `.push()` | Adds an item to an array | [Working with Array Fields](#working-with-array-fields) |
| `$push` (with `$each`) | `.push_each()` | Adds multiple items to an array in a single operation | [Batch Array Operations](#batch-array-operations) |
| `$pull` | `.pull()` | Removes all instances of a value from an array | [Array Removal Operations](#array-removal-operations) |
| `$pullAll` | `.pull_all()` | Removes multiple values from an array | [Array Removal Operations](#array-removal-operations) |
| `$pop` | `.pop()` | Removes the first or last item from an array | [Array Removal Operations](#array-removal-operations) |
| `$addToSet` | `.add_to_set()` | Adds a value to an array only if it doesn't already exist | [Add to Set (Unique Arrays)](#add-to-set-unique-arrays) |
| `$addToSet` (with `$each`) | `.add_to_set_each()` | Adds multiple unique values to an array in a single operation | [Batch Array Operations](#batch-array-operations) |

### Usage Patterns

- **Simple updates**: Use method chaining with `.set()`, `.inc()`, etc., then call `.build()`
- **Numeric operations**: Use `.inc()` for counters, `.mul()` for scaling
- **Array modifications**: Use `.push()` for adding, `.pull()` for removing, `.add_to_set()` for unique items
- **Dynamic updates**: Use mutable update builders when operations depend on runtime parameters

## Next Steps

Now that you've mastered update operations:

- [**Derive Macros**](05-derive-macros.md) - Understand the macro system in detail
- [**Advanced Topics**](06-advanced-topics.md) - Explore complex scenarios and best practices
