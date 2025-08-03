use nessus::field_witnesses::{FieldName, HasField};
use nessus::updates::{CurrentDateType, PopStrategy, UpdateBuilder, UpdateOperation, empty};

// Mock field name marker types for testing
struct TestFieldName;
impl FieldName for TestFieldName {
    fn field_name() -> &'static str {
        "test_field"
    }
}

struct AnotherFieldName;
impl FieldName for AnotherFieldName {
    fn field_name() -> &'static str {
        "another_field"
    }
}

struct NestedFieldName;
impl FieldName for NestedFieldName {
    fn field_name() -> &'static str {
        "nested.field"
    }
}

struct ArrayFieldName;
impl FieldName for ArrayFieldName {
    fn field_name() -> &'static str {
        "array_field"
    }
}

struct NumericFieldName;
impl FieldName for NumericFieldName {
    fn field_name() -> &'static str {
        "numeric_field"
    }
}

// Test struct to verify HasField trait implementation
struct TestStruct {
    test_field: String,
    another_field: i32,
    nested_field: bool,
    array_field: Vec<String>,
    numeric_field: i32,
}

impl HasField<TestFieldName> for TestStruct {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.test_field
    }
}

impl HasField<AnotherFieldName> for TestStruct {
    type Value = i32;

    fn get_field(&self) -> &Self::Value {
        &self.another_field
    }
}

impl HasField<NestedFieldName> for TestStruct {
    type Value = bool;

    fn get_field(&self) -> &Self::Value {
        &self.nested_field
    }
}

impl HasField<ArrayFieldName> for TestStruct {
    type Value = Vec<String>;

    fn get_field(&self) -> &Self::Value {
        &self.array_field
    }
}

impl HasField<NumericFieldName> for TestStruct {
    type Value = i32;

    fn get_field(&self) -> &Self::Value {
        &self.numeric_field
    }
}

#[test]
fn test_update_operation_to_string() {
    assert_eq!(UpdateOperation::Set.to_string(), "$set");
    assert_eq!(UpdateOperation::Unset.to_string(), "$unset");
    assert_eq!(UpdateOperation::Inc.to_string(), "$inc");
    assert_eq!(UpdateOperation::Mul.to_string(), "$mul");
    assert_eq!(UpdateOperation::Rename.to_string(), "$rename");
    assert_eq!(UpdateOperation::CurrentDate.to_string(), "$currentDate");
    assert_eq!(UpdateOperation::AddToSet.to_string(), "$addToSet");
    assert_eq!(UpdateOperation::Pop.to_string(), "$pop");
    assert_eq!(UpdateOperation::Pull.to_string(), "$pull");
    assert_eq!(UpdateOperation::PullAll.to_string(), "$pullAll");
    assert_eq!(UpdateOperation::Push.to_string(), "$push");
}

#[test]
fn test_update_operation_consistency() {
    // Test that multiple calls to to_string() return the same value
    let set_op = UpdateOperation::Set;

    assert_eq!(set_op.to_string(), set_op.to_string());

    let unset_op = UpdateOperation::Unset;

    assert_eq!(unset_op.to_string(), unset_op.to_string());
}

#[test]
fn test_update_operation_all_variants_covered() {
    // This test ensures all enum variants are explicitly tested
    let operations = [
        UpdateOperation::Set,
        UpdateOperation::Unset,
        UpdateOperation::Inc,
        UpdateOperation::Mul,
        UpdateOperation::Rename,
        UpdateOperation::CurrentDate,
        UpdateOperation::AddToSet,
        UpdateOperation::Pop,
        UpdateOperation::Pull,
        UpdateOperation::PullAll,
        UpdateOperation::Push,
    ];

    let expected_strings = [
        "$set",
        "$unset",
        "$inc",
        "$mul",
        "$rename",
        "$currentDate",
        "$addToSet",
        "$pop",
        "$pull",
        "$pullAll",
        "$push",
    ];

    assert_eq!(operations.len(), expected_strings.len());

    for (op, expected) in operations.iter().zip(expected_strings.iter()) {
        assert_eq!(op.to_string(), *expected);
    }
}

