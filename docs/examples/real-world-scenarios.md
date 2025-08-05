---
title: Real-World Scenarios
layout: page
nav_exclude: true
---

Practical examples showing how to use Tnuctipun in real-world applications, including e-commerce, user management, and analytics.

## E-Commerce Product Catalog

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub tags: Vec<String>,
    pub in_stock: bool,
    pub stock_quantity: i32,
    pub rating: f32,
    pub reviews_count: i32,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

// Product search with basic filters
async fn search_products(
    collection: &mongodb::Collection<Product>,
    category: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    min_rating: Option<f32>,
    in_stock_only: bool,
) -> mongodb::error::Result<Vec<Product>> {
    
    let mut filter_builder = empty::<Product>();
    
    // Category filter
    if let Some(cat) = category {
        filter_builder.eq::<product_fields::Category, _>(cat);
    }
    
    // Price range
    if let Some(min) = min_price {
        filter_builder.gte::<product_fields::Price, _>(min);
    }
    if let Some(max) = max_price {
        filter_builder.lte::<product_fields::Price, _>(max);
    }
    
    // Rating filter
    if let Some(min_rat) = min_rating {
        filter_builder.gte::<product_fields::Rating, _>(min_rat);
    }
    
    // Stock filter
    if in_stock_only {
        filter_builder.eq::<product_fields::InStock, _>(true);
        filter_builder.gt::<product_fields::StockQuantity, _>(0);
    }
    
    let filter = filter_builder.and();
    
    let cursor = collection.find(filter, None).await?;
    // Note: cursor iteration would be implemented here
    let products = Vec::new();
    
    Ok(products)
}

// Update product inventory
async fn update_product_inventory(
    collection: &mongodb::Collection<Product>,
    product_id: &str,
    quantity_sold: i32,
) -> mongodb::error::Result<()> {
    
    let filter = bson::doc! { "_id": product_id };
    
    let update_doc = updates::empty::<Product>()
        .inc::<product_fields::StockQuantity, _>(-quantity_sold)
        .set::<product_fields::UpdatedAt, _>(bson::DateTime::now())
        .build();
    
    collection.update_one(filter, update_doc, None).await?;
    
    Ok(())
}
```

## User Management System

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserActivity {
    pub user_id: String,
    pub session_id: String,
    pub event_type: String,
    pub timestamp: bson::DateTime,
    pub page_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub metadata: bson::Document,
}

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: String,
    pub registration_date: bson::DateTime,
    pub last_active: Option<bson::DateTime>,
    pub total_orders: i32,
    pub total_spent: f64,
    pub favorite_categories: Vec<String>,
    pub marketing_consent: bool,
}

// Find users for marketing campaigns
async fn find_marketing_candidates(
    user_collection: &mongodb::Collection<UserProfile>,
) -> mongodb::error::Result<Vec<UserProfile>> {
    
    let filter = empty::<UserProfile>()
        .eq::<userprofile_fields::MarketingConsent, _>(true)
        .gte::<userprofile_fields::TotalOrders, _>(1)  // Has made at least one order
        .lt::<userprofile_fields::TotalSpent, _>(1000.0)  // Not high-value customers
        .and();
    
    let projection = projection::empty::<UserProfile>()
        .includes::<userprofile_fields::Id>()
        .includes::<userprofile_fields::Email>()
        .includes::<userprofile_fields::Name>()
        .includes::<userprofile_fields::TotalOrders>()
        .includes::<userprofile_fields::TotalSpent>()
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection)
        .limit(1000)
        .build();
    
    let cursor = user_collection.find(filter, find_options).await?;
    // Note: cursor iteration would be implemented here
    let users = Vec::new();
    
    Ok(users)
}

// Update user profile after order
async fn update_user_after_order(
    collection: &mongodb::Collection<UserProfile>,
    user_id: &str,
    order_amount: f64,
) -> mongodb::error::Result<()> {
    
    let filter = bson::doc! { "_id": user_id };
    
    let update_doc = updates::empty::<UserProfile>()
        .inc::<userprofile_fields::TotalOrders, _>(1)
        .inc::<userprofile_fields::TotalSpent, _>(order_amount)
        .set::<userprofile_fields::LastActive, _>(Some(bson::DateTime::now()))
        .build();
    
    collection.update_one(filter, update_doc, None).await?;
    
    Ok(())
}
```

