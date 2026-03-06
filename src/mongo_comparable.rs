/// Compile-time evidence that a value of type `A` can be compared with a value
/// of type `B` in MongoDB query operators.
///
/// This trait is implemented by the `MongoComparable` derive macro and used by
/// typed builders such as filters and expressions.
pub trait MongoComparable<A, B> {}

/// Compile-time evidence that a value of type `A` can be used with MongoDB
/// ordering operators against a value of type `B`.
///
/// This trait is intended for operators such as `$gt`, `$gte`, `$lt`, and
/// `$lte`. It is narrower than [`MongoComparable`] and allows APIs to enforce
/// stricter ordering semantics while keeping broader comparability for
/// operators like `$eq` and `$ne`.
pub trait MongoOrdered<A, B> {}