#[test]
fn test_update_operation_string_format() {
    // Test that all operations start with '$' as expected by MongoDB
    let operations = vec![
        UpdateOperation::Set,
        UpdateOperation::Unset,
        UpdateOperation::Inc,
        UpdateOperation::Mul,
        UpdateOperation::Rename,
        UpdateOperation::CurrentDate,
        UpdateOperation::AddToSet,
        UpdateOperation::Pop,
        UpdateOperation::Pull,
        UpdateOperation::PullAll,
        UpdateOperation::Push,
    ];

    for op in operations {
        let string_repr = op.to_string();

        assert!(
            string_repr.starts_with('$'),
            "Operation {} should start with '$'",
            string_repr
        );

        assert!(
            !string_repr.is_empty(),
            "Operation string should not be empty"
        );
    }
}

#[test]
fn test_update_operation_hash_equality() {
    use std::collections::HashSet;

    // Test that the same operation variants are equal and hash to the same value
    let mut set = HashSet::new();

    set.insert(UpdateOperation::Set);
    set.insert(UpdateOperation::Set); // Duplicate should not increase size

    assert_eq!(set.len(), 1);

    // Test that different operations are not equal
    set.insert(UpdateOperation::Unset);

    assert_eq!(set.len(), 2);
}

#[test]
fn test_single_set_operation() {
    let result = empty::<TestStruct>()
        .set::<TestFieldName, _>("test_value")
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "test_value"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_set_operations_combined() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("first_value")
        .set::<AnotherFieldName, _>(42)
        .set::<NestedFieldName, _>(true)
        .build();

    // Check the entire document structure
    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "first_value",
            "another_field": 42,
            "nested.field": true
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_multiple_set_operations_same_field() {
    let result = empty::<TestStruct>()
        .set::<TestFieldName, _>("first_value")
        .set::<TestFieldName, _>("second_value")
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "second_value"  // last set operation wins
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_set_operations_with_prefix() {
    let mut builder = UpdateBuilder::<TestStruct>::new();

    // Add prefix to simulate nested document updates
    builder.prefix.push("parent".to_string());
    builder.prefix.push("child".to_string());

    builder.set::<TestFieldName, _>("nested_value");
    builder.set::<AnotherFieldName, _>(100);

    let doc = builder.build();

    // Check the entire document structure with prefixed field paths
    let expected_doc = bson::doc! {
        "$set": {
            "parent.child.test_field": "nested_value",
            "parent.child.another_field": 100
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_set_operations_different_bson_types() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("string_value")
        .set::<AnotherFieldName, _>(42i32)
        .set::<NestedFieldName, _>(true)
        .build();

    // Check the entire document structure with different BSON types
    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "string_value",
            "another_field": 42,
            "nested.field": true
        }
    };

    assert_eq!(doc, expected_doc);
}

#[test]
fn test_empty_builder_produces_empty_document() {
    let doc = empty::<TestStruct>().build();
    
    assert!(doc.is_empty());
}

#[test]
fn test_comprehensive_document_structure() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("comprehensive_test")
        .set::<AnotherFieldName, _>(999)
        .set::<NestedFieldName, _>(false)
        .build();

    // This approach is much cleaner than checking individual fields!
    // It verifies the entire document structure in one assertion
    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "comprehensive_test",
            "another_field": 999,
            "nested.field": false
        }
    };

    // Single assertion covers structure, field names, values, and types
    assert_eq!(doc, expected_doc);
}

