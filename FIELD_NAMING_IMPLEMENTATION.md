# Enhanced Field Witnesses with Optional Parameters

## Current Implementation

The field witnesses macro has been enhanced with infrastructure for optional parameters, specifically field naming strategies. The implementation includes:

### 1. **Enhanced Derive Macro**

- Updated `FieldWitnesses` derive macro supports attribute parsing
- Infrastructure for both built-in and custom field naming strategies
- Proper error handling and helpful error messages
- Comprehensive documentation with examples

### 2. **Built-in Field Naming Strategies**

The macro supports two built-in strategies with **generic string transformations** that work with any field name:

- **PascalCase**: Converts `snake_case` to `PascalCase`
  - `user_name` → `"UserName"`
  - `created_at` → `"CreatedAt"`
  - `some_arbitrary_field_name` → `"SomeArbitraryFieldName"`

- **camelCase**: Converts `snake_case` to `camelCase` 
  - `user_name` → `"userName"`
  - `created_at` → `"createdAt"`
  - `some_arbitrary_field_name` → `"someArbitraryFieldName"`

**Implementation Details:**
- Uses Rust's built-in string methods (`split`, `chars`, `to_uppercase`, `to_lowercase`)
- Works with any field name, not just hardcoded mappings
- Zero runtime overhead - all transformations happen at macro expansion time

### 3. **Field-level Overrides**

- `#[tnuctipun(rename = "custom_name")]` - Override specific field names
- `#[tnuctipun(skip)]` - Skip field witness generation for specific fields

### 4. **Private Field Control**

- `#[tnuctipun(include_private = true)]` - Include private fields in witness generation
- `#[tnuctipun(include_private = false)]` - Skip private fields (default behavior)

## Usage Examples

### Basic Usage (Current - Working)

```rust
#[derive(FieldWitnesses)]
struct User {
    user_name: String,  // Field name: "user_name"
    age: i32,           // Field name: "age"
}
```

### With Attributes (Currently Working)

```rust
// Built-in strategies
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase")]
struct User {
    user_name: String,      // → "userName"
    email_address: String,  // → "emailAddress"
}

// Field-level overrides
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase")]
struct User {
    user_name: String,              // → "userName"
    #[tnuctipun(rename = "email")]
    email_address: String,          // → "email" (override)
    #[tnuctipun(skip)]
    internal_id: String,            // Skipped entirely
}

// Private field control
#[derive(FieldWitnesses)]
#[tnuctipun(include_private = true)]
struct UserWithPrivate {
    pub user_name: String,          // Public field - included
    email_address: String,          // Private field - included
}

// Combined attributes
#[derive(FieldWitnesses)]
#[tnuctipun(field_naming = "camelCase", include_private = true)]
struct UserCombined {
    pub user_name: String,          // → "userName" (public)
    email_address: String,          // → "emailAddress" (private, included)
}
```

## Implementation Status

✅ **Completed:**

- Enhanced derive macro with attribute parsing infrastructure
- Built-in transformation functions (PascalCase, camelCase)
- Field-level attribute support (rename, skip)
- Private field inclusion control (include_private)
- Comprehensive error handling
- Updated documentation and examples
- Complete functionality tests
- The `tnuctipun` attribute is registered and working correctly

✅ **All Features Working:**

The field naming implementation is fully functional:

1. **Basic usage** works without any attributes
2. **Built-in strategies** (`PascalCase`, `camelCase`) work correctly
3. **Field-level overrides** (`rename`, `skip`) work correctly
4. **Private field control** (`include_private`) works correctly
5. **All tests pass** (51+ tests covering various scenarios)

## Benefits of Current Approach

1. **Type-Safe**: All transformations happen at compile time
2. **Performance**: Zero runtime overhead
3. **Simple & Maintainable**: Clean architecture with only essential features
4. **Backward Compatible**: Existing code continues to work unchanged
5. **Error-Friendly**: Clear compilation errors for invalid configurations
6. **Well-Tested**: Comprehensive test coverage for all functionality

## Architecture

The implementation uses a clean, focused approach where:
- **Built-in strategies** are resolved and applied at macro expansion time
- **Field-level overrides** take precedence over container-level settings
- **Error handling** provides helpful messages for invalid configurations
- **Simple design** ensures maintainability and reliability

This provides the core functionality needed while working within Rust's proc macro constraints and ensuring type safety and performance. The architecture is intentionally simple and focused on the most common use cases.