## Content Management System

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Article {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: String,
    pub author_id: String,
    pub category: String,
    pub tags: Vec<String>,
    pub status: String,  // draft, published, archived
    pub featured: bool,
    pub view_count: i64,
    pub like_count: i32,
    pub comment_count: i32,
    pub published_at: Option<bson::DateTime>,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

// Get published articles for homepage
async fn get_published_articles(
    collection: &mongodb::Collection<Article>,
    category: Option<String>,
    featured_only: bool,
    page: u64,
    page_size: u64,
) -> mongodb::error::Result<Vec<Article>> {
    
    let mut filter_builder = empty::<Article>();
    
    // Only published articles
    filter_builder.eq::<article_fields::Status, _>("published".to_string());
    
    // Category filter
    if let Some(cat) = category {
        filter_builder.eq::<article_fields::Category, _>(cat);
    }
    
    // Featured filter
    if featured_only {
        filter_builder.eq::<article_fields::Featured, _>(true);
    }
    
    let filter = filter_builder.and();
    
    // Projection optimized for listing (exclude large content)
    let projection = projection::empty::<Article>()
        .includes::<article_fields::Id>()
        .includes::<article_fields::Title>()
        .includes::<article_fields::Slug>()
        .includes::<article_fields::Excerpt>()
        .includes::<article_fields::AuthorId>()
        .includes::<article_fields::Category>()
        .includes::<article_fields::Featured>()
        .includes::<article_fields::ViewCount>()
        .includes::<article_fields::PublishedAt>()
        .excludes::<article_fields::Content>()  // Large field
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection)
        .sort(bson::doc! { 
            "featured": -1,      // Featured articles first
            "published_at": -1   // Then by publish date
        })
        .skip(page * page_size)
        .limit(page_size as i64)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    // Note: cursor iteration would be implemented here
    let articles = Vec::new();
    
    Ok(articles)
}

// Track article views
async fn track_article_view(
    collection: &mongodb::Collection<Article>,
    article_id: &str,
) -> mongodb::error::Result<()> {
    
    let filter = bson::doc! { 
        "_id": article_id,
        "status": "published"
    };
    
    let update_doc = updates::empty::<Article>()
        .inc::<article_fields::ViewCount, _>(1)
        .set::<article_fields::UpdatedAt, _>(bson::DateTime::now())
        .build();
    
    collection.update_one(filter, update_doc, None).await?;
    
    Ok(())
}
```

## Financial Transaction System

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Transaction {
    pub id: String,
    pub account_id: String,
    pub transaction_type: String,  // credit, debit, transfer
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub reference_id: Option<String>,
    pub status: String,  // pending, completed, failed, cancelled
    pub created_at: bson::DateTime,
    pub processed_at: Option<bson::DateTime>,
    pub metadata: bson::Document,
}

// Find large transactions for monitoring
async fn detect_large_transactions(
    collection: &mongodb::Collection<Transaction>
) -> mongodb::error::Result<Vec<Transaction>> {
    
    let filter = empty::<Transaction>()
        .eq::<transaction_fields::Status, _>("completed".to_string())
        .gte::<transaction_fields::Amount, _>(10000.0)  // Large transactions
        .and();
    
    let find_options = mongodb::options::FindOptions::builder()
        .sort(bson::doc! { "created_at": -1 })
        .limit(100)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    // Note: cursor iteration would be implemented here
    let transactions = Vec::new();
    
    Ok(transactions)
}

// Calculate account balance (simplified)
async fn get_account_balance(
    collection: &mongodb::Collection<Transaction>,
    account_id: &str,
) -> mongodb::error::Result<f64> {
    
    let filter = empty::<Transaction>()
        .eq::<transaction_fields::AccountId, _>(account_id.to_string())
        .eq::<transaction_fields::Status, _>("completed".to_string())
        .and();
    
    // In a real implementation, this would use aggregation
    // For this example, returning a placeholder
    Ok(0.0)
}
```

