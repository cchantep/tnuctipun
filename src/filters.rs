use crate::field_filters::FieldFilterBuilder;
use crate::field_witnesses::{FieldName, HasField};
use crate::mongo_comparable::MongoComparable;
use crate::path::Path;
use bson;

/// A builder for constructing MongoDB filters with type safety.
///
/// This builder provides a fluent interface for creating complex MongoDB queries
/// while ensuring at compile time that fields exist and have compatible types.
///
/// The builder can be used to chain multiple filter conditions together,
/// and provides methods to combine them using MongoDB's `$and` semantics.
///
/// # Type Parameters
///
/// * `T` - The struct type that this filter builder operates on (e.g., `User`, `Product`)
pub struct FilterBuilder<T> {
    prefix: Vec<String>,
    clauses: Vec<bson::Document>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> FilterBuilder<T> {
    /// Creates a new empty FilterBuilder instance.
    ///
    /// Notes: Prefer using the `the `empty` function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::FilterBuilder;
    /// use tnuctipun::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User { pub Name: String }
    ///
    /// let builder = FilterBuilder::<User>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            prefix: Vec::new(),
            clauses: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a fully qualified field path for the given field name marker type.
    fn field_path<F: FieldName>(&self) -> String {
        if self.prefix.is_empty() {
            F::field_name().to_string()
        } else {
            format!("{}.{}", self.prefix.join("."), F::field_name())
        }
    }

    /// Creates a type-safe equality filter (`$eq`) that checks at compile time if the field exists
    /// and has the correct type or a compatible type.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::Name`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use tnuctipun::filters::empty;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { pub Name: String, pub Age: i32 }
    ///
    /// // Using builder pattern with efficient chaining:
    /// empty::<User>().eq::<user_fields::Name, _>("John".to_string());
    /// ```
    pub fn eq<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses.push(bson::doc! { path: value.into() });

        self
    }

    /// Returns the current filter clauses as a vector of BSON documents.
    pub fn clauses(&self) -> &Vec<bson::Document> {
        &self.clauses
    }

    /// Creates a type-safe version of MongoDB's greater than (`$gt`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Price`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub Price: f64 }
    ///
    /// // Filter for products with price > 500
    /// empty::<Product>().gt::<product_fields::Price, _>(500.0);
    /// // Resulting BSON: { "Price": { "$gt": 500.0 } }
    /// ```
    pub fn gt<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$gt": value.into() } });

