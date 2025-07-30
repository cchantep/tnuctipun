// This test verifies that primitive types do NOT implement NonEmptyStruct

use nessus::NonEmptyStruct;

fn main() {
    // This should fail to compile because String does not implement NonEmptyStruct
    fn _assert_implements_non_empty_struct<T: NonEmptyStruct>() {}
    _assert_implements_non_empty_struct::<String>();
}