## Best Practices for Real-World Usage

1. **Performance Optimization**
   - Use appropriate projections to limit data transfer
   - Structure queries to leverage database indexes
   - Apply selective filters early in the query chain

2. **Error Handling**
   - Always handle MongoDB operation results
   - Implement proper logging for debugging
   - Use connection pooling for high-traffic applications

3. **Data Consistency**
   - Use transactions for multi-document operations
   - Implement proper validation at the application level
   - Consider eventual consistency in distributed systems

4. **Security Considerations**
   - Validate all input parameters
   - Implement proper authentication and authorization
   - Use connection encryption in production

5. **Monitoring and Maintenance**
   - Monitor query performance and optimize slow queries
   - Implement proper backup and recovery procedures
   - Keep dependencies updated for security patches

## Notes on Advanced Features

Some advanced features shown in documentation examples may require additional implementation:

- **Complex Aggregations**: Use MongoDB's aggregation pipeline for complex analytics
- **Full-Text Search**: Implement MongoDB text indexes for search functionality  
- **Real-time Updates**: Use change streams for real-time data synchronization
- **Geospatial Queries**: Leverage MongoDB's geospatial capabilities for location-based features

## User Analytics and Segmentation

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserActivity {
    pub user_id: String,
    pub session_id: String,
    pub event_type: String,
    pub timestamp: bson::DateTime,
    pub page_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub metadata: bson::Document,
}

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: String,
    pub registration_date: bson::DateTime,
    pub last_active: Option<bson::DateTime>,
    pub total_orders: i32,
    pub total_spent: f64,
    pub favorite_categories: Vec<String>,
    pub marketing_consent: bool,
}

// Identify inactive users for re-engagement campaigns
async fn find_inactive_users_for_campaign(
    user_collection: &mongodb::Collection<UserProfile>,
    activity_collection: &mongodb::Collection<UserActivity>,
) -> mongodb::error::Result<Vec<UserProfile>> {
    
    // Define inactive period (30 days)
    let inactive_threshold = bson::DateTime::from_millis(
        bson::DateTime::now().timestamp_millis() - (30 * 24 * 60 * 60 * 1000)
    );
    let registration_cutoff = bson::DateTime::from_millis(
        bson::DateTime::now().timestamp_millis() - (60 * 24 * 60 * 60 * 1000)
    );
    
    let filter = empty::<UserProfile>()
        .eq::<userprofile_fields::MarketingConsent, _>(true)
        .lt::<userprofile_fields::RegistrationDate, _>(registration_cutoff)  // Not new users
        .gte::<userprofile_fields::TotalOrders, _>(1)  // Has made at least one order
        .lt::<userprofile_fields::TotalSpent, _>(1000.0)  // Not high-value (different strategy)
        // Advanced boolean logic would require additional API methods
        // For now, using simplified filter
        .and();
    
    let projection = projection::empty::<UserProfile>()
        .includes::<userprofile_fields::Id>()
        .includes::<userprofile_fields::Email>()
        .includes::<userprofile_fields::Name>()
        .includes::<userprofile_fields::TotalOrders>()
        .includes::<userprofile_fields::TotalSpent>()
        .includes::<userprofile_fields::FavoriteCategories>()
        .includes::<userprofile_fields::LastActive>()
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection)
        .sort(doc! { "total_spent": -1 })  // Prioritize higher-value customers
        .limit(1000)  // Campaign batch size
        .build();
    
    let cursor = user_collection.find(filter, find_options).await?;
    // Note: cursor iteration would be implemented here
    let users = Vec::new();
    
    Ok(users)
}

