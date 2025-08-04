---
title: Advanced Topics
layout: page
nav_order: 6
parent: User Guide
---

# Advanced Topics

This guide covers advanced usage patterns, complex scenarios, and best practices for using Tnuctipun in production applications.

## Table of Contents

- [Complex Query Patterns](#complex-query-patterns)
- [Performance Optimization](#performance-optimization)
- [Error Handling Patterns](#error-handling-patterns)
- [Integration Patterns](#integration-patterns)
- [Testing Strategies](#testing-strategies)
- [Production Considerations](#production-considerations)
- [Migration Strategies](#migration-strategies)

## Complex Query Patterns

### Dynamic Query Building

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
            filter_builder.in_array::<user_fields::Role, _>(roles);
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

### Complex Nested Logic

Build complex boolean logic with nested conditions:

```rust
fn complex_user_segmentation() -> bson::Document {
    let mut main_filter = empty::<User>();
    
    // Base condition: must be active
    main_filter.eq::<user_fields::IsActive, _>(true);
    
    // Complex segmentation logic:
    // (Premium users OR (Regular users who are very active))
    main_filter.with_lookup(|segment_filter| {
        // Premium users segment
        segment_filter.with_lookup(|premium_filter| {
            premium_filter.eq::<user_fields::Role, _>("premium".to_string());
        });
        
        // Very active regular users segment
        segment_filter.with_lookup(|active_regular_filter| {
            active_regular_filter.eq::<user_fields::Role, _>("regular".to_string());
            active_regular_filter.gte::<user_fields::LoginCount, _>(50); // Assuming this field exists
            active_regular_filter.gte::<user_fields::Age, _>(18);
            active_regular_filter.lte::<user_fields::Age, _>(65);
        });
        
        segment_filter.or()  // Premium OR Active Regular
    });
    
    main_filter.and()
}
```

### Multi-Collection Query Coordination

Coordinate queries across multiple related collections:

```rust
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Order {
    pub id: String,
    pub user_id: String,
    pub status: String,
    pub total_amount: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub id: String,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub in_stock: bool,
}

async fn find_high_value_customers_with_recent_orders(
    user_collection: &mongodb::Collection<User>,
    order_collection: &mongodb::Collection<Order>,
) -> Result<Vec<User>, mongodb::error::Error> {
    
    // Step 1: Find recent high-value orders
    let recent_date = chrono::Utc::now() - chrono::Duration::days(30);
    
    let high_value_order_filter = empty::<Order>()
        .gte::<order_fields::TotalAmount, _>(1000.0)
        .gte::<order_fields::CreatedAt, _>(recent_date)
        .eq::<order_fields::Status, _>("completed".to_string())
        .and();
    
    let projection = tnuctipun::projection::empty::<Order>()
        .includes::<order_fields::UserId>()
        .build();
    
    let find_options = mongodb::options::FindOptions::builder()
        .projection(projection)
        .build();
    
    let order_cursor = order_collection.find(high_value_order_filter, find_options).await?;
    let orders: Vec<Order> = order_cursor.try_collect().await?;
    
    // Step 2: Extract user IDs
    let user_ids: Vec<String> = orders.into_iter()
        .map(|order| order.user_id)
        .collect::<std::collections::HashSet<_>>()  // Deduplicate
        .into_iter()
        .collect();
    
    if user_ids.is_empty() {
        return Ok(Vec::new());
    }
    
    // Step 3: Find corresponding users
    let user_filter = empty::<User>()
        .in_array::<user_fields::Id, _>(user_ids)
        .eq::<user_fields::IsActive, _>(true)
        .and();
    
    let user_cursor = user_collection.find(user_filter, None).await?;
    let users: Vec<User> = user_cursor.try_collect().await?;
    
    Ok(users)
}
```

## Performance Optimization

### Query Optimization Strategies

```rust
// Efficient projection to reduce network traffic
fn optimized_user_list_projection() -> bson::Document {
    tnuctipun::projection::empty::<User>()
        .includes::<user_fields::Id>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .excludes::<user_fields::CreatedAt>()      // Exclude large timestamps
        .excludes::<user_fields::LastLoginData>()  // Exclude complex nested data
        .build()
}

// Index-friendly query patterns
fn index_optimized_queries() {
    // Leverage compound indexes: (is_active, role, created_at)
    let filter = empty::<User>()
        .eq::<user_fields::IsActive, _>(true)      // Use indexed field first
        .eq::<user_fields::Role, _>("premium".to_string())  // Then secondary index field
        .gte::<user_fields::CreatedAt, _>(chrono::Utc::now() - chrono::Duration::days(30))
        .and();
    
    // Range queries on indexed fields
    let age_range_filter = empty::<User>()
        .gte::<user_fields::Age, _>(18)            // Efficient range query
        .lte::<user_fields::Age, _>(65)
        .and();
}
```

### Batch Operations

```rust
use futures::stream::StreamExt;

async fn batch_update_users(
    collection: &mongodb::Collection<User>,
    user_updates: Vec<(String, String)>,  // (user_id, new_name)
) -> Result<(), mongodb::error::Error> {
    
    let batch_size = 100;
    let chunks: Vec<_> = user_updates.chunks(batch_size).collect();
    
    for chunk in chunks {
        let mut models = Vec::new();
        
        for (user_id, new_name) in chunk {
            let filter = doc! { "_id": user_id };
            
            let update = tnuctipun::updates::empty::<User>()
                .set::<user_fields::Name, _>(new_name.clone())
                .set::<user_fields::LastModified, _>(chrono::Utc::now())
                .build();
            
            models.push(mongodb::options::UpdateOneModel::builder()
                .filter(filter)
                .update(update)
                .build());
        }
        
        collection.bulk_write(models, None).await?;
    }
    
    Ok(())
}
```

### Connection Pool Optimization

```rust
use mongodb::{Client, options::ClientOptions};

async fn optimized_mongodb_client() -> Result<Client, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    
    // Connection pool optimization
    client_options.max_pool_size = Some(50);
    client_options.min_pool_size = Some(10);
    client_options.max_idle_time = Some(std::time::Duration::from_secs(300));
    
    // Read preferences for performance
    client_options.read_concern = Some(mongodb::options::ReadConcern::majority());
    client_options.write_concern = Some(mongodb::options::WriteConcern::builder()
        .w(mongodb::options::Acknowledgment::Majority)
        .journal(true)
        .build());
    
    let client = Client::with_options(client_options)?;
    Ok(client)
}
```

## Error Handling Patterns

### Comprehensive Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("MongoDB error: {0}")]
    Database(#[from] mongodb::error::Error),
    
    #[error("User not found: {id}")]
    UserNotFound { id: String },
    
    #[error("Invalid user data: {reason}")]
    InvalidData { reason: String },
    
    #[error("Permission denied for user: {user_id}")]
    PermissionDenied { user_id: String },
}

type Result<T> = std::result::Result<T, UserServiceError>;

async fn safe_user_operations(
    collection: &mongodb::Collection<User>,
    user_id: &str,
    new_name: Option<String>,
) -> Result<User> {
    
    // Validate input
    if let Some(ref name) = new_name {
        if name.trim().is_empty() {
            return Err(UserServiceError::InvalidData {
                reason: "Name cannot be empty".to_string(),
            });
        }
    }
    
    // Build filter with validation
    let filter = doc! { "_id": user_id };
    
    // Check if user exists
    let existing_user = collection.find_one(filter.clone(), None).await?
        .ok_or_else(|| UserServiceError::UserNotFound {
            id: user_id.to_string(),
        })?;
    
    // Permission check
    if !existing_user.is_active {
        return Err(UserServiceError::PermissionDenied {
            user_id: user_id.to_string(),
        });
    }
    
    // Build update if needed
    if let Some(name) = new_name {
        let update = tnuctipun::updates::empty::<User>()
            .set::<user_fields::Name, _>(name)
            .set::<user_fields::LastModified, _>(chrono::Utc::now())
            .build();
        
        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        
        let updated_user = collection
            .find_one_and_update(filter, update, options)
            .await?
            .ok_or_else(|| UserServiceError::UserNotFound {
                id: user_id.to_string(),
            })?;
        
        Ok(updated_user)
    } else {
        Ok(existing_user)
    }
}
```

### Retry Patterns

```rust
use tokio::time::{sleep, Duration};

async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
    base_delay: Duration,
) -> std::result::Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<T, E>> + Send>>,
    E: std::fmt::Debug,
{
    let mut attempts = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                attempts += 1;
                
                if attempts >= max_retries {
                    return Err(error);
                }
                
                let delay = base_delay * 2_u32.pow(attempts as u32 - 1);

                eprintln!("Operation failed (attempt {}), retrying in {:?}: {:?}", 
                         attempts, delay, error);

                sleep(delay).await;
            }
        }
    }
}

// Usage
async fn resilient_user_update(
    collection: &mongodb::Collection<User>,
    user_id: &str,
    new_name: String,
) -> Result<User> {
    
    retry_with_backoff(
        || {
            let collection = collection.clone();
            let user_id = user_id.to_string();
            let new_name = new_name.clone();
            
            Box::pin(async move {
                let filter = doc! { "_id": user_id };
                let update = tnuctipun::updates::empty::<User>()
                    .set::<user_fields::Name, _>(new_name)
                    .build();
                
                collection.update_one(filter, update, None).await
            })
        },
        3,  // max retries
        Duration::from_millis(100),  // base delay
    ).await
    .map_err(UserServiceError::Database)?;
    
    // Fetch updated document
    let filter = doc! { "_id": user_id };

    collection.find_one(filter, None).await?
        .ok_or_else(|| UserServiceError::UserNotFound {
            id: user_id.to_string(),
        })
}
```

## Integration Patterns

### Repository Pattern

```rust
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
    async fn find_by_criteria(&self, criteria: UserSearchCriteria) -> Result<Vec<User>>;
    async fn create(&self, user: User) -> Result<User>;
    async fn update(&self, id: &str, update: UserUpdate) -> Result<Option<User>>;
    async fn delete(&self, id: &str) -> Result<bool>;
}

pub struct MongoUserRepository {
    collection: mongodb::Collection<User>,
}

impl MongoUserRepository {
    pub fn new(database: &mongodb::Database) -> Self {
        Self {
            collection: database.collection("users"),
        }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<User>> {
        let filter = doc! { "_id": id };
        let user = self.collection.find_one(filter, None).await?;
        Ok(user)
    }
    
    async fn find_by_criteria(&self, criteria: UserSearchCriteria) -> Result<Vec<User>> {
        let filter = build_dynamic_user_query(criteria);
        let cursor = self.collection.find(filter, None).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }
    
    async fn create(&self, user: User) -> Result<User> {
        self.collection.insert_one(&user, None).await?;
        Ok(user)
    }
    
    async fn update(&self, id: &str, update_data: UserUpdate) -> Result<Option<User>> {
        let filter = doc! { "_id": id };
        
        let mut update_builder = tnuctipun::updates::empty::<User>();
        
        if let Some(name) = update_data.name {
            update_builder.set::<user_fields::Name, _>(name);
        }
        
        if let Some(email) = update_data.email {
            update_builder.set::<user_fields::Email, _>(email);
        }
        
        update_builder.set::<user_fields::LastModified, _>(chrono::Utc::now());
        
        let update = update_builder.build();
        
        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();
        
        let result = self.collection
            .find_one_and_update(filter, update, options)
            .await?;
        
        Ok(result)
    }
    
    async fn delete(&self, id: &str) -> Result<bool> {
        let filter = doc! { "_id": id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result.deleted_count > 0)
    }
}

#[derive(Debug)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
}
```

### Service Layer Pattern

```rust
pub struct UserService {
    repository: Box<dyn UserRepository + Send + Sync>,
    event_publisher: Box<dyn EventPublisher + Send + Sync>,
}

impl UserService {
    pub fn new(
        repository: Box<dyn UserRepository + Send + Sync>,
        event_publisher: Box<dyn EventPublisher + Send + Sync>,
    ) -> Self {
        Self {
            repository,
            event_publisher,
        }
    }
    
    pub async fn update_user_profile(
        &self,
        user_id: &str,
        update: UserUpdate,
    ) -> Result<User> {
        // Validation
        if let Some(ref email) = update.email {
            if !self.is_valid_email(email) {
                return Err(UserServiceError::InvalidData {
                    reason: "Invalid email format".to_string(),
                });
            }
        }
        
        // Update user
        let updated_user = self.repository
            .update(user_id, update)
            .await?
            .ok_or_else(|| UserServiceError::UserNotFound {
                id: user_id.to_string(),
            })?;
        
        // Publish event
        self.event_publisher.publish(UserEvent::ProfileUpdated {
            user_id: user_id.to_string(),
            timestamp: chrono::Utc::now(),
        }).await?;
        
        Ok(updated_user)
    }
    
    pub async fn search_users(&self, criteria: UserSearchCriteria) -> Result<Vec<User>> {
        // Apply business logic filters
        let mut enhanced_criteria = criteria;
        enhanced_criteria.active_only = true;  // Only return active users
        
        let users = self.repository.find_by_criteria(enhanced_criteria).await?;
        
        // Apply post-processing if needed
        Ok(users)
    }
    
    fn is_valid_email(&self, email: &str) -> bool {
        // Simple email validation
        email.contains('@') && email.contains('.')
    }
}

#[derive(Debug)]
pub enum UserEvent {
    ProfileUpdated { user_id: String, timestamp: chrono::DateTime<chrono::Utc> },
    UserCreated { user_id: String, timestamp: chrono::DateTime<chrono::Utc> },
    UserDeleted { user_id: String, timestamp: chrono::DateTime<chrono::Utc> },
}

#[async_trait]
pub trait EventPublisher {
    async fn publish(&self, event: UserEvent) -> Result<()>;
}
```

## Testing Strategies

### Unit Testing with Mock Collections

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    
    mock! {
        UserRepo {}
        
        #[async_trait]
        impl UserRepository for UserRepo {
            async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
            async fn find_by_criteria(&self, criteria: UserSearchCriteria) -> Result<Vec<User>>;
            async fn create(&self, user: User) -> Result<User>;
            async fn update(&self, id: &str, update: UserUpdate) -> Result<Option<User>>;
            async fn delete(&self, id: &str) -> Result<bool>;
        }
    }
    
    #[tokio::test]
    async fn test_user_service_update_profile() {
        let mut mock_repo = MockUserRepo::new();
        let mut mock_publisher = MockEventPublisher::new();
        
        // Setup expectations
        mock_repo
            .expect_update()
            .with(eq("user123"), any())
            .times(1)
            .returning(|_, _| Ok(Some(User {
                id: "user123".to_string(),
                name: "Updated Name".to_string(),
                email: "updated@example.com".to_string(),
                age: 30,
                is_active: true,
                created_at: chrono::Utc::now(),
            })));
        
        mock_publisher
            .expect_publish()
            .times(1)
            .returning(|_| Ok(()));
        
        let service = UserService::new(
            Box::new(mock_repo),
            Box::new(mock_publisher),
        );
        
        let update = UserUpdate {
            name: Some("Updated Name".to_string()),
            email: Some("updated@example.com".to_string()),
            age: None,
        };
        
        let result = service.update_user_profile("user123", update).await;
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.name, "Updated Name");
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::*;
    use testcontainers_modules::mongo::Mongo;
    
    #[tokio::test]
    async fn test_user_repository_integration() {
        // Start MongoDB container
        let docker = clients::Cli::default();
        let mongo_container = docker.run(Mongo::default());
        let mongo_port = mongo_container.get_host_port_ipv4(27017);
        
        // Connect to test database
        let connection_string = format!("mongodb://localhost:{}", mongo_port);
        let client = mongodb::Client::with_uri_str(&connection_string).await.unwrap();
        let database = client.database("test_db");
        
        let repository = MongoUserRepository::new(&database);
        
        // Test user creation
        let user = User {
            id: "test_user".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            age: 25,
            is_active: true,
            created_at: chrono::Utc::now(),
        };
        
        let created_user = repository.create(user.clone()).await.unwrap();

        assert_eq!(created_user.name, user.name);
        
        // Test user retrieval
        let found_user = repository.find_by_id("test_user").await.unwrap();

        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().name, "Test User");
        
        // Test user update
        let update = UserUpdate {
            name: Some("Updated Test User".to_string()),
            email: None,
            age: None,
        };
        
        let updated_user = repository.update("test_user", update).await.unwrap();

        assert!(updated_user.is_some());
        assert_eq!(updated_user.unwrap().name, "Updated Test User");
    }
}
```

## Production Considerations

### Monitoring and Metrics

```rust
use prometheus::{Counter, Histogram, Opts, Registry};

pub struct UserServiceMetrics {
    pub queries_total: Counter,
    pub query_duration: Histogram,
    pub errors_total: Counter,
}

impl UserServiceMetrics {
    pub fn new(registry: &Registry) -> Result<Self, prometheus::Error> {
        let queries_total = Counter::with_opts(
            Opts::new("user_queries_total", "Total number of user queries")
        )?;
        
        let query_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "user_query_duration_seconds",
                "Duration of user queries in seconds"
            )
        )?;
        
        let errors_total = Counter::with_opts(
            Opts::new("user_errors_total", "Total number of user service errors")
        )?;
        
        registry.register(Box::new(queries_total.clone()))?;
        registry.register(Box::new(query_duration.clone()))?;
        registry.register(Box::new(errors_total.clone()))?;
        
        Ok(Self {
            queries_total,
            query_duration,
            errors_total,
        })
    }
}

// Usage in service
impl UserService {
    pub async fn monitored_find_user(&self, id: &str) -> Result<Option<User>> {
        let _timer = self.metrics.query_duration.start_timer();

        self.metrics.queries_total.inc();
        
        match self.repository.find_by_id(id).await {
            Ok(user) => Ok(user),
            Err(error) => {
                self.metrics.errors_total.inc();
                Err(error)
            }
        }
    }
}
```

### Configuration Management

```rust
use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub database_name: String,
    pub max_pool_size: u32,
    pub min_pool_size: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("APP").separator("_"));
        
        if let Ok(env) = std::env::var("ENVIRONMENT") {
            config = config.add_source(File::with_name(&format!("config/{}", env)).required(false));
        }
        
        config.build()?.try_deserialize()
    }
}
```

## Migration Strategies

### Schema Evolution

```rust
// V1 User struct
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct UserV1 {
    pub id: String,
    pub name: String,
    pub email: String,
}

// V2 User struct with additional fields
#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
pub struct UserV2 {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: Option<i32>,        // New optional field
    pub is_active: bool,         // New required field with default
    pub version: i32,            // Version tracking
}

// Migration service
pub struct UserMigrationService {
    collection: mongodb::Collection<bson::Document>,
}

impl UserMigrationService {
    pub async fn migrate_v1_to_v2(&self) -> Result<()> {
        let filter = doc! { "version": { "$exists": false } };  // V1 documents
        
        let update = doc! {
            "$set": {
                "is_active": true,
                "version": 2,
                "migrated_at": chrono::Utc::now()
            }
        };
        
        let result = self.collection.update_many(filter, update, None).await?;
        println!("Migrated {} documents from V1 to V2", result.modified_count);
        
        Ok(())
    }
    
    pub async fn validate_migration(&self) -> Result<bool> {
        let v1_count = self.collection
            .count_documents(doc! { "version": { "$exists": false } }, None)
            .await?;
        
        let v2_count = self.collection
            .count_documents(doc! { "version": 2 }, None)
            .await?;
        
        println!("V1 documents remaining: {}", v1_count);
        println!("V2 documents: {}", v2_count);
        
        Ok(v1_count == 0)
    }
}
```

### Data Consistency Patterns

```rust
use mongodb::options::{ReadConcern, WriteConcern, Acknowledgment};

pub async fn ensure_data_consistency(
    user_collection: &mongodb::Collection<User>,
    audit_collection: &mongodb::Collection<AuditLog>,
    user_id: &str,
    new_email: String,
) -> Result<()> {
    
    // Start a session for transaction
    let mut session = user_collection.client().start_session(None).await?;
    
    // Start transaction
    session.start_transaction(None).await?;
    
    // Update user with session
    let user_filter = doc! { "_id": user_id };
    let user_update = tnuctipun::updates::empty::<User>()
        .set::<user_fields::Email, _>(new_email.clone())
        .set::<user_fields::LastModified, _>(chrono::Utc::now())
        .build();
    
    let user_result = user_collection
        .update_one_with_session(user_filter, user_update, None, &mut session)
        .await?;
    
    if user_result.modified_count == 0 {
        session.abort_transaction().await?;
        return Err(UserServiceError::UserNotFound {
            id: user_id.to_string(),
        });
    }
    
    // Create audit log with session
    let audit_log = AuditLog {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        action: "email_updated".to_string(),
        old_value: None,  // Would need to fetch this
        new_value: Some(new_email),
        timestamp: chrono::Utc::now(),
    };
    
    audit_collection
        .insert_one_with_session(audit_log, None, &mut session)
        .await?;
    
    // Commit transaction
    session.commit_transaction().await?;
    
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Best Practices Summary

1. **Performance**: Use projections, batch operations, and appropriate indexes
2. **Error Handling**: Implement comprehensive error types and retry patterns
3. **Testing**: Use both unit tests with mocks and integration tests with real databases
4. **Architecture**: Implement repository and service patterns for clean separation
5. **Monitoring**: Add metrics and logging for production visibility
6. **Configuration**: Use environment-based configuration management
7. **Migration**: Plan for schema evolution and data consistency
8. **Type Safety**: Leverage Tnuctipun's compile-time validation throughout your application

These patterns will help you build robust, maintainable applications with Tnuctipun in production environments.
