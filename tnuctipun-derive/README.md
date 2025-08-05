# tnuctipun-derive

Derive macros for the [Tnuctipun](https://crates.io/crates/tnuctipun) MongoDB type-safe query builder library.

This crate provides procedural macros that automatically generate implementations for the Tnuctipun traits, making it easy to use your Rust structs with MongoDB operations in a type-safe manner.

## Usage

This crate is typically used as a dependency of the main `tnuctipun` crate. You don't need to add it directly to your `Cargo.toml` unless you're doing advanced macro work.

```toml
[dependencies]
tnuctipun = "0.1.0"
```

The derive macros are re-exported through the main `tnuctipun` crate for convenience.

## Features

- Automatic trait implementations for MongoDB operations
- Type-safe query building
- Field-level access control and validation
- Integration with the broader Tnuctipun ecosystem

## Documentation

For full documentation and examples, see the main [Tnuctipun documentation](https://docs.rs/tnuctipun).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE](../LICENSE) or http://opensource.org/licenses/MIT)

at your option.
