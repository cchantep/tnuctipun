// This test verifies that a struct WITHOUT FieldWitnesses does NOT implement NonEmptyStruct

use nessus::NonEmptyStruct;
use serde::{Serialize, Deserialize};

// This struct does NOT derive FieldWitnesses
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegularStruct {
    Name: String,
    Age: i32,
}

fn main() {
    // This should fail to compile because RegularStruct does not derive FieldWitnesses
    // and therefore does not implement NonEmptyStruct
    fn _assert_implements_non_empty_struct<T: NonEmptyStruct>() {}
    _assert_implements_non_empty_struct::<RegularStruct>();
}
