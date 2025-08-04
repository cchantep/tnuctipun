//! Tests for nested field operations using with_lookup

use super::test_fixtures::*;
use tnuctipun::filters::empty;

#[test]
fn test_simple_nested_lookup() {
    // Test basic nested field access
    let result = empty::<User>()
        .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
            |path| path.field::<address_fields::City>(),
            |nested| nested.eq::<address_fields::City, _>("Portland".to_string()),
        )
        .and();

    let expected = bson::doc! { "home_address.city": "Portland" };

    assert_eq!(result, expected);
}
