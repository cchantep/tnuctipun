//! Projection Tests
//!
//! This module has been refactored into smaller, more focused modules.
//! Tests are now organized in the `projections/` directory:
//!
//! - `test_fixtures`: Shared test data structures and field witnesses
//! - `basic_operations`: Core projection operations (includes, excludes, project)
//! - `builder_functionality`: ProjectionBuilder patterns and method chaining
//! - `nested_operations`: Nested field projections using with_lookup
//! - `advanced_features`: Complex projection scenarios and edge cases
//! - `integration`: Comprehensive tests combining multiple features

mod projections;

// Re-export all tests from the modular structure for backwards compatibility
pub use projections::*;
