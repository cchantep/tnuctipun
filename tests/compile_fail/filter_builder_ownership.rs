// This test verifies that storing an intermediate filter builder in a variable
// and trying to use it later will cause ownership/borrowing compile errors

use tnuctipun::{FieldWitnesses, MongoComparable};
use tnuctipun::filters::empty;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
}

fn main() {
    // This pattern will cause ownership issues - storing intermediate builder
    let filter_builder = empty::<User>()
        .eq::<user_fields::Name, _>("John".to_string())
        .gt::<user_fields::Age, _>(18);

    // This should fail to compile due to ownership issues
    let filter_doc = filter_builder.and(); // Error: value used after move
}
