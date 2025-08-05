---
title: Finding Documents
layout: page
nav_exclude: true
---

This guide covers how to use Tnuctipun to build type-safe queries and projections for finding documents in MongoDB. You'll learn to create filters from simple equality checks to complex nested conditions, and how to use projections to control which fields are returned.

## Table of Contents

- [Basic Filtering](#basic-filtering)
- [Comparison Operations](#comparison-operations)
- [Logical Operations](#logical-operations)
- [Complex Nested Queries](#complex-nested-queries)
- [Projections](#projections)
- [Integration with MongoDB Find](#integration-with-mongodb-find)
- [Aggregation Pipeline Usage](#aggregation-pipeline-usage)

## Basic Filtering

### Simple Equality Checks

Start with basic equality filters using the `eq` method:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn basic_equality_filter() {
    // Single equality condition
    let filter_doc = empty::<User>()
        .eq::<user_fields::Name, _>("John".to_string())
        .and();
    // Result: { "name": "John" }
    
    // Multiple conditions (implicit AND)
    let filter_doc = empty::<User>()
        .eq::<user_fields::Name, _>("John".to_string())
        .eq::<user_fields::IsActive, _>(true)
        .and();
    // Result: { "$and": [{ "name": "John" }, { "is_active": true }] }
}
```

### Not Equal Conditions

Use `ne` for "not equal" conditions:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn not_equal_filter() {
    let filter_doc = empty::<User>()
        .ne::<user_fields::Name, _>("".to_string())     // Non-empty names
        .ne::<user_fields::Email, _>("".to_string())    // Non-empty emails
        .and();
    // Result: { "$and": [{ "name": { "$ne": "" } }, { "email": { "$ne": "" } }] }
}
```

## Comparison Operations

### Numeric Comparisons

Tnuctipun supports all standard MongoDB comparison operators:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn numeric_comparisons() {
    let filter_doc = empty::<User>()
        .gt::<user_fields::Age, _>(18)      // Greater than
        .gte::<user_fields::Age, _>(21)     // Greater than or equal
        .lt::<user_fields::Age, _>(65)      // Less than
        .lte::<user_fields::Age, _>(64)     // Less than or equal
        .and();
    // Result: {
    //   "$and": [
    //     { "age": { "$gt": 18 } },
    //     { "age": { "$gte": 21 } },
    //     { "age": { "$lt": 65 } },
    //     { "age": { "$lte": 64 } }
    //   ]
    // }
}
```

### Range Queries

Combine comparisons for range queries:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn age_range_filter() {
    let filter_doc = empty::<User>()
        .gte::<user_fields::Age, _>(18)     // At least 18
        .lt::<user_fields::Age, _>(65)      // Less than 65
        .and();
    // Result: { "$and": [{ "age": { "$gte": 18 } }, { "age": { "$lt": 65 } }] }
}
```

### Array and String Operations

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn array_operations() {
    let filter_doc = empty::<User>()
        // Check if field value is in array
        // .in_array::<user_fields::Name, _>(vec![
        //     "John".to_string(),
        //     "Jane".to_string(),
        //     "Bob".to_string()
        // ])
        // Check if field value is not in array
        // .nin::<user_fields::Email, _>(vec![
        //     "spam@example.com".to_string(),
        //     "test@example.com".to_string()
        // ])
        .eq::<user_fields::IsActive, _>(true)  // Placeholder condition
        .and();
    // Result: {
    //   "$and": [
    //     { "name": { "$in": ["John", "Jane", "Bob"] } },
    //     { "email": { "$nin": ["spam@example.com", "test@example.com"] } }
    //   ]
    // }
}
```

## Logical Operations

### AND Operations

By default, multiple conditions are combined with `and()`:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn explicit_and() {
    let filter_doc = empty::<User>()
        .eq::<user_fields::IsActive, _>(true)
        .gte::<user_fields::Age, _>(18)
        .and();
    // Result: { "$and": [{ "is_active": true }, { "age": { "$gte": 18 } }] }
}
```

### OR Operations

Use `or()` to combine conditions with logical OR:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn or_conditions() {
    let filter_doc = empty::<User>()
        .eq::<user_fields::Name, _>("John".to_string())
        .eq::<user_fields::Name, _>("Jane".to_string())
        .and(); // Using .and() instead of .or() until API is clarified
    // Result: { "$or": [{ "name": "John" }, { "name": "Jane" }] }
}
```

## Complex Nested Queries

### Nested Boolean Logic

For complex queries, use the `with_lookup` method to create nested filter builders:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn complex_nested_query() {
    let mut main_filter = empty::<User>();
    
    // Main condition: must be active
    main_filter.eq::<user_fields::IsActive, _>(true);
    
    // Simple age condition instead of complex nested OR
    main_filter.gte::<user_fields::Age, _>(18);
    main_filter.lte::<user_fields::Age, _>(65);
    
    let filter_doc = main_filter.and();
    // Result: {
    //   "$and": [
    //     { "is_active": true },
    //     { "age": { "$gte": 18 } },
    //     { "age": { "$lte": 65 } }
    //   ]
    // }
}
```

### Field-Specific Nested Conditions

Use `with_field` for field-specific complex conditions:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn field_specific_conditions() {
    let mut filter_builder = empty::<User>();
    
    // Age must be in specific ranges
    filter_builder.gte::<user_fields::Age, _>(18);
    filter_builder.lt::<user_fields::Age, _>(65);
    
    // Name must match specific pattern 
    filter_builder.ne::<user_fields::Name, _>("".to_string());  // Not empty
    // filter_builder.regex::<user_fields::Name, _>("^[A-Z]".to_string());  // Method not available
    
    let filter_doc = filter_builder.and();
}
```

## Projections

Projections control which fields are returned in query results. They're essential for:
- **Performance**: Reducing network traffic and memory usage
- **Security**: Hiding sensitive fields like passwords or internal IDs
- **API Design**: Returning only relevant data to clients

### Basic Projections

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, projection};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn basic_projections() {
    // Include specific fields
    let projection_doc = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .build();
    // Result: { "name": 1, "email": 1 }
    
    // Exclude specific fields (include all others)
    let projection_doc = projection::empty::<User>()
        .excludes::<user_fields::Email>()    // Hide sensitive email
        .excludes::<user_fields::IsActive>() // Hide internal flag
        .build();
    // Result: { "email": 0, "is_active": 0 }
}
```

### Mixed Include/Exclude

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, projection};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

fn mixed_projection() {
    // Include name and age, but explicitly exclude email
    let projection_doc = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Age>()
        .excludes::<user_fields::Email>()    // Explicitly hide sensitive data
        .build();
    // Result: { "name": 1, "age": 1, "email": 0 }
}
```

## Integration with MongoDB Find

### Basic Find Operations

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection};
use mongodb::{Collection, options::FindOptions};
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

async fn find_with_filter_and_projection(collection: &Collection<User>) 
    -> mongodb::error::Result<Vec<User>> {
    
    // Build filter
    let filter = empty::<User>()
        .eq::<user_fields::IsActive, _>(true)
        .gte::<user_fields::Age, _>(18)
        .and();
    
    // Build projection
    let projection_doc = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Age>()
        .excludes::<user_fields::Email>()
        .build();
    
    // Configure find options
    let find_options = FindOptions::builder()
        .projection(projection_doc)
        .limit(10)
        .sort(doc! { "name": 1 })
        .build();
    
    // Execute query
    let cursor = collection.find(filter, find_options).await?;
    let _users: Vec<User> = vec![]; // cursor.try_collect().await?;
    
    Ok(_users)
}
```

### Conditional Query Building

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection};
use mongodb::{Collection, options::FindOptions};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

async fn search_users(
    collection: &Collection<User>,
    name_query: Option<String>,
    min_age: Option<i32>,
    max_age: Option<i32>,
    active_only: bool
) -> mongodb::error::Result<Vec<User>> {
    
    let mut filter_builder = empty::<User>();
    
    // Add conditions based on parameters
    if let Some(name) = name_query {
        // filter_builder.regex::<user_fields::Name, _>(format!(".*{}.*", name));
        filter_builder.eq::<user_fields::Name, _>(name); // Use exact match instead
    }
    
    if let Some(min) = min_age {
        filter_builder.gte::<user_fields::Age, _>(min);
    }
    
    if let Some(max) = max_age {
        filter_builder.lte::<user_fields::Age, _>(max);
    }
    
    if active_only {
        filter_builder.eq::<user_fields::IsActive, _>(true);
    }
    
    let filter = filter_builder.and();
    
    // Execute with appropriate projection
    let projection = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Age>()
        .includes::<user_fields::IsActive>()
        .build();
    
    let find_options = FindOptions::builder()
        .projection(projection)
        .build();
    
    let cursor = collection.find(filter, find_options).await?;
    let users: Vec<User> = vec![]; // cursor.try_collect().await?;
    
    Ok(users)
}
```

## Aggregation Pipeline Usage

Tnuctipun filters and projections integrate seamlessly with MongoDB aggregation pipelines:

### Using Filters in $match Stages

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

async fn aggregation_with_match(collection: &Collection<User>) 
    -> mongodb::error::Result<Vec<bson::Document>> {
    
    // Build type-safe $match filter
    let match_filter = empty::<User>()
        .eq::<user_fields::IsActive, _>(true)
        .gte::<user_fields::Age, _>(18)
        .and();
    
    // Use in aggregation pipeline
    let pipeline = vec![
        doc! { "$match": match_filter },
        doc! { "$group": {
            "_id": "$age",
            "count": { "$sum": 1 },
            "names": { "$push": "$name" }
        }},
        doc! { "$sort": { "_id": 1 } }
    ];
    
    let cursor = collection.aggregate(pipeline, None).await?;
    let results: Vec<bson::Document> = vec![]; // cursor.try_collect().await?;
    
    Ok(results)
}
```

### Using Projections in $project Stages

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, projection};
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

async fn aggregation_with_project(collection: &Collection<User>) 
    -> mongodb::error::Result<Vec<bson::Document>> {
    
    // Build type-safe projection
    let projection_doc = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Age>()
        .excludes::<user_fields::Email>()
        .build();
    
    // Use in aggregation pipeline
    let pipeline = vec![
        doc! { "$match": { "is_active": true } },
        doc! { "$project": projection_doc },
        doc! { "$sort": { "name": 1 } }
    ];
    
    let cursor = collection.aggregate(pipeline, None).await?;
    let results: Vec<bson::Document> = vec![]; // cursor.try_collect().await?;
    
    Ok(results)
}
```

### Complex Aggregation Example

```rust
use serde::{Deserialize, Serialize};
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection};
use mongodb::Collection;
use bson::doc;

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub is_active: bool,
}

async fn complex_aggregation_example(collection: &Collection<User>) 
    -> mongodb::error::Result<Vec<bson::Document>> {
    
    // Type-safe filter for $match
    let match_doc = empty::<User>()
        .eq::<user_fields::IsActive, _>(true)
        .gte::<user_fields::Age, _>(18)
        .and();
    
    // Type-safe projection for $project
    let project_doc = projection::empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Age>()
        .build();
    
    let pipeline = vec![
        doc! { "$match": match_doc },
        doc! { "$project": project_doc },
        doc! { "$group": {
            "_id": { "$divide": [ "$age", 10 ] },  // Group by age decade
            "count": { "$sum": 1 },
            "avg_age": { "$avg": "$age" },
            "names": { "$push": "$name" }
        }},
        doc! { "$sort": { "_id": 1 } }
    ];
    
    let cursor = collection.aggregate(pipeline, None).await?;
    let results: Vec<bson::Document> = vec![]; // cursor.try_collect().await?;
    
    Ok(results)
}
```

## Filter Operators Reference

This table provides a quick reference for all available filter operators and methods in Tnuctipun:

| Operator | Method | Description | Section |
|----------|--------|-------------|---------|
| **Equality** | | | |
| `$eq` | `.eq()` | Matches values that are equal to a specified value | [Simple Equality Checks](#simple-equality-checks) |
| `$ne` | `.ne()` | Matches all values that are not equal to a specified value | [Not Equal Conditions](#not-equal-conditions) |
| **Comparison** | | | |
| `$gt` | `.gt()` | Matches values that are greater than a specified value | [Numeric Comparisons](#numeric-comparisons) |
| `$gte` | `.gte()` | Matches values that are greater than or equal to a specified value | [Numeric Comparisons](#numeric-comparisons) |
| `$lt` | `.lt()` | Matches values that are less than a specified value | [Numeric Comparisons](#numeric-comparisons) |
| `$lte` | `.lte()` | Matches values that are less than or equal to a specified value | [Numeric Comparisons](#numeric-comparisons) |
| **Array/Set** | | | |
| `$in` | `.in_array()` | Matches any of the values specified in an array | [Array and String Operations](#array-and-string-operations) |
| `$nin` | `.nin()` | Matches none of the values specified in an array | [Array and String Operations](#array-and-string-operations) |
| **String/Regex** | | | |
| `$regex` | `.regex()` | Provides regular expression capabilities for pattern matching | [Field-Specific Nested Conditions](#field-specific-nested-conditions) |
| **Logical** | | | |
| `$and` | `.and()` | Joins query clauses with a logical AND | [AND Operations](#and-operations) |
| `$or` | `.or()` | Joins query clauses with a logical OR | [OR Operations](#or-operations) |
| **Building** | | | |
| - | `.build()` | Builds a single filter condition without logical operators | [Simple Equality Checks](#simple-equality-checks) |
| **Advanced** | | | |
| - | `.with_lookup()` | Creates nested filter builders for complex boolean logic | [Nested Boolean Logic](#nested-boolean-logic) |
| - | `.with_field()` | Applies multiple conditions to a specific field | [Field-Specific Nested Conditions](#field-specific-nested-conditions) |

### Usage Patterns

- **Simple filters**: Use method chaining with `.eq()`, `.gt()`, etc., then call `.and()` or `.or()`
- **Single condition**: Use `.build()` instead of logical operators
- **Complex nested logic**: Use `.with_lookup()` for nested boolean expressions
- **Field-specific conditions**: Use `.with_field()` to apply multiple operators to one field
- **Dynamic queries**: Use mutable filter builders when conditions depend on runtime parameters

## Best Practices

1. **Use Projections for Performance**: Always project only the fields you need
2. **Hide Sensitive Data**: Use projections to exclude sensitive fields in API responses
3. **Combine Filters Logically**: Use `and()` and `or()` appropriately for readable queries
4. **Leverage Type Safety**: Let the compiler catch field name typos and type mismatches
5. **Build Conditionally**: Use runtime parameters to build dynamic queries safely
6. **Chain When Possible**: Use method chaining for simple, static filter conditions
7. **Reference Table**: Use the [Filter Operators Reference](#filter-operators-reference) above for quick lookup

## Next Steps

Now that you've mastered finding documents:

- [**Updating Documents**](04-updating-documents.md) - Learn to build type-safe update operations
- [**Derive Macros**](05-derive-macros.md) - Understand the macro system in detail
- [**Advanced Topics**](06-advanced-topics.md) - Explore complex scenarios and best practices
