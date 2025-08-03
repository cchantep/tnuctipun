// Modules
pub mod field_filters;
pub mod field_witnesses;
pub mod filters;
pub mod mongo_comparable;
pub mod path;
pub mod projection;
pub mod updates;

// Re-export the procedural macros
pub use nessus_derive::FieldWitnesses;
pub use nessus_derive::MongoComparable;

// Export the traits
pub use crate::field_filters::FieldFilterBuilder;
pub use crate::field_witnesses::{FieldName, HasField, NonEmptyStruct};
pub use crate::mongo_comparable::MongoComparable;
pub use crate::path::Path;
