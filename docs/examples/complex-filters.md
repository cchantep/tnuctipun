---
title: Complex Filters
layout: page
parent: Examples
---

# Complex Filter Examples

Advanced filtering examples showing nested boolean logic, dynamic query building, and complex conditions.

## Nested Boolean Logic

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
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

// Find users who are either:
// - Premium users (any age), OR
// - Regular users who are active and have logged in recently
fn complex_user_segmentation() -> bson::Document {
    let mut main_filter = empty::<User>();
    
    // Base condition: must be active
    main_filter.eq::<user_fields::IsActive, _>(true);
    
    // Complex OR condition
    main_filter.with_lookup(|segment_filter| {
        // Premium users (any age)
        segment_filter.with_lookup(|premium_filter| {
            premium_filter.eq::<user_fields::Role, _>("premium".to_string());
        });
        
        // Regular users with specific criteria
        segment_filter.with_lookup(|regular_filter| {
            regular_filter.eq::<user_fields::Role, _>("regular".to_string());
            regular_filter.gte::<user_fields::LoginCount, _>(10);
            
            // Recent login (within last 30 days)
            let thirty_days_ago = chrono::Utc::now() - chrono::Duration::days(30);
            regular_filter.gte::<user_fields::LastLogin, _>(Some(thirty_days_ago));
        });
        
        segment_filter.or()  // Premium OR Active Regular
    });
    
    main_filter.and()
}

// Find users in specific age groups with role-based logic
fn age_and_role_complex_filter() -> bson::Document {
    let mut main_filter = empty::<User>();
    
    // Must be active
    main_filter.eq::<user_fields::IsActive, _>(true);
    
    // Complex age and role logic
    main_filter.with_lookup(|age_role_filter| {
        // Young adults (18-30) - any role
        age_role_filter.with_lookup(|young_adult_filter| {
            young_adult_filter.gte::<user_fields::Age, _>(18);
            young_adult_filter.lte::<user_fields::Age, _>(30);
        });
        
        // Middle-aged (31-50) - must be premium or admin
        age_role_filter.with_lookup(|middle_aged_filter| {
            middle_aged_filter.gte::<user_fields::Age, _>(31);
            middle_aged_filter.lte::<user_fields::Age, _>(50);
            
            middle_aged_filter.with_lookup(|role_filter| {
                role_filter.eq::<user_fields::Role, _>("premium".to_string());
                role_filter.eq::<user_fields::Role, _>("admin".to_string());
                role_filter.or()  // premium OR admin
            });
        });
        
        // Seniors (51+) - must be premium with high login count
        age_role_filter.with_lookup(|senior_filter| {
            senior_filter.gte::<user_fields::Age, _>(51);
            senior_filter.eq::<user_fields::Role, _>("premium".to_string());
            senior_filter.gte::<user_fields::LoginCount, _>(50);
        });
        
        age_role_filter.or()  // Young OR (Middle-aged + Premium/Admin) OR (Senior + Premium + Active)
    });
    
    main_filter.and()
}
```

## Dynamic Query Building

```rust
#[derive(Debug)]
struct UserSearchCriteria {
    name_pattern: Option<String>,
    min_age: Option<i32>,
    max_age: Option<i32>,
    roles: Option<Vec<String>>,
    active_only: bool,
    min_login_count: Option<i32>,
    recent_login_days: Option<i64>,
}

fn build_dynamic_user_query(criteria: UserSearchCriteria) -> bson::Document {
    let mut filter_builder = empty::<User>();
    
    // Name pattern matching (case-insensitive regex)
    if let Some(pattern) = criteria.name_pattern {
        let regex_pattern = format!(".*{}.*", regex::escape(&pattern));
        filter_builder.regex::<user_fields::Name, _>(regex_pattern);
    }
    
    // Age range
    if let Some(min_age) = criteria.min_age {
        filter_builder.gte::<user_fields::Age, _>(min_age);
    }
    if let Some(max_age) = criteria.max_age {
        filter_builder.lte::<user_fields::Age, _>(max_age);
    }
    
    // Role filtering
    if let Some(roles) = criteria.roles {
        if !roles.is_empty() {
            filter_builder.in_array::<user_fields::Role, _>(roles);
        }
    }
    
    // Active status
    if criteria.active_only {
        filter_builder.eq::<user_fields::IsActive, _>(true);
    }
    
    // Login count threshold
    if let Some(min_login_count) = criteria.min_login_count {
        filter_builder.gte::<user_fields::LoginCount, _>(min_login_count);
    }
    
    // Recent login filtering
    if let Some(days) = criteria.recent_login_days {
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days);
        filter_builder.gte::<user_fields::LastLogin, _>(Some(cutoff_date));
    }
    
    filter_builder.and()
}

// Usage example
fn example_dynamic_queries() {
    // Search for active premium users aged 25-40 with recent activity
    let criteria = UserSearchCriteria {
        name_pattern: None,
        min_age: Some(25),
        max_age: Some(40),
        roles: Some(vec!["premium".to_string()]),
        active_only: true,
        min_login_count: Some(5),
        recent_login_days: Some(7),  // Logged in within last week
    };
    
    let filter = build_dynamic_user_query(criteria);
    println!("Dynamic filter: {}", filter);
    
    // Search for users by name pattern
    let name_search = UserSearchCriteria {
        name_pattern: Some("john".to_string()),
        min_age: None,
        max_age: None,
        roles: None,
        active_only: true,
        min_login_count: None,
        recent_login_days: None,
    };
    
    let name_filter = build_dynamic_user_query(name_search);
    println!("Name search filter: {}", name_filter);
}
```

## Multi-Field Complex Conditions

```rust
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Order {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub total_amount: f64,
    pub items_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub shipping_country: String,
    pub payment_method: String,
}

