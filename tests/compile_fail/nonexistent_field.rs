// This test verifies that using a nonexistent field will cause a compile error

use nessus::field_witnesses::{FieldName, HasField};
use nessus::FieldWitnesses;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Product {
    pub Name: String,
    pub Price: f64,
}

// Define a field witness for a nonexistent field
#[allow(non_camel_case_types)]
struct Weight;

impl FieldName for Weight {
    fn field_name() -> &'static str { "Weight" }
}

pub fn test_nonexistent<F, T, V>(value: V) -> (&'static str, String) 
where
    F: FieldName,
    T: HasField<F, Value = V>,
    V: ToString
{
    (F::field_name(), value.to_string())
}

fn main() {
    // This should fail to compile because the Product struct doesn't have a Weight field
    let weight_filter = test_nonexistent::<Weight, Product, _>(10.5);
}
