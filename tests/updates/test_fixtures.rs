//! Shared test fixtures, mock types, and field implementations

use nessus::field_witnesses::{FieldName, HasField};

// Mock field name marker types for testing
pub struct TestFieldName;

impl FieldName for TestFieldName {
    fn field_name() -> &'static str {
        "test_field"
    }
}

pub struct AnotherFieldName;

impl FieldName for AnotherFieldName {
    fn field_name() -> &'static str {
        "another_field"
    }
}

pub struct NestedFieldName;

impl FieldName for NestedFieldName {
    fn field_name() -> &'static str {
        "nested.field"
    }
}

pub struct ArrayFieldName;

impl FieldName for ArrayFieldName {
    fn field_name() -> &'static str {
        "array_field"
    }
}

pub struct NumericFieldName;

impl FieldName for NumericFieldName {
    fn field_name() -> &'static str {
        "numeric_field"
    }
}

// Test struct to verify HasField trait implementation
pub struct TestStruct {
    pub test_field: String,
    pub another_field: i32,
    pub nested_field: bool,
    pub array_field: Vec<String>,
    pub numeric_field: i32,
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

// Additional structs for testing nested functionality
#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
    pub country: String,
}

#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: i32,
    pub home_address: Address,
    pub work_address: Address,
    pub contact: ContactInfo,
}

#[derive(Debug, Clone)]
pub struct Company {
    pub name: String,
    pub address: Address,
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub company: Company,
}

// Field name marker types for nested structures
pub struct UserId;

impl FieldName for UserId {
    fn field_name() -> &'static str {
        "id"
    }
}

pub struct UserName;

impl FieldName for UserName {
    fn field_name() -> &'static str {
        "name"
    }
}

pub struct UserAge;

impl FieldName for UserAge {
    fn field_name() -> &'static str {
        "age"
    }
}

pub struct UserHomeAddress;

impl FieldName for UserHomeAddress {
    fn field_name() -> &'static str {
        "home_address"
    }
}

pub struct UserWorkAddress;

impl FieldName for UserWorkAddress {
    fn field_name() -> &'static str {
        "work_address"
    }
}

pub struct UserContact;

impl FieldName for UserContact {
    fn field_name() -> &'static str {
        "contact"
    }
}

pub struct AddressStreet;

impl FieldName for AddressStreet {
    fn field_name() -> &'static str {
        "street"
    }
}

pub struct AddressCity;

impl FieldName for AddressCity {
    fn field_name() -> &'static str {
        "city"
    }
}

pub struct AddressZipCode;

impl FieldName for AddressZipCode {
    fn field_name() -> &'static str {
        "zip_code"
    }
}

pub struct AddressCountry;

impl FieldName for AddressCountry {
    fn field_name() -> &'static str {
        "country"
    }
}

pub struct ContactEmail;

impl FieldName for ContactEmail {
    fn field_name() -> &'static str {
        "email"
    }
}

pub struct ContactPhone;

impl FieldName for ContactPhone {
    fn field_name() -> &'static str {
        "phone"
    }
}

pub struct CompanyName;

impl FieldName for CompanyName {
    fn field_name() -> &'static str {
        "name"
    }
}

pub struct CompanyAddress;

impl FieldName for CompanyAddress {
    fn field_name() -> &'static str {
        "address"
    }
}

pub struct EmployeeId;

impl FieldName for EmployeeId {
    fn field_name() -> &'static str {
        "id"
    }
}

pub struct EmployeeName;

impl FieldName for EmployeeName {
    fn field_name() -> &'static str {
        "name"
    }
}

pub struct EmployeeCompany;

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
