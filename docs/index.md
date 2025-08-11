---
title: Home
layout: page
description: "Tnuctipun - Type-safe MongoDB query and update builder for Rust"
permalink: /
---

<img src="https://repository-images.githubusercontent.com/1030517113/b428d5ff-e9b3-4ae4-a3e7-77979debc7b0" alt="Tnuctipun Logo" width="600" style="border: 1px solid #ddd; border-radius: 8px;" />

**Type-safe MongoDB query and update builder for Rust**

Welcome to the comprehensive user guide for **Tnunctipun**, a type-safe MongoDB query and update builder library for Rust.

## Quick Navigation

### 📚 User Guide

- [**Introduction & Motivation**](user-guide/01-introduction.md) - Project overview and why Tnuctipun exists
- [**Getting Started**](user-guide/02-getting-started.md) - Installation and basic setup
- [**Finding Documents**](user-guide/03-finding-documents.md) - Query building with filters and projections
- [**Updating Documents**](user-guide/04-updating-documents.md) - Type-safe update operations
- [**Derive Macros**](user-guide/05-derive-macros.md) - Automatic trait implementations
- [**Advanced Topics**](user-guide/06-advanced-topics.md) - Performance tuning and edge cases

### 🔧 Reference Documentation

- [**API Docs.rs**](https://docs.rs/tnuctipun/latest/tnuctipun/) - Complete API documentation
- [**Crates.io**](https://crates.io/crates/tnuctipun) - Released versions

### 📖 Examples

- [**Basic Usage**](examples/basic-usage.md) - Simple queries and updates
- [**Complex Filters**](examples/complex-filters.md) - Advanced filtering patterns
- [**Aggregation Pipelines**](examples/aggregation-pipelines.md) - Complex data processing
- [**Real-World Scenarios**](examples/real-world-scenarios.md) - Production use cases

## What is Tnuctipun?

Tnuctipun is a compile-time type-safe MongoDB query and update builder for Rust. It provides:

- **Type Safety**: Compile-time guarantees that your queries and updates are valid
- **Zero Runtime Cost**: No performance overhead compared to manual query building
- **Ergonomic API**: Intuitive, chainable interface for building complex queries
- **Field Validation**: Ensures referenced fields actually exist in your structs
- **Update Safety**: Prevents invalid update operations at compile time

## Getting Started

Add Tnunctipun to your `Cargo.toml`:

```toml
[dependencies]
tnuctipun = "0.1.1"
```

Then start building type-safe MongoDB queries:

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
}

// Type-safe query building
let filter = empty::<User>()
    .eq::<user_fields::Name, _>("Alice".to_string())
    .gte::<user_fields::Age, _>(18)
    .and();

// Type-safe updates
let update = updates::empty::<User>()
    .set::<user_fields::Active, _>(true)
    .inc::<user_fields::Age, _>(1)
    .build();
```

## Community & Support

- **GitHub Repository**: [cchantep/tnuctipun](https://github.com/cchantep/tnuctipun)
- **Documentation**: This user guide and [API documentation](https://docs.rs/tnuctipun/latest/tnuctipun/)
- **Issues**: Report bugs or request features on [GitHub Issues](https://github.com/cchantep/tnunctipun/issues)