// Update user activity metrics
async fn update_user_metrics_on_order(
    collection: &mongodb::Collection<UserProfile>,
    user_id: &str,
    order_amount: f64,
    order_categories: Vec<String>,
) -> mongodb::error::Result<()> {
    
    let filter = doc! { "_id": user_id };
    
    let update_doc = updates::empty::<UserProfile>()
        .inc::<userprofile_fields::TotalOrders, _>(1)
        .inc::<userprofile_fields::TotalSpent, _>(order_amount)
        .set::<userprofile_fields::LastActive, _>(Some(bson::DateTime::now()))
        // Note: push_each operation may require additional API methods
        // .push_each::<userprofile_fields::FavoriteCategories, _>(order_categories)
        .build();
    
    collection.update_one(filter, update_doc, None).await?;
    
    Ok(())
}
```

## Content Management System

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Article {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: String,
    pub author_id: String,
    pub category: String,
    pub tags: Vec<String>,
    pub status: String,  // draft, published, archived
    pub featured: bool,
    pub view_count: i64,
    pub like_count: i32,
    pub comment_count: i32,
    pub published_at: Option<bson::DateTime>,
    pub created_at: bson::DateTime,
    pub updated_at: bson::DateTime,
}

// Public article listing with SEO optimization
async fn get_published_articles(
    collection: &mongodb::Collection<Article>,
    category: Option<String>,
    tag: Option<String>,
    featured_only: bool,
    page: u64,
    page_size: u64,
) -> mongodb::error::Result<Vec<Article>> {
    
    let mut filter_builder = empty::<Article>();
    
    // Only published articles
    filter_builder.eq::<article_fields::Status, _>("published".to_string());
    // Note: exists method may require additional API method
    // filter_builder.exists::<article_fields::PublishedAt, _>(true);
    
    // Category filter
    if let Some(cat) = category {
        filter_builder.eq::<article_fields::Category, _>(cat);
    }
    
    // Tag filter - simplified (in_array may require additional API)
    if let Some(_t) = tag {
        // filter_builder.in_array::<article_fields::Tags, _>(vec![t]);
    }
    
    // Featured filter
    if featured_only {
        filter_builder.eq::<article_fields::Featured, _>(true);
    }
    
    let filter = filter_builder.and();
    
    // Projection optimized for listing (exclude large content)
    let projection = projection::empty::<Article>()
        .includes::<article_fields::Id>()
        .includes::<article_fields::Title>()
        .includes::<article_fields::Slug>()
        .includes::<article_fields::Excerpt>()
        .includes::<article_fields::AuthorId>()
        .includes::<article_fields::Category>()
        .includes::<article_fields::Tags>()
        .includes::<article_fields::Featured>()
        .includes::<article_fields::ViewCount>()
        .includes::<article_fields::LikeCount>()
        .includes::<article_fields::CommentCount>()
        .includes::<article_fields::PublishedAt>()
        .excludes::<article_fields::Content>()  // Large field
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection)
        .sort(doc! { 
            "featured": -1,      // Featured articles first
            "published_at": -1   // Then by publish date
        })
        .skip(page * page_size)
        .limit(page_size as i64)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    // Note: try_collect requires futures TryStreamExt trait
    // Using simplified approach for this example

    let mut articles = Vec::new();
    // cursor iteration would be implemented here
    
    Ok(articles)
}

// Article engagement tracking
async fn track_article_view(
    collection: &mongodb::Collection<Article>,
    article_id: &str,
) -> mongodb::error::Result<()> {
    
    let filter = doc! { 
        "_id": article_id,
        "status": "published"
    };
    
    let update_doc = updates::empty::<Article>()
        .inc::<article_fields::ViewCount, _>(1)
        .set::<article_fields::UpdatedAt, _>(bson::DateTime::now())
        .build();
    
    collection.update_one(filter, update_doc, None).await?;
    
    Ok(())
}

// Content moderation queries
async fn get_articles_for_review(
    collection: &mongodb::Collection<Article>
) -> mongodb::error::Result<Vec<Article>> {
    
    let filter = empty::<Article>()
        .eq::<article_fields::Status, _>("draft".to_string())
        .gt::<article_fields::CreatedAt, _>(
            bson::DateTime::from_millis(
                bson::DateTime::now().timestamp_millis() - (30 * 24 * 60 * 60 * 1000)
            )
        )
        .and();
    
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "created_at": 1 })  // Oldest first
        .limit(50)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    // Note: cursor iteration would be implemented here
 
    let articles = Vec::new();
    
    Ok(articles)
}
```

