---
title: Updating Documents
layout: page
nav_exclude: true
---

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
- [Handling Optional Fields](#handling-optional-fields)

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
        .max::<gamestats_fields::HighScore, _>(1500) // Only update if 1500 > current high_score
        .min::<gamestats_fields::LowScore, _>(100)   // Only update if 100 < current low_score
        .build();
    // Result: {
    //   "$max": { "high_score": 1500 },
    //   "$min": { "low_score": 100 }
    // }
    
    // Performance tracking with time bounds
    let update_doc = updates::empty::<GameStats>()
        .min::<gamestats_fields::BestTime, _>(45.5)   // Update best time if faster
        .max::<gamestats_fields::WorstTime, _>(120.0) // Update worst time if slower
        .build();
    // Result: {
    //   "$min": { "best_time": 45.5 },
    //   "$max": { "worst_time": 120.0 }
    // }
}
```

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

## Handling Optional Fields

When building update documents dynamically, you often need to handle optional values where some fields should only be updated if certain values are provided. Tnuctipun provides the `if_some` method to elegantly handle `Option<T>` values without explicit conditional logic.

### Basic Optional Field Updates

Tnuctipun supports two approaches for handling optional fields:

1. **Direct assignment**: You can directly set `Option<T>` values using the regular `set` method
2. **Conditional updates**: Use the `if_some` method to conditionally apply operations based on whether an `Option<T>` contains a value

#### Direct Option Assignment

You can directly set `Option` fields using the standard `set` method, regardless of whether the value is `Some` or `None`:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub name: String,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
}

fn direct_option_assignment() -> bson::Document {
    updates::empty::<UserProfile>()
        .set::<userprofile_fields::Name, _>("John Doe".to_string())
        .set::<userprofile_fields::Email, _>(Some("john@example.com".to_string())) // Set Some value
        .set::<userprofile_fields::Bio, _>(Some("Software developer".to_string()))  // Set Some value
        .set::<userprofile_fields::Website, _>(None::<String>)                       // Set None value
        .build()
    // Result: {
    //   "$set": {
    //     "name": "John Doe",
    //     "email": "john@example.com",
    //     "bio": "Software developer",
    //     "website": null
    //   }
    // }
}

// Working with variables containing Option values
fn direct_option_from_variables(
    maybe_email: Option<String>,
    maybe_bio: Option<String>,
) -> bson::Document {
    updates::empty::<UserProfile>()
        .set::<userprofile_fields::Name, _>("Jane Smith".to_string())
        .set::<userprofile_fields::Email, _>(maybe_email)    // Directly pass Option value
        .set::<userprofile_fields::Bio, _>(maybe_bio)        // Directly pass Option value
        .build()
    // Result depends on the Option values:
    // - If maybe_email is Some("jane@example.com"), email field will be set to "jane@example.com"
    // - If maybe_email is None, email field will be set to null
}
```

#### Conditional Updates with if_some

The `if_some` method allows you to conditionally apply update operations based on whether an `Option<T>` contains a value:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub name: String,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub age: i32,
    pub score: i32,
}

fn update_user_with_optional_fields(
    name: String,
    maybe_email: Option<String>,
    maybe_bio: Option<String>,
    maybe_website: Option<String>,
    maybe_score_bonus: Option<i32>,
) -> bson::Document {
    updates::empty::<UserProfile>()
        .set::<userprofile_fields::Name, _>(name)
        .if_some(maybe_email, |builder, email| {
            builder.set::<userprofile_fields::Email, _>(email)
        })
        .if_some(maybe_bio, |builder, bio| {
            builder.set::<userprofile_fields::Bio, _>(bio)
        })
        .if_some(maybe_website, |builder, website| {
            builder.set::<userprofile_fields::Website, _>(website)
        })
        .if_some(maybe_score_bonus, |builder, bonus| {
            builder.inc::<userprofile_fields::Score, _>(bonus)
        })
        .build()
}

