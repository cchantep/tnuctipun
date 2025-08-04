---
title: Advanced Topics
layout: page
nav_order: 6
parent: User Guide
---

# Advanced Query Patterns

This guide demonstrates advanced Tnuctipun-specific query building techniques that leverage the library's type-safe field access and compile-time validation features.

## Dynamic Query Building

Build queries dynamically based on runtime parameters:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
struct UserSearchCriteria {
    name_pattern: Option<String>,
    min_age: Option<i32>,
    max_age: Option<i32>,
    roles: Option<Vec<String>>,
    active_only: bool,
    created_after: Option<chrono::DateTime<chrono::Utc>>,
}

fn build_dynamic_user_query(criteria: UserSearchCriteria) -> bson::Document {
    let mut filter_builder = empty::<User>();
    
    // Name pattern matching
    if let Some(pattern) = criteria.name_pattern {
        filter_builder.regex::<user_fields::Name, _>(format!(".*{}.*", pattern));
    }
    
    // Age range filtering
    if let Some(min_age) = criteria.min_age {
        filter_builder.gte::<user_fields::Age, _>(min_age);
    }
    
    if let Some(max_age) = criteria.max_age {
        filter_builder.lte::<user_fields::Age, _>(max_age);
    }
    
    // Role filtering
    if let Some(roles) = criteria.roles {
        if !roles.is_empty() {
            filter_builder.r#in::<user_fields::Role, _>(roles);
        }
    }
    
    // Active status
    if criteria.active_only {
        filter_builder.eq::<user_fields::IsActive, _>(true);
    }
    
    // Creation date filtering
    if let Some(created_after) = criteria.created_after {
        filter_builder.gte::<user_fields::CreatedAt, _>(created_after);
    }
    
    filter_builder.and()
}
```

## Complex Nested Logic

Build complex boolean logic with nested conditions using `with_lookup` and `with_field`:

```rust
use tnuctipun::filters::empty;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
    pub zip_code: String,
}

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
    pub role: String,
    pub login_count: i32,
    pub home_address: Address,
    pub work_address: Option<Address>,
    pub created_at: bson::DateTime,
}

fn complex_user_segmentation() -> bson::Document {
    let mut filter = empty::<User>();
    
    // Base condition: must be active
    filter.eq::<user_fields::IsActive, _>(true);
    
    // Use with_field for simple field-level operations
    filter.with_field::<user_fields::Role, _>(|role_filter| {
        // Premium or VIP users
        let premium_roles = vec!["premium".to_string(), "vip".to_string()];
        role_filter.r#in::<user_fields::Role, _>(premium_roles)
    });
    
    // Age restrictions using with_field
    filter.with_field::<user_fields::Age, _>(|age_filter| {
        age_filter
            .gte::<user_fields::Age, _>(18)
            .lte::<user_fields::Age, _>(65)
    });
    
    // Use with_lookup for nested field access - home address filtering
    filter.with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
        |path| path.field::<address_fields::City>(),
        |nested| {
            // Users from major cities
            let major_cities = vec![
                "New York".to_string(),
                "Los Angeles".to_string(),
                "Chicago".to_string(),
                "San Francisco".to_string(),
            ];
            nested.r#in::<address_fields::City, _>(major_cities)
        }
    );
    
    // Additional nested field filtering - country restriction
    filter.with_lookup::<user_fields::HomeAddress, _, address_fields::Country, Address, _>(
        |path| path.field::<address_fields::Country>(),
        |nested| {
            nested.eq::<address_fields::Country, _>("United States".to_string())
        }
    );
    
    filter.and()
}
```

## Best Practices Summary

1. **Dynamic Queries**: Use Tnuctipun's filter builders for runtime query construction
2. **Complex Logic**: Leverage `with_lookup` for nested boolean conditions
3. **Type Safety**: Leverage Tnuctipun's compile-time field validation throughout your application

These patterns demonstrate Tnuctipun's specific capabilities for building type-safe, maintainable MongoDB applications.