## Financial Transaction System

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Transaction {
    pub id: String,
    pub account_id: String,
    pub transaction_type: String,  // credit, debit, transfer
    pub amount: f64,  // Simplified from Decimal128
    pub currency: String,
    pub description: String,
    pub reference_id: Option<String>,
    pub status: String,  // pending, completed, failed, cancelled
    pub created_at: bson::DateTime,
    pub processed_at: Option<bson::DateTime>,
    pub metadata: bson::Document,
}

// Fraud detection queries
async fn detect_suspicious_transactions(
    collection: &mongodb::Collection<Transaction>
) -> mongodb::error::Result<Vec<Transaction>> {
    
    // Using simplified filter for large transactions
    let filter = empty::<Transaction>()
        .eq::<transaction_fields::Status, _>("completed".to_string())
        .gte::<transaction_fields::Amount, _>(10000.0)  // Large transactions
        .and();
    
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .limit(100)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    // Note: try_collect requires futures TryStreamExt trait
    // Using simplified approach for this example

    let mut transactions = Vec::new();
    // cursor iteration would be implemented here
    
    Ok(transactions)
}

// Account balance calculation
async fn calculate_account_balance(
    collection: &mongodb::Collection<Transaction>,
    account_id: &str,
    up_to_date: Option<bson::DateTime>,
) -> mongodb::error::Result<bson::Decimal128> {
    
    let filter = if let Some(date) = up_to_date {
        empty::<Transaction>()
            .eq::<transaction_fields::AccountId, _>(account_id.to_string())
            .eq::<transaction_fields::Status, _>("completed".to_string())
            .lte::<transaction_fields::ProcessedAt, _>(Some(date))
            .and()
    } else {
        empty::<Transaction>()
            .eq::<transaction_fields::AccountId, _>(account_id.to_string())
            .eq::<transaction_fields::Status, _>("completed".to_string())
            .and()
    };
    
    // Use aggregation for balance calculation
    let pipeline = vec![
        doc! { "$match": filter },
        doc! { "$group": {
            "_id": null,
            "total_credits": {
                "$sum": {
                    "$cond": [
                        { "$eq": ["$transaction_type", "credit"] },
                        "$amount",
                        0
                    ]
                }
            },
            "total_debits": {
                "$sum": {
                    "$cond": [
                        { "$eq": ["$transaction_type", "debit"] },
                        "$amount",
                        0
                    ]
                }
            }
        }},
        doc! { "$project": {
            "balance": { "$subtract": ["$total_credits", "$total_debits"] }
        }}
    ];
    
    let mut cursor = collection.aggregate(pipeline, None).await?;
    
    // Note: try_next requires futures TryStreamExt trait
    // Using simplified approach for this example
    let balance = bson::Decimal128::from_bytes([0u8; 16]);  // Default zero
    
    Ok(balance)
}
```

These examples demonstrate how Tnuctipun enables building complex, real-world applications with type-safe MongoDB operations while maintaining performance and code clarity.
