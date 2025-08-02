# Nessus

[![CI](https://github.com/cchantep/nessus/workflows/CI/badge.svg)](https://github.com/cchantep/nessus/actions)
[![Crates.io](https://img.shields.io/crates/v/nessus.svg)](https://crates.io/crates/nessus)
[![Documentation](https://docs.rs/nessus/badge.svg)](https://docs.rs/nessus)
[![GitHub Pages](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://cchantep.github.io/nessus/nessus/)
[![codecov](https://codecov.io/gh/cchantep/nessus/branch/master/graph/badge.svg)](https://codecov.io/gh/cchantep/nessus)

The Puppeteer from Ringworld — wise, cautious, clever — or a type-safe MongoDB builder library.

## Features

- **Type-safe field access**: Use compile-time validated field names
- **MongoDB query building**: Build complex queries with type safety
- **MongoDB projection building**: Create projections with fluent method chaining
- **Derive macros**: Automatically generate field witnesses and comparable traits
- **Compile-time validation**: Catch field name typos and type mismatches at compile time

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nessus = "0.1.0"
```

The core library only requires the `bson` crate for MongoDB document types. If you need to connect to MongoDB (for example, in applications using the binary), enable the `mongodb-client` feature:

```toml
[dependencies]
nessus = { version = "0.1.0", features = ["mongodb-client"] }
```

## Example

```rust
use nessus::{FieldWitnesses, MongoComparable, filters::empty, projection::ProjectionBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    name: String,
    age: i32,
    email: String,
}

// Type-safe filter building with compile-time field validation
let mut builder = empty::<User>()
    .eq::<user_fields::Name, _>("John".to_string())
    .gt::<user_fields::Age, _>(18);

// Convert to MongoDB document  
let filter_doc = builder.and();
// Results in: { "$and": [{ "name": "John" }, { "age": { "$gt": 18 } }] }

// Type-safe projection building with method chaining
let projection_doc = ProjectionBuilder::<User>::new()
    .includes::<user_fields::Name>()
    .includes::<user_fields::Age>()
    .excludes::<user_fields::Email>()  // Hide sensitive data
    .build();
// Results in: { "name": 1, "age": 1, "email": 0 }
```

## Documentation

- **[API Documentation (GitHub Pages)](https://cchantep.github.io/nessus/nessus/)** - Latest development docs
- **[Crates.io Documentation](https://docs.rs/nessus)** - Released version docs

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
