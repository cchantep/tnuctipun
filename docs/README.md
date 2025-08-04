---
title: Tnuctipun User Guide
layout: home
nav_order: 1
---

# Tnuctipun User Guide

Welcome to the comprehensive user guide for **Tnuctipun**, a type-safe MongoDB query and update builder library for Rust.

## Quick Navigation

### 📚 User Guide

- [**Introduction & Motivation**](user-guide/01-introduction.md) - Project overview and why Tnunctipun exists
- [**Getting Started**](user-guide/02-getting-started.md) - Installation and basic setup
- [**Finding Documents**](user-guide/03-finding-documents.md) - Query building with filters and projections
- [**Updating Documents**](user-guide/04-updating-documents.md) - Type-safe update operations
- [**Derive Macros**](user-guide/05-derive-macros.md) - Detailed documentation on procedural macros
- [**Advanced Topics**](user-guide/06-advanced-topics.md) - Complex scenarios and best practices

### 🔗 Additional Resources

- [**API Documentation**](api/tnuctipun/) - Complete API reference (auto-generated)
- [**Crates.io**](https://crates.io/crates/tnuctipun) - Released versions
- [**GitHub Repository**](https://github.com/cchantep/tnuctipun) - Source code and issues

## What is Tnuctipun?

Tnuctipun is a Rust library that provides **type-safe builders** for MongoDB operations. It allows you to:

- Build complex queries with **compile-time field validation**
- Create projections with **fluent method chaining**
- Construct update documents with **type-safe field operations**
- Use **derive macros** to automatically generate field witnesses

## Key Benefits

- **Compile-time Safety**: Catch field name typos and type mismatches at compile time
- **Fine-grained Control**: Build queries at the field and operation level (unlike full-document approaches)
- **MongoDB Integration**: Works seamlessly with the official MongoDB Rust driver
- **Zero Runtime Overhead**: All validation happens at compile time

## Quick Example

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}

// Type-safe filter building with method chaining
let filter_doc = empty::<User>()
    .eq::<user_fields::Name, _>("John".to_string())
    .gt::<user_fields::Age, _>(18)
    .and();
// Results in: { "$and": [{ "name": "John" }, { "age": { "$gt": 18 } }] }
```

Start with the [Introduction](user-guide/01-introduction.md) to learn more about the project's motivation and core concepts.
