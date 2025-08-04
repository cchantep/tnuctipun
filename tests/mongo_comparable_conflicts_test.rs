#![allow(non_camel_case_types)]

use tnuctipun::{FieldWitnesses, MongoComparable};

// Test potential MongoComparable conflicts
mod user_management {
    use super::*;
    #[derive(Debug, Clone, FieldWitnesses, MongoComparable)]
    pub struct User {
        pub name: String,
        pub age: i32,
    }

    #[derive(Debug, Clone, FieldWitnesses, MongoComparable)]
    pub struct Product {
        pub name: String, // Same type as User::name
        pub price: f64,
    }
}

mod admin_panel {
    use super::*;

    #[derive(Debug, Clone, FieldWitnesses, MongoComparable)]
    pub struct User {
        pub name: String, // Same type as user_management::User::name
        pub permissions: Vec<String>,
    }

    #[derive(Debug, Clone, FieldWitnesses, MongoComparable)]
    pub struct Settings {
        pub name: String, // Same type again
        pub value: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tnuctipun::mongo_comparable::MongoComparable;

    #[test]
    fn test_mongo_comparable_no_conflicts() {
        // This test will demonstrate if MongoComparable implementations conflict
        // when multiple structs have fields of the same type

        // All these structs should implement MongoComparable<String, String>
        // Let's verify they all compile and work independently

        // Test that each struct properly implements MongoComparable for String fields
        fn check_mongo_comparable<T>()
        where
            T: MongoComparable<String, String>,
        {
            // This function will only compile if T implements MongoComparable<String, String>
        }

        // All of these should compile without conflicts
        check_mongo_comparable::<user_management::User>();
        check_mongo_comparable::<user_management::Product>();
        check_mongo_comparable::<admin_panel::User>();
        check_mongo_comparable::<admin_panel::Settings>();

        // Test i32 compatibility
        fn check_i32_comparable<T>()
        where
            T: MongoComparable<i32, i32>,
        {
            // This function will only compile if T implements MongoComparable<i32, i32>
        }

        check_i32_comparable::<user_management::User>();
        // check_i32_comparable::<user_management::Product>(); // Should not compile - Product has no i32 fields

        // If this test compiles, MongoComparable implementations don't conflict
    }

    #[test]
    fn test_mongo_comparable_cross_compatibility() {
        // Test that MongoComparable implementations are properly scoped to their structs
        // and don't interfere with each other

        fn test_string_compatibility<T: MongoComparable<String, String>>(_: T) {}
        fn test_i32_compatibility<T: MongoComparable<i32, i32>>(_: T) {}
        fn test_vec_compatibility<T: MongoComparable<Vec<String>, String>>(_: T) {}

        let user_mgmt_user = user_management::User {
            name: "Alice".to_string(),
            age: 30,
        };

        let admin_user = admin_panel::User {
            name: "Bob".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
        };

        // These should all work without conflicts
        test_string_compatibility(user_mgmt_user.clone());
        test_i32_compatibility(user_mgmt_user);

        test_string_compatibility(admin_user.clone());
        test_vec_compatibility(admin_user);

        // MongoComparable works correctly across different struct types
    }
}