// Find high-value orders with complex business logic
fn complex_order_analysis() -> bson::Document {
    let mut filter_builder = empty::<Order>();
    
    // Must be completed orders
    filter_builder.eq::<order_fields::Status, _>("completed".to_string());
    
    // Complex value-based segmentation
    filter_builder.with_lookup(|value_filter| {
        // High-value single-item orders (luxury items)
        value_filter.with_lookup(|luxury_filter| {
            luxury_filter.gte::<order_fields::TotalAmount, _>(1000.0);
            luxury_filter.lte::<order_fields::ItemsCount, _>(3);
        });
        
        // Medium-value bulk orders
        value_filter.with_lookup(|bulk_filter| {
            bulk_filter.gte::<order_fields::TotalAmount, _>(500.0);
            bulk_filter.gte::<order_fields::ItemsCount, _>(10);
        });
        
        // International premium orders
        value_filter.with_lookup(|international_filter| {
            international_filter.gte::<order_fields::TotalAmount, _>(200.0);
            international_filter.nin::<order_fields::ShippingCountry, _>(vec![
                "US".to_string(),
                "CA".to_string(),
            ]);
            international_filter.eq::<order_fields::PaymentMethod, _>("credit_card".to_string());
        });
        
        value_filter.or()  // Any of the above conditions
    });
    
    // Recent orders only (last 90 days)
    let ninety_days_ago = chrono::Utc::now() - chrono::Duration::days(90);
    filter_builder.gte::<order_fields::CreatedAt, _>(ninety_days_ago);
    
    filter_builder.and()
}

// Geographic and temporal analysis
fn geographic_temporal_filter() -> bson::Document {
    let mut filter_builder = empty::<Order>();
    
    // Time-based conditions
    let start_of_year = chrono::Utc.ymd(2024, 1, 1).and_hms(0, 0, 0);
    let start_of_quarter = chrono::Utc.ymd(2024, 10, 1).and_hms(0, 0, 0);
    
    filter_builder.gte::<order_fields::CreatedAt, _>(start_of_year);
    
    // Geographic segmentation with business logic
    filter_builder.with_lookup(|geo_filter| {
        // North American high-value orders
        geo_filter.with_lookup(|na_filter| {
            na_filter.in_array::<order_fields::ShippingCountry, _>(vec![
                "US".to_string(),
                "CA".to_string(),
                "MX".to_string(),
            ]);
            na_filter.gte::<order_fields::TotalAmount, _>(100.0);
        });
        
        // European orders (any value, but recent)
        geo_filter.with_lookup(|eu_filter| {
            eu_filter.in_array::<order_fields::ShippingCountry, _>(vec![
                "GB".to_string(),
                "FR".to_string(),
                "DE".to_string(),
                "IT".to_string(),
                "ES".to_string(),
            ]);
            eu_filter.gte::<order_fields::CreatedAt, _>(start_of_quarter);
        });
        
        // Rest of world - premium orders only
        geo_filter.with_lookup(|row_filter| {
            row_filter.nin::<order_fields::ShippingCountry, _>(vec![
                "US".to_string(), "CA".to_string(), "MX".to_string(),
                "GB".to_string(), "FR".to_string(), "DE".to_string(),
                "IT".to_string(), "ES".to_string(),
            ]);
            row_filter.gte::<order_fields::TotalAmount, _>(500.0);
            row_filter.eq::<order_fields::PaymentMethod, _>("credit_card".to_string());
        });
        
        geo_filter.or()  // Any geographic segment
    });
    
    filter_builder.and()
}
```

## Text Search and Pattern Matching

```rust
// Advanced text search patterns
fn text_search_examples() -> Vec<bson::Document> {
    let mut filters = Vec::new();
    
    // Case-insensitive name search
    let mut name_search = empty::<User>();
    name_search.regex::<user_fields::Name, _>("(?i)john.*smith".to_string());
    filters.push(name_search.build());
    
    // Email domain filtering
    let mut email_domain = empty::<User>();
    email_domain.regex::<user_fields::Email, _>(".*@(gmail|yahoo|hotmail)\\.com$".to_string());
    filters.push(email_domain.build());
    
    // Exclude test/internal users
    let mut production_users = empty::<User>();
    production_users.not_regex::<user_fields::Email, _>(".*@(test|internal|dev)\\.".to_string());
    production_users.ne::<user_fields::Name, _>("".to_string());  // Non-empty names
    filters.push(production_users.and());
    
    filters
}
```

## Performance-Optimized Complex Queries

```rust
// Optimized query for common dashboard metrics
fn dashboard_metrics_query() -> bson::Document {
    let mut filter_builder = empty::<User>();
    
    // Use indexed fields first
    filter_builder.eq::<user_fields::IsActive, _>(true);  // Assuming this is indexed
    
    // Date range on indexed field
    let last_month = chrono::Utc::now() - chrono::Duration::days(30);
    filter_builder.gte::<user_fields::LastLogin, _>(Some(last_month));
    
    // Role filtering (assuming compound index on is_active + role)
    filter_builder.in_array::<user_fields::Role, _>(vec![
        "premium".to_string(),
        "admin".to_string(),
    ]);
    
    filter_builder.and()
}
```
