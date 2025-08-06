# Tnuctipun

[![CI](https://github.com/cchantep/tnuctipun/workflows/CI/badge.svg)](https://github.com/cchantep/tnuctipun/actions)
[![Crates.io](https://img.shields.io/crates/v/tnuctipun.svg)](https://crates.io/crates/tnuctipun)
[![API](https://docs.rs/tnuctipun/badge.svg)](https://docs.rs/tnuctipun)
[![Documentation](https://img.shields.io/badge/docs-User%20Guide-blue)](https://cchantep.github.io/tnuctipun/)
[![Codecov](https://codecov.io/gh/cchantep/tnuctipun/branch/master/graph/badge.svg)](https://codecov.io/gh/cchantep/tnuctipun)

The Tnuctipun of Ringworld — ancient, subversive, ingenious — or a type-safe MongoDB builder library.

## Features

- **Type-safe field access**: Use compile-time validated field names
- **MongoDB query building**: Build complex queries with type safety
- **MongoDB projection building**: Create projections with fluent method chaining
- **MongoDB update building**: Create update documents with type-safe field operations
- **Derive macros**: Automatically generate field witnesses and comparable traits
- **Compile-time validation**: Catch field name typos and type mismatches at compile time

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tnuctipun = "0.1.1"
```

The library only requires the `bson` crate for MongoDB document types and provides type-safe query building capabilities.

## Example

```rust
use tnuctipun::{FieldWitnesses, MongoComparable, filters::empty, projection, updates};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    pub name: String,
    pub age: i32,
    pub email: String,
}

// Type-safe filter building with compile-time field validation
let mut filter_builder = empty::<User>();

filter_builder.eq::<user_fields::Name, _>("John".to_string());
filter_builder.gt::<user_fields::Age, _>(18);

// Convert to MongoDB document  
let filter_doc = filter_builder.and();
// Results in: { "$and": [{ "name": "John" }, { "age": { "$gt": 18 } }] }

// Type-safe projection building with method chaining
let projection_doc = projection::empty::<User>()
    .includes::<user_fields::Name>()
    .includes::<user_fields::Age>()
    .excludes::<user_fields::Email>()  // Hide sensitive data
    .build();
// Results in: { "name": 1, "age": 1, "email": 0 }

// Type-safe update building with compile-time field validation  
let update_doc = updates::empty::<User>()
    .set::<user_fields::Name, _>("Jane".to_string())
    .inc::<user_fields::Age, _>(1)
    .unset::<user_fields::Email>()
    .build();
// Results in: { 
//   "$set": { "name": "Jane" }, 
//   "$inc": { "age": 1 }, 
//   "$unset": { "email": "" } 
// }
```

## Documentation

- **[User Guide](https://cchantep.github.io/tnuctipun/)** - Comprehensive documentation with examples and tutorials
- **[API Documentation](https://docs.rs/tnuctipun/latest/tnuctipun/)** - Complete API reference
- **[Crates.io Documentation](https://docs.rs/tnuctipun)** - Released version docs

## Development

For information about contributing and releasing new versions, see:

- [Release Guide](RELEASE.md) - How to publish new versions
- [Changelog](CHANGELOG.md) - What's new in each version

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
