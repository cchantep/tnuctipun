---
title: Introduction & Motivation
layout: page
nav_exclude: true
---

## What is Tnuctipun?

Tnuctipun is a **type-safe MongoDB query and update builder library** for Rust.

> The name comes from the Tnuctipun of Ringworld — ancient, subversive, ingenious — reflecting the library's approach to providing elegant solutions to MongoDB query construction.

## Project Overview

Tnuctipun provides a comprehensive suite of tools for building MongoDB operations with compile-time safety:

- **Type-safe field access**: Use compile-time validated field names
- **MongoDB query building**: Build complex queries with type safety
- **MongoDB projection building**: Create projections with fluent method chaining
- **MongoDB update building**: Create update documents with type-safe field operations
- **Derive macros**: Automatically generate field witnesses and comparable traits
- **Compile-time validation**: Catch field name typos and type mismatches at compile time

## The Problem: Limited Field-Level Control

The official MongoDB Rust driver provides excellent macros like `doc!` for creating documents, but these approaches have limitations:

### Full-Document Approach Limitations

```rust
// Official MongoDB driver approach
let filter = doc! {
    "name": "John",        // ❌ No compile-time validation
    "agee": { "$gt": 18 }  // ❌ Typo won't be caught until runtime
};

// Struct-based insertion/loading
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

// Works great for full document operations
let user = User { name: "John".to_string(), age: 25 };
// collection.insert_one(user, None).await?;
```

**Issues with this approach:**

- ✅ **Good for**: Full document insertion and loading
- ❌ **Limited for**: Field-level queries and partial updates
- ❌ **No validation**: Field names are strings, typos cause runtime errors
- ❌ **No type safety**: Wrong types for field values aren't caught
- ❌ **Maintenance burden**: Renaming struct fields doesn't update queries

### Real-World Scenarios Requiring Finer Control

1. **Complex Filtering**: Building queries with multiple conditions, logical operators
2. **Partial Projections**: Selecting only specific fields, hiding sensitive data
3. **Granular Updates**: Setting specific fields, incrementing counters, array operations
4. **Dynamic Queries**: Building queries conditionally based on runtime parameters
5. **Aggregation Pipelines**: Using filters, projections or updates (e.g. in `$match`, `$project` or `$set` stages)

## The Tnuctipun Solution

Tnuctipun addresses these limitations by providing **type-safe builders** that operate at the field and operation level:

### Compile-Time Field Validation

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};

#[derive(FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}

let mut filter = empty::<User>();

filter.eq::<user_fields::Name, _>("John".to_string());   // ✅ Compile-time validated
filter.gt::<user_fields::Age, _>(18);                    // ❌ Compile error - field doesn't exist
```

### Type-Safe Operations

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};

#[derive(FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}

// Type safety for field values
let mut filter = empty::<User>();

// filter.eq::<user_fields::Age, _>("not a number");  // ❌ Compile error - wrong type
filter.eq::<user_fields::Age, _>(25);              // ✅ Correct type
```

### Fine-Grained Control

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}

// Complex filtering
let _filter_doc = empty::<User>()
    .eq::<user_fields::Name, _>("John".to_string())
    .gt::<user_fields::Age, _>(18)
    .ne::<user_fields::Email, _>("".to_string())
    .and();

// Selective projections
let _projection_doc = projection::empty::<User>()
    .includes::<user_fields::Name>()
    .includes::<user_fields::Age>()
    .excludes::<user_fields::Email>()  // Hide sensitive data
    .build();

// Granular updates
let _update_doc = updates::empty::<User>()
    .set::<user_fields::Name, _>("Jane".to_string())
    .inc::<user_fields::Age, _>(1)
    .unset::<user_fields::Email>()
    .build();

// Example of using with MongoDB collection
let _user = User {
    name: "John".to_string(),
    age: 25,
    email: "john@example.com".to_string(),
};
// collection.insert_one(user, None).await?;
```

## Key Advantages

### 1. **Compile-Time Safety**

- Field names are validated at compile time
- Type mismatches are caught before runtime
- Refactoring struct fields automatically updates all usages

### 2. **MongoDB Driver Integration**

- Generated documents work seamlessly with the official MongoDB Rust driver
- No runtime overhead - all validation happens at compile time
- Compatible with both direct queries and aggregation pipelines

### 3. **Developer Experience**

- IDE autocomplete for field names
- Clear compile errors for invalid operations
- Fluent, chainable API for building complex operations

### 4. **Maintainability**

- Changing struct field names automatically updates all query builders
- Type changes are enforced across the entire codebase
- No string-based field references to maintain

## Use Cases

Tnuctipun excels in scenarios requiring:

1. **Complex Query Logic**: Multiple conditions, nested boolean logic
2. **Dynamic Filtering**: Building queries based on runtime conditions
3. **Data Privacy**: Selective field projection to hide sensitive information
4. **Partial Updates**: Updating only specific fields while preserving others
5. **Aggregation Pipelines**: Type-safe stage construction
6. **API Development**: Building flexible query endpoints with compile-time safety

## Next Steps

Now that you understand the motivation behind Tnuctipun, let's get started:

- [**Getting Started**](02-getting-started.md) - Installation and basic setup
- [**Finding Documents**](03-finding-documents.md) - Learn to build queries and projections
- [**Updating Documents**](04-updating-documents.md) - Master update operations