// Usage examples:
fn examples() {
    // Update with all optional fields present
    let update_all = update_user_with_optional_fields(
        "John Doe".to_string(),
        Some("john@example.com".to_string()),
        Some("Software developer".to_string()),
        Some("https://johndoe.dev".to_string()),
        Some(100),
    );
    // Result: {
    //   "$set": {
    //     "name": "John Doe",
    //     "email": "john@example.com",
    //     "bio": "Software developer",
    //     "website": "https://johndoe.dev"
    //   },
    //   "$inc": { "score": 100 }
    // }

    // Update with some optional fields missing
    let update_partial = update_user_with_optional_fields(
        "Jane Smith".to_string(),
        Some("jane@example.com".to_string()),
        None, // No bio update
        None, // No website update
        Some(50),
    );
    // Result: {
    //   "$set": {
    //     "name": "Jane Smith",
    //     "email": "jane@example.com"
    //   },
    //   "$inc": { "score": 50 }
    // }

    // Update with no optional fields
    let update_minimal = update_user_with_optional_fields(
        "Bob Wilson".to_string(),
        None,
        None,
        None,
        None,
    );
    // Result: {
    //   "$set": { "name": "Bob Wilson" }
    // }
}
```

#### Choosing Between Direct Assignment and if_some

Both approaches have their use cases:

**Use direct assignment when:**
- You want to explicitly set a field to `null` (None) in the database
- You have an `Option<T>` value and want to set the field regardless of whether it's `Some` or `None`
- You're building simple updates where the field should always be updated

**Use `if_some` when:**
- You only want to update the field if a value is present
- You want to skip the update operation entirely when the value is `None`
- You need to perform multiple related operations when a value is present

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserSettings {
    pub theme: Option<String>,
    pub notifications: Option<bool>,
    pub last_updated: String,
}

// Example showing the difference
fn compare_approaches(
    maybe_theme: Option<String>,
    maybe_notifications: Option<bool>,
) {
    // Direct assignment - always updates the fields (even to null)
    let direct_update = updates::empty::<UserSettings>()
        .set::<usersettings_fields::Theme, _>(maybe_theme.clone())
        .set::<usersettings_fields::Notifications, _>(maybe_notifications)
        .set::<usersettings_fields::LastUpdated, _>("2025-08-06".to_string())
        .build();
    // Result: {
    //   "$set": {
    //     "theme": null,           // Set to null if maybe_theme was None
    //     "notifications": null,   // Set to null if maybe_notifications was None
    //     "last_updated": "2025-08-06"
    //   }
    // }

    // Conditional assignment - only updates fields that have values
    let conditional_update = updates::empty::<UserSettings>()
        .if_some(maybe_theme, |builder, theme| {
            builder.set::<usersettings_fields::Theme, _>(theme)
        })
        .if_some(maybe_notifications, |builder, notifications| {
            builder.set::<usersettings_fields::Notifications, _>(notifications)
        })
        .set::<usersettings_fields::LastUpdated, _>("2025-08-06".to_string())
        .build();
    // Result: {
    //   "$set": {
    //     "last_updated": "2025-08-06"
    //     // theme and notifications fields are only included if the Options contained values
    //   }
    // }
}
```

### Complex Operations with Optional Values

The `if_some` method supports complex update operations within the closure, allowing you to perform multiple updates when a value is present:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub name: String,
    pub tags: Vec<String>,
    pub discount: Option<f64>,
    pub sale_count: i32,
    pub featured: bool,
}

fn apply_discount_campaign(
    product_name: String,
    maybe_discount: Option<f64>,
) -> bson::Document {
    updates::empty::<Product>()
        .set::<product_fields::Name, _>(product_name)
        .if_some(maybe_discount, |builder, discount| {
            builder
                .set::<product_fields::Discount, _>(discount)
                .push::<product_fields::Tags, _>("on-sale".to_string())
                .set::<product_fields::Featured, _>(true)
                .inc::<product_fields::SaleCount, _>(1)
        })
        .build()
}

