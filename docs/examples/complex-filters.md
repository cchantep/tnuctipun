---
title: Complex Filters
layout: page
nav_exclude: true
---

# Complex Filter Examples

Advanced filtering examples showing dynamic query building and complex conditions using the available Tnuctipun API.

## Multi-Condition Filtering

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub role: String,
    pub is_active: bool,
    pub login_count: i32,
    pub last_login: Option<bson::DateTime>,
}

// Find users who meet complex criteria
fn complex_user_segmentation() -> bson::Document {
    empty::<User>()
        .eq::<user_fields::IsActive, _>(true)
        .eq::<user_fields::Role, _>("premium".to_string())
        .gte::<user_fields::LoginCount, _>(10)
        .gte::<user_fields::Age, _>(18)
        .and()
}
```

## Dynamic Query Building

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub role: String,
    pub is_active: bool,
    pub login_count: i32,
    pub last_login: Option<bson::DateTime>,
}

#[derive(Debug)]
struct UserSearchCriteria {
    name_pattern: Option<String>,
    min_age: Option<i32>,
    max_age: Option<i32>,
    role: Option<String>,
    min_login_count: Option<i32>,
    active_only: bool,
}

fn build_dynamic_user_query(criteria: UserSearchCriteria) -> bson::Document {
    let mut filter_builder = empty::<User>();
    
    // Name pattern matching (exact match for now)
    if let Some(name) = criteria.name_pattern {
        filter_builder.eq::<user_fields::Name, _>(name);
    }
    
    // Age range filtering
    if let Some(min_age) = criteria.min_age {
        filter_builder.gte::<user_fields::Age, _>(min_age);
    }
    if let Some(max_age) = criteria.max_age {
        filter_builder.lte::<user_fields::Age, _>(max_age);
    }
    
    // Role filtering
    if let Some(role) = criteria.role {
        filter_builder.eq::<user_fields::Role, _>(role);
    }
    
    // Active status
    if criteria.active_only {
        filter_builder.eq::<user_fields::IsActive, _>(true);
    }
    
    // Login count threshold
    if let Some(min_login_count) = criteria.min_login_count {
        filter_builder.gte::<user_fields::LoginCount, _>(min_login_count);
    }
    
    filter_builder.and()
}
```

## Multi-Field Conditions

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Order {
    pub id: String,
    pub customer_id: String,
    pub total: f64,
    pub status: String,
    pub created_at: bson::DateTime,
    pub items_count: i32,
}

// Complex business logic filters
fn complex_order_analysis() -> Vec<bson::Document> {
    let mut queries = Vec::new();
    
    // High-value recent orders
    let high_value_filter = empty::<Order>()
        .gte::<order_fields::Total, _>(1000.0)
        .eq::<order_fields::Status, _>("completed".to_string())
        .gte::<order_fields::ItemsCount, _>(5)
        .and();
    queries.push(high_value_filter);
    
    // Bulk orders 
    let bulk_filter = empty::<Order>()
        .gte::<order_fields::ItemsCount, _>(20)
        .ne::<order_fields::Status, _>("cancelled".to_string())
        .and();
    queries.push(bulk_filter);
    
    queries
}
```

## Best Practices for Complex Filters

1. **Index Strategy**: Structure queries to leverage database indexes
2. **Selective Filters First**: Apply most selective conditions early
3. **Dynamic Building**: Use conditional filter building for flexible queries
4. **Type Safety**: Leverage Tnuctipun's compile-time field validation
5. **Performance**: Consider query execution plans for complex conditions

## Notes on Advanced Features

Some advanced filtering features shown in other documentation examples may require additional API methods:

- **Regex Support**: Pattern matching methods like `.regex()` 
- **Array Operations**: Methods like `.in_array()` for multi-value filtering
- **Nested Logic**: Advanced boolean combinators like `.with_lookup()`
- **Text Search**: Full-text search capabilities

Check the current API documentation for available methods and their signatures.
