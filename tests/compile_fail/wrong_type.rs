// This test verifies that using the wrong type will cause a compile error

use nessus::{FieldName, FieldWitnesses, HasField};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
struct Product {
    Price: f64,
}

pub fn test_wrong_type<F, T, V>(value: V) -> (&'static str, String) 
where
    F: FieldName,
    T: HasField<F, Value = V>,
    V: ToString
{
    (F::field_name(), value.to_string())
}

fn main() {
    // This should fail to compile because we're trying to use a string for a f64 field
    let price_filter = test_wrong_type::<product_fields::Price, Product, _>("wrong type".to_string());
}
