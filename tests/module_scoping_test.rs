use tnuctipun::{FieldName, FieldWitnesses, HasField};
// Test that structs with the same name in different modules don't conflict
mod user_management {
    use super::*;

    #[derive(Debug, Clone, FieldWitnesses)]
    pub struct User {
        pub name: String,
        pub email: String,
        pub role: String,
    }

    #[derive(Debug, Clone, FieldWitnesses)]
    pub struct Product {
        pub name: String,
        pub price: f64,
    }
}

mod admin_panel {
    use super::*;

    #[derive(Debug, Clone, FieldWitnesses)]
    pub struct User {
        pub name: String,
        pub permissions: Vec<String>,
        pub admin_level: i32,
    }

    #[derive(Debug, Clone, FieldWitnesses)]
    pub struct Product {
        pub name: String,
        pub category: String,
    }
}

mod inventory {
    use super::*;

    #[derive(Debug, Clone, FieldWitnesses)]
    pub struct Product {
        pub name: String,
        pub stock: i32,
        pub warehouse_id: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_struct_names_different_modules_no_conflict() {
        // Create instances of structs with same names in different modules
        let user_mgmt_user = user_management::User {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            role: "manager".to_string(),
        };

        let admin_user = admin_panel::User {
            name: "Bob".to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
            admin_level: 5,
        };

        // Field witnesses should be scoped and not conflict
        assert_eq!(user_management::user_fields::Name::field_name(), "name");
        assert_eq!(user_management::user_fields::Email::field_name(), "email");
        assert_eq!(user_management::user_fields::Role::field_name(), "role");

        assert_eq!(admin_panel::user_fields::Name::field_name(), "name");
        assert_eq!(
            admin_panel::user_fields::Permissions::field_name(),
            "permissions"
        );
        assert_eq!(
            admin_panel::user_fields::AdminLevel::field_name(),
            "admin_level"
        );

        // HasField implementations should work correctly for each scoped type
        let user_mgmt_name = <user_management::User as HasField<
            user_management::user_fields::Name,
        >>::get_field(&user_mgmt_user);
        let admin_name =
            <admin_panel::User as HasField<admin_panel::user_fields::Name>>::get_field(&admin_user);

        assert_eq!(user_mgmt_name, "Alice");
        assert_eq!(admin_name, "Bob");

        // Access other fields to ensure they're properly scoped
        let user_mgmt_email = <user_management::User as HasField<
            user_management::user_fields::Email,
        >>::get_field(&user_mgmt_user);
        let admin_permissions = <admin_panel::User as HasField<
            admin_panel::user_fields::Permissions,
        >>::get_field(&admin_user);

        assert_eq!(user_mgmt_email, "alice@example.com");
        assert_eq!(
            admin_permissions,
            &vec!["read".to_string(), "write".to_string()]
        );
    }

    #[test]
    fn test_same_struct_and_field_names_multiple_modules() {
        // Test Products with same field names across multiple modules
        let user_mgmt_product = user_management::Product {
            name: "Office Chair".to_string(),
            price: 299.99,
        };

        let admin_product = admin_panel::Product {
            name: "Admin Software".to_string(),
            category: "Software".to_string(),
        };

        let inventory_product = inventory::Product {
            name: "Warehouse Scanner".to_string(),
            stock: 15,
            warehouse_id: "WH-001".to_string(),
        };

        // All three have "name" fields but they should be scoped differently
        assert_eq!(user_management::product_fields::Name::field_name(), "name");
        assert_eq!(admin_panel::product_fields::Name::field_name(), "name");
        assert_eq!(inventory::product_fields::Name::field_name(), "name");

        // Access the name field from each Product type
        let user_mgmt_name = <user_management::Product as HasField<
            user_management::product_fields::Name,
        >>::get_field(&user_mgmt_product);
        let admin_name =
            <admin_panel::Product as HasField<admin_panel::product_fields::Name>>::get_field(
                &admin_product,
            );
        let inventory_name =
            <inventory::Product as HasField<inventory::product_fields::Name>>::get_field(
                &inventory_product,
            );

        assert_eq!(user_mgmt_name, "Office Chair");
        assert_eq!(admin_name, "Admin Software");
        assert_eq!(inventory_name, "Warehouse Scanner");

        // Access unique fields from each Product type
        let price = <user_management::Product as HasField<
            user_management::product_fields::Price,
        >>::get_field(&user_mgmt_product);
        let category =
            <admin_panel::Product as HasField<admin_panel::product_fields::Category>>::get_field(
                &admin_product,
            );
        let stock = <inventory::Product as HasField<inventory::product_fields::Stock>>::get_field(
            &inventory_product,
        );

        assert_eq!(*price, 299.99);
        assert_eq!(category, "Software");
        assert_eq!(*stock, 15);
    }

    #[test]
    fn test_field_witness_types_are_distinct() {
        // Demonstrate that field witnesses from different modules are distinct types
        // This test ensures that the type system correctly distinguishes between
        // field witnesses even when they have the same struct and field names

        // These should be different types even though both are "name" fields
        let _user_mgmt_name_witness = user_management::user_fields::Name;
        let _admin_name_witness = admin_panel::user_fields::Name;
        let _user_mgmt_product_name_witness = user_management::product_fields::Name;
        let _admin_product_name_witness = admin_panel::product_fields::Name;
        let _inventory_product_name_witness = inventory::product_fields::Name;

        // If there were type conflicts, this test would fail to compile
        // The fact that we can create these distinct witnesses proves the scoping works
        // If this test compiles, module scoping is working correctly
    }

    #[test]
    fn test_cross_module_field_witness_incompatibility() {
        // This test demonstrates that you cannot use a field witness from one module
        // with a struct from another module (which would be a type error)

        let user_mgmt_user = user_management::User {
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            role: "developer".to_string(),
        };

        // This works - correct pairing
        let _name =
            <user_management::User as HasField<user_management::user_fields::Name>>::get_field(
                &user_mgmt_user,
            );

        // The following would NOT compile if uncommented, proving type safety:
        // let _name = <user_management::User as HasField<admin_panel::user_fields::Name>>::get_field(&user_mgmt_user);
        // Error: the trait `HasField<admin_panel::user_fields::Name>` is not implemented for `user_management::User`

        // Type safety ensures field witnesses can only be used with their corresponding structs
    }
}
