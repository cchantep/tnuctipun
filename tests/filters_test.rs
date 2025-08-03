//! Filter Tests
//!
//! This module has been refactored into smaller, more focused modules.
//! Tests are now organized in the `filters/` directory:
//!
//! - `test_fixtures`: Shared test data structures and field witnesses
//! - `basic_operations`: Core filter operations (eq, gt, lt, in, exists, etc.)
//! - `logical_operations`: Logical operations (and, or, not) and combinations
//! - `nested_operations`: Nested field filtering using with_lookup
//! - `operation_builder`: FilterBuilder patterns and method chaining
//! - `integration`: Comprehensive tests combining multiple features

mod filters {
    pub mod basic_operations;
    pub mod integration;
    pub mod logical_operations;
    pub mod nested_operations;
    pub mod operation_builder;
    pub mod test_fixtures;
}
