---
title: Real-World Scenarios
layout: page
parent: Examples
---

# Real-World Scenarios

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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Product search with filters
async fn search_products(
    collection: &mongodb::Collection<Product>,
    search_term: Option<String>,
    category: Option<String>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    min_rating: Option<f32>,
    in_stock_only: bool,
    page: u64,
    page_size: u64,
) -> mongodb::error::Result<Vec<Product>> {
    
    let mut filter_builder = empty::<Product>();
    
    // Text search in name and description
    if let Some(term) = search_term {
        let regex_pattern = format!(".*{}.*", regex::escape(&term));
        filter_builder.with_lookup(|text_filter| {
            text_filter.regex::<product_fields::Name, _>(regex_pattern.clone());
            text_filter.regex::<product_fields::Description, _>(regex_pattern);
            text_filter.or()
        });
    }
    
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
    
    // Projection for list view (exclude large fields)
    let projection_doc = projection::empty::<Product>()
        .includes::<product_fields::Id>()
        .includes::<product_fields::Name>()
        .includes::<product_fields::Price>()
        .includes::<product_fields::Category>()
        .includes::<product_fields::InStock>()
        .includes::<product_fields::Rating>()
        .includes::<product_fields::ReviewsCount>()
        .excludes::<product_fields::Description>()  // Large field
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection_doc)
        .sort(doc! { "rating": -1, "reviews_count": -1 })
        .skip(page * page_size)
        .limit(page_size as i64)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    let products: Vec<Product> = cursor.try_collect().await?;
    
    Ok(products)
}

// Update product inventory
async fn update_product_inventory(
    collection: &mongodb::Collection<Product>,
    product_id: &str,
    quantity_sold: i32,
) -> mongodb::error::Result<Option<Product>> {
    
    let filter = doc! { 
        "_id": product_id,
        "stock_quantity": { "$gte": quantity_sold }  // Ensure sufficient stock
    };
    
    let update_doc = updates::empty::<Product>()
        .inc::<product_fields::StockQuantity, _>(-quantity_sold)
        .set::<product_fields::UpdatedAt, _>(chrono::Utc::now())
        // Set in_stock to false if quantity becomes 0
        .build();
    
    // Use findOneAndUpdate for atomic operation
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();
    
    let result = collection
        .find_one_and_update(filter, update_doc, options)
        .await?;
    
    // Update in_stock flag if needed
    if let Some(ref product) = result {
        if product.stock_quantity <= 0 {
            let stock_filter = doc! { "_id": product_id };
            let stock_update = updates::empty::<Product>()
                .set::<product_fields::InStock, _>(false)
                .build();
            
            collection.update_one(stock_filter, stock_update, None).await?;
        }
    }
    
    Ok(result)
}
```

## User Analytics and Segmentation

```rust
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserActivity {
    pub user_id: String,
    pub session_id: String,
    pub event_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub page_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub metadata: bson::Document,
}

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct UserProfile {
    pub id: String,
    pub email: String,
    pub name: String,
    pub registration_date: chrono::DateTime<chrono::Utc>,
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
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
    let inactive_threshold = chrono::Utc::now() - chrono::Duration::days(30);
    let registration_cutoff = chrono::Utc::now() - chrono::Duration::days(60);
    
    let filter = empty::<UserProfile>()
        .eq::<user_profile_fields::MarketingConsent, _>(true)
        .lt::<user_profile_fields::RegistrationDate, _>(registration_cutoff)  // Not new users
        .gte::<user_profile_fields::TotalOrders, _>(1)  // Has made at least one order
        .lt::<user_profile_fields::TotalSpent, _>(1000.0)  // Not high-value (different strategy)
        .with_lookup(|activity_filter| {
            // Either no last_active or last_active is old
            activity_filter.exists::<user_profile_fields::LastActive, _>(false);
            activity_filter.lt::<user_profile_fields::LastActive, _>(Some(inactive_threshold));
            activity_filter.or()
        })
        .and();
    
    let projection = projection::empty::<UserProfile>()
        .includes::<user_profile_fields::Id>()
        .includes::<user_profile_fields::Email>()
        .includes::<user_profile_fields::Name>()
        .includes::<user_profile_fields::TotalOrders>()
        .includes::<user_profile_fields::TotalSpent>()
        .includes::<user_profile_fields::FavoriteCategories>()
        .includes::<user_profile_fields::LastActive>()
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection)
        .sort(doc! { "total_spent": -1 })  // Prioritize higher-value customers
        .limit(1000)  // Campaign batch size
        .build();
    
    let cursor = user_collection.find(filter, find_options).await?;
    let users: Vec<UserProfile> = cursor.try_collect().await?;
    
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
        .inc::<user_profile_fields::TotalOrders, _>(1)
        .inc::<user_profile_fields::TotalSpent, _>(order_amount)
        .set::<user_profile_fields::LastActive, _>(Some(chrono::Utc::now()))
        .push_each::<user_profile_fields::FavoriteCategories, _>(order_categories)
        .build();
    
    collection.update_one(filter, update_doc, None).await?;
    
    Ok(())
}
```

## Content Management System

```rust
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
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    filter_builder.exists::<article_fields::PublishedAt, _>(true);
    
    // Category filter
    if let Some(cat) = category {
        filter_builder.eq::<article_fields::Category, _>(cat);
    }
    
    // Tag filter
    if let Some(t) = tag {
        filter_builder.in_array::<article_fields::Tags, _>(vec![t]);
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
    let articles: Vec<Article> = cursor.try_collect().await?;
    
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
        .set::<article_fields::UpdatedAt, _>(chrono::Utc::now())
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
            chrono::Utc::now() - chrono::Duration::days(30)
        )
        .and();
    
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "created_at": 1 })  // Oldest first
        .limit(50)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    let articles: Vec<Article> = cursor.try_collect().await?;
    
    Ok(articles)
}
```

## Financial Transaction System

```rust
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Transaction {
    pub id: String,
    pub account_id: String,
    pub transaction_type: String,  // credit, debit, transfer
    pub amount: bson::Decimal128,
    pub currency: String,
    pub description: String,
    pub reference_id: Option<String>,
    pub status: String,  // pending, completed, failed, cancelled
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: bson::Document,
}

