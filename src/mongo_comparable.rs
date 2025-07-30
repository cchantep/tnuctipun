// MongoComparable provides evidence that a value of type A can be compared with a value of type B
// in MongoDB queries. This trait is used at the struct level to indicate that specific fields
// can be compared with specific types of values in MongoDB operations.
pub trait MongoComparable<A, B> {}