        self
    }

    /// Creates a type-safe version of MongoDB's less (`$lt`) than filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Stock`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub Stock: i32 }
    ///
    /// // Filter for products with stock < 10
    /// empty::<Product>().lt::<product_fields::Stock, _>(10);
    /// // Resulting BSON: { "Stock": { "$lt": 10 } }
    /// ```
    pub fn lt<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$lt": value.into() } });

        self
    }

    /// Type-safe version of "in" operator filter
    ///
    /// Creates a MongoDB filter that matches any of the values in the provided array.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::Age`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { pub Age: i32 }
    ///
    /// // Filter for users with age in [20, 30, 40]
    /// empty::<User>().r#in::<user_fields::Age, _>(vec![20, 30, 40]);
    /// // Resulting BSON: { "Age": { "$in": [20, 30, 40] } }
    /// ```
    pub fn r#in<F, V>(&mut self, values: Vec<V>) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.clauses
            .push(bson::doc! { path: { "$in": bson_values } });

        self
    }

    /// Creates a type-safe version of MongoDB's "not equal" (`$ne`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `order_fields::Status`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Order { pub Status: String }
    ///
    /// // Filter for orders with status not equal to "Delivered"
    /// empty::<Order>().ne::<order_fields::Status, _>("Delivered".to_string());
    /// // Resulting BSON: { "Status": { "$ne": "Delivered" } }
    /// ```
    pub fn ne<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$ne": value.into() } });

        self
    }

    /// Creates a type-safe version of MongoDB's "greater than or equal" (`$gte`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Rating`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub Rating: f64 }
    ///
    /// // Filter for products with rating >= 4.5
    /// empty::<Product>().gte::<product_fields::Rating, _>(4.5);
    /// // Resulting BSON: { "Rating": { "$gte": 4.5 } }
    /// ```
    pub fn gte<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$gte": value.into() } });

        self
    }

    /// Creates a type-safe version of MongoDB's "less than or equal" (`$lte`) filter.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Price`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub Price: f64 }
    ///
    /// // Filter for products with price <= 100.0
    /// empty::<Product>().lte::<product_fields::Price, _>(100.0);
    /// // Resulting BSON: { "Price": { "$lte": 100.0 } }
    /// ```
    pub fn lte<F, V>(&mut self, value: V) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$lte": value.into() } });

        self
    }

    /// Creates a type-safe version of "exists" filter, that checks if a field exists in the document.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::OptionalField`)
    ///   (must implement `HasField<F>` to ensure compile-time field existence)
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User {
    ///     pub Name: String,
    ///     pub PhoneNumber: Option<String>
    /// }
    ///
    /// // Filter for users that have a phone number
    /// empty::<User>().exists::<user_fields::PhoneNumber>(true);
    /// // Resulting BSON: { "PhoneNumber": { "$exists": true } }
    ///
    /// // Filter for users without a phone number
    /// empty::<User>().exists::<user_fields::PhoneNumber>(false);
    /// // Resulting BSON: { "PhoneNumber": { "$exists": false } }
    /// ```
    pub fn exists<F>(&mut self, exists: bool) -> &mut Self
    where
        F: FieldName,
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { path: { "$exists": exists } });

        self
    }

    /// Creates a type-safe version of MongoDB's "not in" (`$nin`) operator filter,
    /// that matches values NOT in the provided array.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Category`)
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub Category: String }
    ///
    /// // Filter for products NOT in the categories "Clothing", "Shoes", or "Accessories"
    /// empty::<Product>().nin::<product_fields::Category, _>(vec![
    ///     "Clothing".to_string(),
    ///     "Shoes".to_string(),
    ///     "Accessories".to_string()
    /// ]);
    /// // Resulting BSON: { "Category": { "$nin": ["Clothing", "Shoes", "Accessories"] } }
    /// ```
    pub fn nin<F, V>(&mut self, values: Vec<V>) -> &mut Self
    where
        F: FieldName,
        T: HasField<F> + MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let path = self.field_path::<F>();
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.clauses
            .push(bson::doc! { path: { "$nin": bson_values } });

        self
    }

    /// Creates filters for nested fields within documents using a path-based lookup approach.
    /// This method provides explicit control over field path construction,
    /// allowing you to specify exactly which nested field to target through a lookup function.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type for the base field (e.g., `user_fields::HomeAddress`)
    /// * `L` - The lookup function type that resolves the field path
    /// * `G` - The field name marker type for the target nested field (e.g., `address_fields::City`)
    /// * `U` - The type of the nested structure containing the target field
    /// * `N` - The closure that builds filters on the nested FilterBuilder
    ///
    /// # Arguments
    /// * `lookup` - A function that takes a `Path<F, T, T>` and returns a `Path<G, U, T>` to specify the target field
    /// * `f` - A closure that builds filter conditions on the resolved nested field
    ///
    /// # Note
    /// For simpler cases where you want to filter on the field itself (identity lookup),
    /// consider using the `with_field` method instead, which is more concise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Address {
    ///     pub Street: String,
    ///     pub City: String,
    ///     pub ZipCode: String,
    /// }
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub Name: String,
    ///     pub HomeAddress: Address,
    /// }
    ///
    /// // Using field navigation for accessing nested fields (G≠F, U≠T)
    /// let mut builder = empty::<User>().with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
    ///     |path| path.field::<address_fields::City>(),
    ///     |nested| {
    ///         nested.eq::<address_fields::City, _>("New York".to_string())
    ///     }
    /// );
    /// // Resulting BSON: { "HomeAddress.City": "New York" }
    ///
    /// // For identity cases (filtering on the field itself), prefer with_field():
    /// // builder.with_field::<user_fields::HomeAddress, _>(|nested| {
    /// //     nested.exists::<user_fields::HomeAddress>(true)
    /// // });
    /// // Resulting BSON: { "HomeAddress": { "$exists": true } }
    /// ```
    pub fn with_lookup<F: FieldName, L, G: FieldName, U: HasField<G>, N>(
        &mut self,
        lookup: L,
        f: N,
    ) -> &mut Self
    where
        T: HasField<F>,
        L: FnOnce(&Path<F, T, T>) -> Path<G, U, T>,
        N: FnOnce(&mut FilterBuilder<U>) -> &mut FilterBuilder<U>,
    {
        // Create a base field path for the lookup
        let base_field: Path<F, T, T> = Path {
            prefix: self.prefix.clone(),
            _marker: std::marker::PhantomData,
        };

        // Resolve the field path using the provided lookup function
        let resolved_field = lookup(&base_field);

        // Create a new FilterBuilder for the nested field
        let mut nested_builder = FilterBuilder::<U> {
            prefix: resolved_field.prefix.clone(),
            clauses: vec![],
            _marker: std::marker::PhantomData,
        };

        f(&mut nested_builder);

        // Add the nested clauses individually to the main builder
        self.clauses.extend(nested_builder.clauses);

        self
    }

    /// Convenience method for filtering on a field directly (using identity lookup).
    ///
    /// Notes: This is a specialized version of `with_lookup` that uses `std::convert::identity`
    /// as the lookup function, making it easier to apply filters directly to a field
    /// without needing to specify the identity function explicitly.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `user_fields::HomeAddress`)
    /// * `N` - The closure that builds filters on the field
    ///
    /// # Arguments
    /// * `f` - A closure that builds filter conditions on the field
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub Name: String,
    ///     pub HomeAddress: String,
    /// }
    ///
    /// let mut builder = empty::<User>().with_field::<user_fields::HomeAddress, _>(|nested| {
    ///     nested.exists::<user_fields::HomeAddress>(true)
    /// });
    /// // Resulting BSON: { "HomeAddress": { "$exists": true } }
    /// ```
    pub fn with_field<F: FieldName, N>(&mut self, f: N) -> &mut Self
    where
        T: HasField<F>,
        N: FnOnce(&mut FilterBuilder<T>) -> &mut FilterBuilder<T>,
    {
        self.with_lookup::<F, _, F, T, _>(
            |path| Path {
                prefix: path.prefix.clone(),
                _marker: std::marker::PhantomData,
            },
            f,
        )
    }

    /// Create a type-safe version of MongoDB's "$or" operator,
    /// where each clause is generated by applying a closure to each item in the input iterable.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Category`)
    /// * `V` - An iterable type containing values to process
    /// * `N` - A closure that takes a FilterBuilder and an item from V, and returns the FilterBuilder
    ///
    /// # Arguments
    ///
    /// * `input` - An iterable collection of values to process
    /// * `f` - A closure that builds filter conditions for each value in the input
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product {
    ///     pub name: String,
    ///     pub category: String,
    ///     pub price: f64,
    /// }
    ///
    /// // Filter for products that match any of the given names
    /// let names = vec!["Laptop", "Smartphone", "Tablet"];
    ///
    /// empty::<Product>().or::<product_fields::Name, _, _>(names, |filter, name| {
    ///     filter.eq::<product_fields::Name, _>(name.to_string())
    /// });
    ///
    /// // Resulting BSON:
    /// // { "$or": [
    /// //     { "name": "Laptop" },
    /// //     { "name": "Smartphone" },
    /// //     { "name": "Tablet" }
    /// // ]}
    /// ```
    ///
    /// # Complex Example with Multiple Conditions
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product {
    ///     pub name: String,
    ///     pub category: String,
    ///     pub price: f64,
    /// }
    ///
    /// // Filter for products in specific price ranges
    /// let price_ranges = vec![(0.0, 100.0), (500.0, 1000.0), (2000.0, 5000.0)];
    ///
    /// empty::<Product>().or::<product_fields::Price, _, _>(price_ranges, |filter, (min, max)| {
    ///     filter.gte::<product_fields::Price, _>(min)
    ///           .lte::<product_fields::Price, _>(max)
    /// });
    ///
    /// // Resulting BSON (note: multiple clauses from each iteration are flattened):
    /// // { "$or": [
    /// //     { "price": { "$gte": 0.0 } },
    /// //     { "price": { "$lte": 100.0 } },
    /// //     { "price": { "$gte": 500.0 } },
    /// //     { "price": { "$lte": 1000.0 } },
    /// //     { "price": { "$gte": 2000.0 } },
    /// //     { "price": { "$lte": 5000.0 } }
    /// // ]}
    /// ```
    pub fn or<F, V: IntoIterator, N>(&mut self, input: V, f: N) -> &mut Self
    where
        F: FieldName,
        T: HasField<F>,
        N: Fn(&mut FilterBuilder<T>, V::Item) -> &mut FilterBuilder<T>,
    {
        let mut nested = empty::<T>();
        let mut or_clauses: Vec<bson::Document> = vec![];

        for value in input {
            f(&mut nested, value);

            match nested.clauses.len() {
                0 => continue, // Skip empty nested clauses
                1 => or_clauses.push(nested.clauses.clone().into_iter().next().unwrap()),
                _ => or_clauses.extend(nested.clauses.clone()),
            }

            nested.clauses.clear(); // Clear for next iteration
        }

        self.clauses.push(bson::doc! { "$or": or_clauses });

        self
    }

    /// Create a type-safe version of MongoDB's "$not" operator.
    ///
    /// Such MongoDB filter negates operations on a specific field.
    /// This method uses a `FieldFilterBuilder` to construct the operations to be negated.
    ///
    /// # Type parameters:
    /// * `F` - The field name marker type (e.g., `product_fields::Price`)
    /// * `B` - A closure that takes a `FieldFilterBuilder` and returns it with configured operations
    ///
    /// # Arguments
    /// * `f` - A closure that builds the operations to be negated using the `FieldFilterBuilder`
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product {
    ///     pub name: String,
    ///     pub price: f64,
    ///     pub category: String,
    /// }
    ///
    /// // Filter for products where the name is NOT "Smartphone"
    /// empty::<Product>().not::<product_fields::Name, _>(|op| {
    ///     op.eq("Smartphone".to_string())
    /// });
    /// // Resulting BSON: { "name": { "$not": { "name": "Smartphone" } } }
    ///
    /// // Filter for products where the price is NOT equal to 500.0
    /// empty::<Product>().not::<product_fields::Price, _>(|op| {
    ///     op.eq(500.0)
    /// });
    /// // Resulting BSON: { "price": { "$not": { "price": 500.0 } } }
    /// ```
    ///
    /// # MongoDB Behavior
    ///
    /// The `$not` operator in MongoDB performs logical NOT operation on the specified expression.
    /// It can be used to negate the result of any MongoDB expression, including:
    ///
    /// - Equality checks
    /// - Range queries  
    /// - Pattern matching
    /// - Other conditional expressions
    ///
    /// Note that `$not` affects the semantics of the query and can behave differently than
    /// using `$ne` (not equal) for simple equality checks, especially with missing fields.
    pub fn not<F, B>(&mut self, f: B) -> &mut Self
    where
        F: FieldName,
        T: HasField<F>,
        B: FnOnce(FieldFilterBuilder<F, T>) -> FieldFilterBuilder<F, T>,
    {
        let op_builder = FieldFilterBuilder::new();
        let prepared_ops = f(op_builder).build();
        let bson_path = self.field_path::<F>();

        self.clauses
            .push(bson::doc! { bson_path: bson::doc! { "$not": prepared_ops } });

        self
    }

    /// Combines all clauses into a single BSON document,
    /// according to the MongoDB `$and` semantics.
    ///
    /// - If no clauses exist, returns an empty document `{}`
    /// - If only one clause exists, returns that clause directly
    /// - If multiple clauses exist, wraps them in a `$and` array
    ///
    /// This method borrows the builder, allowing you to continue using it afterwards.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::filters::empty;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub name: String,
    ///     pub age: i32,
    ///     pub email: String,
    /// }
    ///
    /// let filter = empty::<User>()
    ///     .eq::<user_fields::Name, _>("John Doe".to_string())
    ///     .gt::<user_fields::Age, _>(18)
    ///     .exists::<user_fields::Email>(true)
    ///     .and();
    /// // builder can still be used here
    /// ```
    ///
    /// Resulting BSON:
    /// ```text
    /// { "$and": [{ "name": "John Doe" }, { "age": { "$gt": 18 } }, { "email": { "$exists": true } }] }
    /// ```
    pub fn and(&self) -> bson::Document {
        if self.clauses.is_empty() {
            bson::doc! {}
        } else if self.clauses.len() == 1 {
            self.clauses[0].clone()
        } else {
            bson::doc! { "$and": self.clauses.clone() }
        }
    }
}

impl<T> Default for FilterBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Converts a FilterBuilder into a BSON document using MongoDB's `$and` semantics.
///
/// This implementation allows FilterBuilder to be automatically converted to a BSON document
/// in contexts where `Into<bson::Document>` is expected, providing a convenient way to use
/// the builder directly with MongoDB operations.
///
/// The conversion follows the same logic as the `and()` method:
/// - If no clauses exist, returns an empty document `{}`
/// - If only one clause exists, returns that clause directly
/// - If multiple clauses exist, wraps them in a `$and` array
///
/// # Example
///
/// ```rust
/// use tnuctipun::filters::empty;
/// use serde::{Serialize, Deserialize};
/// use bson;
///
/// // Self-contained example with manual trait implementations
/// # use tnuctipun::field_witnesses::{FieldName, HasField};
/// # use tnuctipun::mongo_comparable::MongoComparable;
/// # struct Name;
/// # impl FieldName for Name {
/// #     fn field_name() -> &'static str { "name" }
/// # }
/// # struct Age;
/// # impl FieldName for Age {
/// #     fn field_name() -> &'static str { "age" }
/// # }
/// # struct User { name: String, age: i32 }
/// # impl HasField<Name> for User {
/// #     type Value = String;
/// #     fn get_field(&self) -> &Self::Value { &self.name }
/// # }
/// # impl HasField<Age> for User {
/// #     type Value = i32;
/// #     fn get_field(&self) -> &Self::Value { &self.age }
/// # }
/// # impl MongoComparable<String, String> for User {}
/// # impl MongoComparable<i32, i32> for User {}
///
/// // The builder can be automatically converted to bson::Document
/// let mut builder = empty::<User>();
/// builder.eq::<Name, _>("John".to_string())
///        .gt::<Age, _>(18);
/// let filter: bson::Document = builder.into();
///
/// // Or use directly in function calls expecting Into<bson::Document>
/// fn process_filter(filter: impl Into<bson::Document>) {
///     let doc = filter.into();
///     // ... use doc for MongoDB query
/// }
///
/// let mut builder2 = empty::<User>();
/// builder2.eq::<Name, _>("Alice".to_string());
/// process_filter(builder2);
/// ```
impl<T> From<FilterBuilder<T>> for bson::Document {
    fn from(val: FilterBuilder<T>) -> Self {
        val.and()
    }
}

/// Creates an empty FilterBuilder instance.
///
/// This is a convenience function that creates a new FilterBuilder.
/// It's equivalent to `FilterBuilder::<T>::new()` but with less typing.
///
/// # Example
///
/// ```rust
/// use tnuctipun::filters::empty;
/// use tnuctipun::{FieldWitnesses, MongoComparable};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
/// struct User { pub Name: String }
///
/// // Create and use a filter builder in one chain
/// empty::<User>().eq::<user_fields::Name, _>("John".to_string());
/// ```
pub fn empty<T>() -> FilterBuilder<T> {
    FilterBuilder::new()
}

// Testing internal/private functions

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_witnesses::FieldName;

    // Test field marker types
    struct Name;
    impl FieldName for Name {
        fn field_name() -> &'static str {
            "Name"
        }
    }

    struct Category;
    impl FieldName for Category {
        fn field_name() -> &'static str {
            "Category"
        }
    }

    struct TestStruct;

    #[test]
    fn test_field_path_empty_prefix() {
        let builder = FilterBuilder::<TestStruct>::new();
        let path = builder.field_path::<Name>();

        assert_eq!(path, "Name");
    }

    #[test]
    fn test_field_path_single_prefix() {
        let mut builder = FilterBuilder::<TestStruct>::new();

        builder.prefix = vec!["user".to_string()];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "user.Name");
    }

    #[test]
    fn test_field_path_multiple_prefix() {
        let mut builder = FilterBuilder::<TestStruct>::new();

        builder.prefix = vec!["profile".to_string(), "details".to_string()];

        let path = builder.field_path::<Category>();

        assert_eq!(path, "profile.details.Category");
    }

    #[test]
    fn test_field_path_nested_deep() {
        let mut builder = FilterBuilder::<TestStruct>::new();

        builder.prefix = vec![
            "root".to_string(),
            "level1".to_string(),
            "level2".to_string(),
            "level3".to_string(),
        ];

        let path = builder.field_path::<Name>();

        assert_eq!(path, "root.level1.level2.level3.Name");
    }
}