// Alternative: Multiple if_some calls for different optional operations
fn flexible_product_update(
    maybe_discount: Option<f64>,
    maybe_new_tags: Option<Vec<String>>,
    maybe_feature: Option<bool>,
) -> bson::Document {
    updates::empty::<Product>()
        .if_some(maybe_discount, |builder, discount| {
            builder.set::<product_fields::Discount, _>(discount)
        })
        .if_some(maybe_new_tags, |builder, tags| {
            let mut updated_builder = builder;
            for tag in tags {
                updated_builder = updated_builder.push::<product_fields::Tags, _>(tag);
            }
            updated_builder
        })
        .if_some(maybe_feature, |builder, featured| {
            builder.set::<product_fields::Featured, _>(featured)
        })
        .build()
}
```

### Comparison with Traditional Conditional Logic

The `if_some` method provides a more fluent alternative to traditional `if let Some(...)` patterns:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Settings {
    pub theme: Option<String>,
    pub notifications: Option<bool>,
    pub language: Option<String>,
}

// Traditional approach with explicit conditionals
fn update_settings_traditional(
    maybe_theme: Option<String>,
    maybe_notifications: Option<bool>,
    maybe_language: Option<String>,
) -> bson::Document {
    let mut builder = updates::empty::<Settings>();
    
    if let Some(theme) = maybe_theme {
        builder.set::<settings_fields::Theme, _>(theme);
    }
    
    if let Some(notifications) = maybe_notifications {
        builder.set::<settings_fields::Notifications, _>(notifications);
    }
    
    if let Some(language) = maybe_language {
        builder.set::<settings_fields::Language, _>(language);
    }
    
    builder.build()
}

// Fluent approach using if_some
fn update_settings_fluent(
    maybe_theme: Option<String>,
    maybe_notifications: Option<bool>,
    maybe_language: Option<String>,
) -> bson::Document {
    updates::empty::<Settings>()
        .if_some(maybe_theme, |builder, theme| {
            builder.set::<settings_fields::Theme, _>(theme)
        })
        .if_some(maybe_notifications, |builder, notifications| {
            builder.set::<settings_fields::Notifications, _>(notifications)
        })
        .if_some(maybe_language, |builder, language| {
            builder.set::<settings_fields::Language, _>(language)
        })
        .build()
}
```

### Nested Optional Operations

You can also chain `if_some` calls for handling nested optional values:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct AdvancedSettings {
    pub basic_setting: String,
    pub optional_setting: Option<String>,
    pub nested_config: Option<String>,
}

fn handle_nested_optionals(
    outer_option: Option<Option<String>>,
) -> bson::Document {
    updates::empty::<AdvancedSettings>()
        .set::<advancedsettings_fields::BasicSetting, _>("always_set".to_string())
        .if_some(outer_option, |builder, inner_option| {
            builder.if_some(inner_option, |inner_builder, value| {
                inner_builder.set::<advancedsettings_fields::NestedConfig, _>(value)
            })
        })
        .build()
}
```

### Working with Results and Option Conversion

The `if_some` method works well with functions that return `Option<T>`, including converted `Result<T, E>` types:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct ProcessedData {
    pub input: String,
    pub processed_value: Option<i32>,
    pub status: String,
}

fn parse_and_update(input_data: &str) -> bson::Document {
    // Function that might fail to parse
    fn try_parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse()
    }
    
    updates::empty::<ProcessedData>()
        .set::<processeddata_fields::Input, _>(input_data.to_string())
        .if_some(try_parse_number(input_data).ok(), |builder, parsed_value| {
            builder
                .set::<processeddata_fields::ProcessedValue, _>(parsed_value)
                .set::<processeddata_fields::Status, _>("success".to_string())
        })
        .build()
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
