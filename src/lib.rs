//! # Tnuctipun
//!
//! <img src="https://repository-images.githubusercontent.com/1030517113/b428d5ff-e9b3-4ae4-a3e7-77979debc7b0" alt="Tnuctipun Logo" width="600" />
//!
//! The Tnuctipun of Ringworld — ancient, subversive, ingenious — or a type-safe MongoDB builder library.
//!
//! ## Features
//!
//! - **Type-safe field access**: Use compile-time validated field names
//! - **MongoDB query building**: Build complex queries with type safety
//! - **MongoDB projection building**: Create projections with fluent method chaining
//! - **MongoDB update building**: Create update documents with type-safe field operations
//! - **Derive macros**: Automatically generate field witnesses and comparable traits
//! - **Compile-time validation**: Catch field name typos and type mismatches at compile time
//!
//! ## Example
//!
//! ```rust
//! use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
//! struct User {
//!     pub name: String,
//!     pub age: i32,
//!     pub email: String,
//! }
//!
//! // Type-safe filter building with compile-time field validation
//! let mut filter_builder = empty::<User>();
//!
//! filter_builder.eq::<user_fields::Name, _>("John".to_string());
//! filter_builder.gt::<user_fields::Age, _>(18);
//!
//! // Convert to MongoDB document
//! let filter_doc = filter_builder.and();
//! ```

// Modules
pub mod field_filters;
pub mod field_witnesses;
pub mod filters;
pub mod mongo_comparable;
pub mod path;
pub mod projection;
pub mod updates;

// Re-export the procedural macros
pub use tnuctipun_derive::FieldWitnesses;
pub use tnuctipun_derive::MongoComparable;

// Export the traits
pub use crate::field_filters::FieldFilterBuilder;
pub use crate::field_witnesses::{FieldName, HasField, NonEmptyStruct};
pub use crate::mongo_comparable::MongoComparable;
pub use crate::path::Path;