// Fraud detection queries
async fn detect_suspicious_transactions(
    collection: &mongodb::Collection<Transaction>
) -> mongodb::error::Result<Vec<Transaction>> {
    
    let recent_time = chrono::Utc::now() - chrono::Duration::hours(24);
    
    let filter = empty::<Transaction>()
        .eq::<transaction_fields::Status, _>("completed".to_string())
        .gte::<transaction_fields::CreatedAt, _>(recent_time)
        .with_lookup(|suspicious_filter| {
            // Large transactions
            suspicious_filter.gte::<transaction_fields::Amount, _>(
                bson::Decimal128::from_str("10000.00").unwrap()
            );
            
            // Multiple transactions in short time (handled by aggregation)
            // Unusual patterns would be detected in application logic
            
            suspicious_filter.build()
        })
        .and();
    
    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "created_at": -1 })
        .limit(100)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    let transactions: Vec<Transaction> = cursor.try_collect().await?;
    
    Ok(transactions)
}

// Account balance calculation
async fn calculate_account_balance(
    collection: &mongodb::Collection<Transaction>,
    account_id: &str,
    up_to_date: Option<chrono::DateTime<chrono::Utc>>,
) -> mongodb::error::Result<bson::Decimal128> {
    
    let mut filter_builder = empty::<Transaction>();
    filter_builder.eq::<transaction_fields::AccountId, _>(account_id.to_string());
    filter_builder.eq::<transaction_fields::Status, _>("completed".to_string());
    
    if let Some(date) = up_to_date {
        filter_builder.lte::<transaction_fields::ProcessedAt, _>(Some(date));
    }
    
    let filter = filter_builder.and();
    
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
    
    if let Some(result) = cursor.try_next().await? {
        if let Ok(balance) = result.get_decimal128("balance") {
            Ok(*balance)
        } else {
            Ok(bson::Decimal128::from_str("0.00").unwrap())
        }
    } else {
        Ok(bson::Decimal128::from_str("0.00").unwrap())
    }
}
```

These examples demonstrate how Tnuctipun enables building complex, real-world applications with type-safe MongoDB operations while maintaining performance and code clarity.
