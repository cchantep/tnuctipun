//! Modular projection tests
//!
//! This module organizes projection tests into logical groups:
//! - test_fixtures: Shared test data structures and field witnesses
//! - basic_operations: Core projection operations (includes, excludes, project)
//! - builder_functionality: ProjectionBuilder patterns and method chaining
//! - nested_operations: Nested field projections using with_lookup
//! - advanced_features: Complex projection scenarios and edge cases
//! - integration: Comprehensive tests combining multiple features

pub mod advanced_features;
pub mod basic_operations;
pub mod builder_functionality;
pub mod integration;
pub mod nested_operations;
pub mod test_fixtures;

// Re-export test fixtures for easy access across modules
pub use test_fixtures::*;