#[test]
fn test_method_chaining_works_with_mut_self_pattern() {
    // This test demonstrates that method chaining now works fully,
    // including calling build() at the end of the chain
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("chained_value")
        .set::<AnotherFieldName, _>(100)
        .inc::<AnotherFieldName, _>(50)
        .build();

    let expected_doc = bson::doc! {
        "$set": {
            "test_field": "chained_value",
            "another_field": 100
        },
        "$inc": {
            "another_field": 50
        }
    };

    assert_eq!(doc, expected_doc);
}

// Tests for empty() function
#[test]
fn test_empty_function_creates_new_builder() {
    let mut builder = empty::<TestStruct>();
    let doc = builder.build();

    assert!(doc.is_empty());
}

#[test]
fn test_empty_function_method_chaining() {
    let doc = empty::<TestStruct>()
        .set::<TestFieldName, _>("test")
        .build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "test"
        }
    };

    assert_eq!(doc, expected);
}

// Tests for unset operation
#[test]
fn test_single_unset_operation() {
    let result = empty::<TestStruct>()
        .unset::<TestFieldName>()
        .build();

    let expected = bson::doc! {
        "$unset": {
            "test_field": bson::Bson::Null
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_unset_operations() {
    let result = empty::<TestStruct>()
        .unset::<TestFieldName>()
        .unset::<AnotherFieldName>()
        .build();

    let expected = bson::doc! {
        "$unset": {
            "test_field": bson::Bson::Null,
            "another_field": bson::Bson::Null
        }
    };

    assert_eq!(result, expected);
}

// Tests for inc operation
#[test]
fn test_single_inc_operation() {
    let result = empty::<TestStruct>()
        .inc::<NumericFieldName, _>(5)
        .build();

    let expected = bson::doc! {
        "$inc": {
            "numeric_field": 5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_inc_operation_negative_value() {
    let result = empty::<TestStruct>()
        .inc::<NumericFieldName, _>(-3)
        .build();

    let expected = bson::doc! {
        "$inc": {
            "numeric_field": -3
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_inc_operations() {
    let result = empty::<TestStruct>()
        .inc::<NumericFieldName, _>(1)
        .inc::<AnotherFieldName, _>(10)
        .build();

    let expected = bson::doc! {
        "$inc": {
            "numeric_field": 1,
            "another_field": 10
        }
    };

    assert_eq!(result, expected);
}

// Tests for mul operation
#[test]
fn test_single_mul_operation() {
    let result = empty::<TestStruct>()
        .mul::<NumericFieldName, _>(2)
        .build();

    let expected = bson::doc! {
        "$mul": {
            "numeric_field": 2
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_mul_operation_decimal() {
    let result = empty::<TestStruct>()
        .mul::<NumericFieldName, _>(1.5f64)
        .build();

    let expected = bson::doc! {
        "$mul": {
            "numeric_field": 1.5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_mul_operations() {
    let result = empty::<TestStruct>()
        .mul::<NumericFieldName, _>(3)
        .mul::<AnotherFieldName, _>(0.5f64)
        .build();

    let expected = bson::doc! {
        "$mul": {
            "numeric_field": 3,
            "another_field": 0.5
        }
    };

    assert_eq!(result, expected);
}

// Tests for rename operation
#[test]
fn test_single_rename_operation() {
    let result = empty::<TestStruct>()
        .rename::<TestFieldName>("new_name")
        .build();

    let expected = bson::doc! {
        "$rename": {
            "test_field": "new_name"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_rename_operations() {
    let result = empty::<TestStruct>()
        .rename::<TestFieldName>("new_test_field")
        .rename::<AnotherFieldName>("new_another_field")
        .build();

    let expected = bson::doc! {
        "$rename": {
            "test_field": "new_test_field",
            "another_field": "new_another_field"
        }
    };

    assert_eq!(result, expected);
}

// Tests for current_date operation
#[test]
fn test_current_date_operation_date_type() {
    let result = empty::<TestStruct>()
        .current_date::<TestFieldName>(CurrentDateType::Date)
        .build();

    let expected = bson::doc! {
        "$currentDate": {
            "test_field": "date"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_current_date_operation_timestamp_type() {
    let result = empty::<TestStruct>()
        .current_date::<TestFieldName>(CurrentDateType::Timestamp)
        .build();

    let expected = bson::doc! {
        "$currentDate": {
            "test_field": "timestamp"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_current_date_operations() {
    let result = empty::<TestStruct>()
        .current_date::<TestFieldName>(CurrentDateType::Date)
        .current_date::<AnotherFieldName>(CurrentDateType::Timestamp)
        .build();

    let expected = bson::doc! {
        "$currentDate": {
            "test_field": "date",
            "another_field": "timestamp"
        }
    };

    assert_eq!(result, expected);
}

// Tests for add_to_set operation
#[test]
fn test_single_add_to_set_operation() {
    let result = empty::<TestStruct>()
        .add_to_set::<ArrayFieldName, _>("new_item".to_string())
        .build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": "new_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_multiple_add_to_set_operations() {
    let mut builder = empty::<TestStruct>();

    builder.add_to_set::<ArrayFieldName, _>("item1".to_string());
    builder.add_to_set::<ArrayFieldName, _>("item2".to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": "item2"  // last one wins due to field overwriting
        }
    };

    assert_eq!(result, expected);
}

// Tests for pop operation
#[test]
fn test_pop_operation_first() {
    let mut builder = empty::<TestStruct>();

    builder.pop::<ArrayFieldName>(PopStrategy::First);

    let result = builder.build();

    let expected = bson::doc! {
        "$pop": {
            "array_field": -1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_pop_operation_last() {
    let mut builder = empty::<TestStruct>();

    builder.pop::<ArrayFieldName>(PopStrategy::Last);

    let result = builder.build();

    let expected = bson::doc! {
        "$pop": {
            "array_field": 1
        }
    };

    assert_eq!(result, expected);
}

// Tests for pull_expr operation
#[test]
fn test_pull_expr_operation() {
    let mut builder = empty::<TestStruct>();
    let condition = bson::doc! { "score": { "$gt": 80 } };

    builder.pull_expr::<ArrayFieldName>(condition.clone().into());

    let result = builder.build();

    let expected = bson::doc! {
        "$pull": {
            "array_field": condition
        }
    };

    assert_eq!(result, expected);
}

// Tests for pull operation
#[test]
fn test_pull_operation() {
    let mut builder = empty::<TestStruct>();
    
    builder.pull::<ArrayFieldName, _>("unwanted_item".to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$pull": {
            "array_field": "unwanted_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_pull_operation_numeric() {
    let mut builder = empty::<TestStruct>();
    
    builder.pull::<ArrayFieldName, _>(42.to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$pull": {
            "array_field": "42"
        }
    };

    assert_eq!(result, expected);
}

// Tests for pull_all operation
#[test]
fn test_pull_all_operation() {
    let mut builder = empty::<TestStruct>();
    let items_to_remove = vec!["item1".to_string(), "item2".to_string()];
    
    builder.pull_all::<ArrayFieldName, _>(items_to_remove);

    let result = builder.build();

    let expected = bson::doc! {
        "$pullAll": {
            "array_field": ["item1", "item2"]
        }
    };

    assert_eq!(result, expected);
}

// Tests for push operation
#[test]
fn test_push_operation() {
    let mut builder = empty::<TestStruct>();

    builder.push::<ArrayFieldName, _>("new_item".to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$push": {
            "array_field": "new_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_push_operation_multiple_items() {
    let result = empty::<TestStruct>()
        .push::<ArrayFieldName, _>("item1".to_string())
        .push::<ArrayFieldName, _>("item2".to_string())
        .build();

    // The second push overwrites the first one due to field overwriting
    let expected = bson::doc! {
        "$push": {
            "array_field": "item2"
        }
    };

    assert_eq!(result, expected);
}

// Tests for CurrentDateType enum
#[test]
fn test_current_date_type_to_string() {
    assert_eq!(CurrentDateType::Date.to_string(), "date");
    assert_eq!(CurrentDateType::Timestamp.to_string(), "timestamp");
}

#[test]
fn test_current_date_type_consistency() {
    let date_type = CurrentDateType::Date;
    
    assert_eq!(date_type.to_string(), date_type.to_string());

    let timestamp_type = CurrentDateType::Timestamp;
    
    assert_eq!(timestamp_type.to_string(), timestamp_type.to_string());
}

// Tests for PopStrategy enum
#[test]
fn test_pop_strategy_from_conversion() {
    let first_strategy = PopStrategy::First;
    let first_bson: bson::Bson = first_strategy.into();
    
    assert_eq!(first_bson, bson::Bson::Int32(-1));

    let last_strategy = PopStrategy::Last;
    let last_bson: bson::Bson = last_strategy.into();
    
    assert_eq!(last_bson, bson::Bson::Int32(1));
}

#[test]
fn test_pop_strategy_consistency() {
    let strategy = PopStrategy::First;
    let bson1: bson::Bson = strategy.into();
    let bson2: bson::Bson = PopStrategy::First.into();
    
    assert_eq!(bson1, bson2);
}

// Integration tests for mixed operations
#[test]
fn test_mixed_operations_comprehensive() {
    let mut builder = empty::<TestStruct>();
    
    builder.set::<TestFieldName, _>("updated_value");
    builder.inc::<NumericFieldName, _>(5);
    builder.unset::<NestedFieldName>();
    builder.push::<ArrayFieldName, _>("new_item".to_string());

    let result = builder.build();

    let expected = bson::doc! {
        "$set": {
            "test_field": "updated_value"
        },
        "$inc": {
            "numeric_field": 5
        },
        "$unset": {
            "nested.field": bson::Bson::Null
        },
        "$push": {
            "array_field": "new_item"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_comprehensive_array_operations() {
    let mut builder = empty::<TestStruct>();
    
    builder.add_to_set::<ArrayFieldName, _>("unique_item".to_string());
    builder.pop::<ArrayFieldName>(PopStrategy::Last);

    let result = builder.build();

    let expected = bson::doc! {
        "$addToSet": {
            "array_field": "unique_item"
        },
        "$pop": {
            "array_field": 1
        }
    };

    assert_eq!(result, expected);
}

// Tests for with_lookup and with_field functions

// Additional structs for testing nested functionality
#[derive(Debug, Clone)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
    country: String,
}

#[derive(Debug, Clone)]
struct ContactInfo {
    email: String,
    phone: String,
}

#[derive(Debug, Clone)]
struct User {
    id: String,
    name: String,
    age: i32,
    home_address: Address,
    work_address: Address,
    contact: ContactInfo,
}

#[derive(Debug, Clone)]
struct Company {
    name: String,
    address: Address,
}

#[derive(Debug, Clone)]
struct Employee {
    id: String,
    name: String,
    company: Company,
}

// Field name marker types for nested structures
struct UserId;
impl FieldName for UserId {
    fn field_name() -> &'static str {
        "id"
    }
}

struct UserName;
impl FieldName for UserName {
    fn field_name() -> &'static str {
        "name"
    }
}

struct UserAge;
impl FieldName for UserAge {
    fn field_name() -> &'static str {
        "age"
    }
}

struct UserHomeAddress;
impl FieldName for UserHomeAddress {
    fn field_name() -> &'static str {
        "home_address"
    }
}

struct UserWorkAddress;
impl FieldName for UserWorkAddress {
    fn field_name() -> &'static str {
        "work_address"
    }
}

struct UserContact;
impl FieldName for UserContact {
    fn field_name() -> &'static str {
        "contact"
    }
}

struct AddressStreet;
impl FieldName for AddressStreet {
    fn field_name() -> &'static str {
        "street"
    }
}

struct AddressCity;
impl FieldName for AddressCity {
    fn field_name() -> &'static str {
        "city"
    }
}

struct AddressZipCode;
impl FieldName for AddressZipCode {
    fn field_name() -> &'static str {
        "zip_code"
    }
}

struct AddressCountry;
impl FieldName for AddressCountry {
    fn field_name() -> &'static str {
        "country"
    }
}

struct ContactEmail;
impl FieldName for ContactEmail {
    fn field_name() -> &'static str {
        "email"
    }
}

struct ContactPhone;
impl FieldName for ContactPhone {
    fn field_name() -> &'static str {
        "phone"
    }
}

struct CompanyName;
impl FieldName for CompanyName {
    fn field_name() -> &'static str {
        "name"
    }
}

struct CompanyAddress;
impl FieldName for CompanyAddress {
    fn field_name() -> &'static str {
        "address"
    }
}

struct EmployeeId;
impl FieldName for EmployeeId {
    fn field_name() -> &'static str {
        "id"
    }
}

struct EmployeeName;
impl FieldName for EmployeeName {
    fn field_name() -> &'static str {
        "name"
    }
}

struct EmployeeCompany;
impl FieldName for EmployeeCompany {
    fn field_name() -> &'static str {
        "company"
    }
}

// HasField implementations for User
impl HasField<UserId> for User {
    type Value = String;
    fn get_field(&self) -> &Self::Value {
        &self.id
    }
}

impl HasField<UserName> for User {
    type Value = String;
    fn get_field(&self) -> &Self::Value {
        &self.name
    }
}

impl HasField<UserAge> for User {
    type Value = i32;
    fn get_field(&self) -> &Self::Value {
        &self.age
    }
}

impl HasField<UserHomeAddress> for User {
    type Value = Address;
    fn get_field(&self) -> &Self::Value {
        &self.home_address
    }
}

impl HasField<UserWorkAddress> for User {
    type Value = Address;
    fn get_field(&self) -> &Self::Value {
        &self.work_address
    }
}

impl HasField<UserContact> for User {
    type Value = ContactInfo;

    fn get_field(&self) -> &Self::Value {
        &self.contact
    }
}

// HasField implementations for Address
impl HasField<AddressStreet> for Address {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.street
    }
}

impl HasField<AddressCity> for Address {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.city
    }
}

impl HasField<AddressZipCode> for Address {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.zip_code
    }
}

impl HasField<AddressCountry> for Address {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.country
    }
}

// HasField implementations for ContactInfo
impl HasField<ContactEmail> for ContactInfo {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.email
    }
}

impl HasField<ContactPhone> for ContactInfo {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.phone
    }
}

// HasField implementations for Company
impl HasField<CompanyName> for Company {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.name
    }
}

impl HasField<CompanyAddress> for Company {
    type Value = Address;

    fn get_field(&self) -> &Self::Value {
        &self.address
    }
}

// HasField implementations for Employee
impl HasField<EmployeeId> for Employee {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.id
    }
}

impl HasField<EmployeeName> for Employee {
    type Value = String;

    fn get_field(&self) -> &Self::Value {
        &self.name
    }
}

impl HasField<EmployeeCompany> for Employee {
    type Value = Company;
    
    fn get_field(&self) -> &Self::Value {
        &self.company
    }
}

// Tests for with_lookup function
#[test]
fn test_with_lookup_single_nested_field() {
    // Test updating a single nested field
    let mut builder = empty::<User>();

    builder.with_lookup::<UserHomeAddress, _, AddressCity, Address, _>(
        |path| path.field::<AddressCity>(),
        |mut nested| {
            nested.set::<AddressCity, _>("New York".to_string());
            nested
        },
    );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "home_address.city": "New York"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_multiple_nested_fields() {
    // Test updating multiple nested fields within the same nested object
    let mut builder = empty::<User>();

    builder
        .with_lookup::<UserHomeAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |mut nested| {
                nested.set::<AddressCity, _>("San Francisco".to_string());
                nested
            },
        )
        .with_lookup::<UserHomeAddress, _, AddressZipCode, Address, _>(
            |path| path.field::<AddressZipCode>(),
            |mut nested| {
                nested.set::<AddressZipCode, _>("94102".to_string());
                nested
            },
        );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "home_address.city": "San Francisco",
            "home_address.zip_code": "94102"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_different_operations() {
    // Test using different MongoDB operators within nested fields
    let mut builder = empty::<User>();

    builder
        .with_lookup::<UserHomeAddress, _, AddressCountry, Address, _>(
            |path| path.field::<AddressCountry>(),
            |mut nested| {
                nested.set::<AddressCountry, _>("USA".to_string());
                nested
            },
        )
        .with_lookup::<UserContact, _, ContactEmail, ContactInfo, _>(
            |path| path.field::<ContactEmail>(),
            |mut nested| {
                nested.unset::<ContactEmail>();
                nested
            },
        );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "home_address.country": "USA"
        },
        "$unset": {
            "contact.email": bson::Bson::Null
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_multiple_nested_objects() {
    // Test updating different nested objects within the same parent
    let mut builder = empty::<User>();

    builder
        .with_lookup::<UserHomeAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |mut nested| {
                nested.set::<AddressCity, _>("Boston".to_string());
                nested
            },
        )
        .with_lookup::<UserWorkAddress, _, AddressCity, Address, _>(
            |path| path.field::<AddressCity>(),
            |mut nested| {
                nested.set::<AddressCity, _>("Cambridge".to_string());
                nested
            },
        );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "home_address.city": "Boston",
            "work_address.city": "Cambridge"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_mixed_with_regular_updates() {
    // Test combining nested updates with regular field updates
    let mut builder = empty::<User>();

    builder
        .set::<UserName, _>("John Doe".to_string())
        .with_lookup::<UserContact, _, ContactEmail, ContactInfo, _>(
            |path| path.field::<ContactEmail>(),
            |mut nested| {
                nested.set::<ContactEmail, _>("john@example.com".to_string());
                nested
            },
        )
        .inc::<UserAge, _>(1);

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "name": "John Doe",
            "contact.email": "john@example.com"
        },
        "$inc": {
            "age": 1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_deep_nesting() {
    // Test deeply nested structures (Employee -> Company -> Address)
    let mut builder = empty::<Employee>();

    builder
        .with_lookup::<EmployeeCompany, _, CompanyName, Company, _>(
            |path| path.field::<CompanyName>(),
            |mut nested| {
                nested.set::<CompanyName, _>("Tech Corp".to_string());
                nested
            },
        )
        .with_lookup::<EmployeeCompany, _, CompanyAddress, Company, _>(
            |path| path.field::<CompanyAddress>(),
            |mut nested| {
                nested.with_lookup::<CompanyAddress, _, AddressCity, Address, _>(
                    |path| path.field::<AddressCity>(),
                    |mut deeply_nested| {
                        deeply_nested.set::<AddressCity, _>("Seattle".to_string());
                        deeply_nested
                    },
                );
                nested
            },
        );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "company.name": "Tech Corp",
            "company.address.city": "Seattle"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_multiple_operations_same_nested_context() {
    // Test multiple operations within the same nested context
    let mut builder = empty::<User>();

    builder.with_lookup::<UserHomeAddress, _, AddressStreet, Address, _>(
        |path| path.field::<AddressStreet>(),
        |mut nested| {
            nested
                .set::<AddressStreet, _>("123 Main St".to_string())
                .set::<AddressCity, _>("Portland".to_string())
                .set::<AddressZipCode, _>("97201".to_string())
                .set::<AddressCountry, _>("USA".to_string());
            nested
        },
    );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "home_address.street": "123 Main St",
            "home_address.city": "Portland",
            "home_address.zip_code": "97201",
            "home_address.country": "USA"
        }
    };

    assert_eq!(result, expected);
}

// Tests for with_field function (convenience method using identity)
#[test]
fn test_with_field_simple_update() {
    // Test with_field for simple field update
    let mut builder = empty::<User>();

    builder.with_field::<UserName, _>(|mut nested| {
        nested.set::<UserName, _>("Alice Smith".to_string());
        nested
    });

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "name": "Alice Smith"
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_field_multiple_operations() {
    // Test with_field with multiple operations on the same field context
    let mut builder = empty::<User>();

    builder.with_field::<UserAge, _>(|mut nested| {
        nested
            .set::<UserName, _>("Bob Johnson".to_string())
            .inc::<UserAge, _>(5);
        nested
    });

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "name": "Bob Johnson"
        },
        "$inc": {
            "age": 5
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_field_vs_direct_comparison() {
    // Test that with_field produces the same result as direct field operations
    let mut builder1 = empty::<User>();
    let mut builder2 = empty::<User>();

    // Using with_field
    builder1.with_field::<UserName, _>(|mut nested| {
        nested.set::<UserName, _>("Test User".to_string());
        nested
    });

    // Using direct field operation
    builder2.set::<UserName, _>("Test User".to_string());

    assert_eq!(builder1.build(), builder2.build());
}

#[test]
fn test_with_field_combined_operations() {
    // Test with_field combined with other operations
    let mut builder = empty::<User>();

    builder
        .with_field::<UserId, _>(|mut nested| {
            nested.set::<UserId, _>("user-123".to_string());
            nested
        })
        .inc::<UserAge, _>(10)
        .with_field::<UserName, _>(|mut nested| {
            nested.set::<UserName, _>("Combined User".to_string());
            nested
        });

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "id": "user-123",
            "name": "Combined User"
        },
        "$inc": {
            "age": 10
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_mixed_with_field_and_with_lookup() {
    // Test combining with_field and with_lookup in the same query
    let mut builder = empty::<User>();

    builder
        .with_field::<UserId, _>(|mut nested| {
            nested.set::<UserId, _>("mixed-user-456".to_string());
            nested
        })
        .with_lookup::<UserContact, _, ContactPhone, ContactInfo, _>(
            |path| path.field::<ContactPhone>(),
            |mut nested| {
                nested.set::<ContactPhone, _>("+1-555-0123".to_string());
                nested
            },
        )
        .with_field::<UserAge, _>(|mut nested| {
            nested.inc::<UserAge, _>(1);
            nested
        });

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "id": "mixed-user-456",
            "contact.phone": "+1-555-0123"
        },
        "$inc": {
            "age": 1
        }
    };

    assert_eq!(result, expected);
}

#[test]
fn test_with_lookup_complex_operations() {
    // Test nested fields with complex MongoDB operators
    let mut builder = empty::<User>();

    builder
        .with_lookup::<UserHomeAddress, _, AddressStreet, Address, _>(
            |path| path.field::<AddressStreet>(),
            |mut nested| {
                nested
                    .set::<AddressStreet, _>("456 Oak Ave".to_string())
                    .set::<AddressCity, _>("Denver".to_string());
                nested
            },
        )
        .with_lookup::<UserContact, _, ContactEmail, ContactInfo, _>(
            |path| path.field::<ContactEmail>(),
            |mut nested| {
                nested.set::<ContactEmail, _>("user@denver.com".to_string());
                nested
            },
        );

    let result = builder.build();
    let expected = bson::doc! {
        "$set": {
            "home_address.street": "456 Oak Ave",
            "home_address.city": "Denver",
            "contact.email": "user@denver.com"
        }
    };

    assert_eq!(result, expected);
}
