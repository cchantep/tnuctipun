use crate::field_witnesses::{FieldName, HasField};
use crate::mongo_comparable::MongoComparable;
use bson;

/// A builder for operation-specific filters.
///
/// # Type Parameters
///
/// * `F` - The field name marker type that this operation builder targets
/// * `T` - The struct type that contains the field `F`
pub struct FieldFilterBuilder<F: FieldName, T: HasField<F>> {
    ops: Vec<(&'static str, bson::Bson)>,
    _marker: std::marker::PhantomData<(F, T)>,
}

impl<F, T> Default for FieldFilterBuilder<F, T>
where
    F: FieldName,
    T: HasField<F>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<F, T> FieldFilterBuilder<F, T>
where
    F: FieldName,
    T: HasField<F>,
{
    /// Creates a new FieldFilterBuilder instance.
    ///
    /// # Arguments
    ///
    /// * `build` - A closure that takes a BSON document and returns a FilterBuilder for the target struct
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    // ---

    /// Type-safe equality operation for the FieldFilterBuilder.
    ///
    /// Adds an equality operation to the current operation builder, which can later
    /// be built into a FilterBuilder with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for equality
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::{FieldFilterBuilder};
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { pub name: String }
    ///
    /// // Create an operation builder and add an equality operation
    /// let filter_doc = FieldFilterBuilder::<user_fields::Name, User>::new()
    ///     .eq("John Doe".to_string())
    ///     .build();
    /// ```
    pub fn eq<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        // For equality, we store it with the $eq operator
        self.ops.push(("$eq", value.into()));

        self
    }

    /// Type-safe greater than operation for the FieldFilterBuilder.
    ///
    /// Adds a greater than operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for greater than comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub price: f64 }
    ///
    /// // Create an operation builder and add a greater than operation
    /// let filter_doc = FieldFilterBuilder::<product_fields::Price, Product>::new()
    ///     .gt(100.0)
    ///     .build();
    /// // Resulting BSON: { "price": { "$gt": 100.0 } }
    /// ```
    pub fn gt<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$gt", value.into()));

        self
    }

    /// Type-safe greater than or equal operation for the FieldFilterBuilder.
    ///
    /// Adds a greater than or equal operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for greater than or equal comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub rating: f64 }
    ///
    /// // Create an operation builder and add a greater than or equal operation
    /// let filter_doc = FieldFilterBuilder::<product_fields::Rating, Product>::new()
    ///     .gte(4.5)
    ///     .build();
    /// // Resulting BSON: { "rating": { "$gte": 4.5 } }
    /// ```
    pub fn gte<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$gte", value.into()));

        self
    }

    /// Type-safe less than operation for the FieldFilterBuilder.
    ///
    /// Adds a less than operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for less than comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub stock: i32 }
    ///
    /// // Create an operation builder and add a less than operation
    /// let filter_doc = FieldFilterBuilder::<product_fields::Stock, Product>::new()
    ///     .lt(10)
    ///     .build();
    /// // Resulting BSON: { "stock": { "$lt": 10 } }
    /// ```
    pub fn lt<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$lt", value.into()));

        self
    }

    /// Type-safe less than or equal operation for the FieldFilterBuilder.
    ///
    /// Adds a less than or equal operation to the current operation builder, which can later
    /// be built into a MongoDB filter with the configured operations.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `value` - The value to compare the field against for less than or equal comparison
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub price: f64 }
    ///
    /// // Create an operation builder and add a less than or equal operation
    /// let filter_doc = FieldFilterBuilder::<product_fields::Price, Product>::new()
    ///     .lte(99.99)
    ///     .build();
    /// // Resulting BSON: { "price": { "$lte": 99.99 } }
    /// ```
    pub fn lte<V>(mut self, value: V) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        self.ops.push(("$lte", value.into()));

        self
    }

    /// Type-safe "in" operation for the FieldFilterBuilder.
    ///
    /// Adds an "in" operation to the current operation builder, which matches any of the
    /// values in the provided array and can later be built into a MongoDB filter.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `values` - A vector of values to match against the field
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct User { pub age: i32 }
    ///
    /// // Create an operation builder and add an "in" operation
    /// let filter_doc = FieldFilterBuilder::<user_fields::Age, User>::new()
    ///     .r#in(vec![20, 30, 40])
    ///     .build();
    /// // Resulting BSON: { "age": { "$in": [20, 30, 40] } }
    /// ```
    pub fn r#in<V>(mut self, values: Vec<V>) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.ops.push(("$in", bson_values.into()));

        self
    }

    /// Type-safe "not in" operation for the FieldFilterBuilder.
    ///
    /// Adds a "not in" operation to the current operation builder, which matches values
    /// NOT in the provided array and can later be built into a MongoDB filter.
    ///
    /// # Type parameters:
    /// * `V` - The type of the field value or a compatible type
    ///
    /// # Arguments
    /// * `values` - A vector of values that the field should NOT match
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
    /// struct Product { pub category: String }
    ///
    /// // Create an operation builder and add a "not in" operation
    /// let filter_doc = FieldFilterBuilder::<product_fields::Category, Product>::new()
    ///     .nin(vec![
    ///         "Clothing".to_string(),
    ///         "Shoes".to_string()
    ///     ])
    ///     .build();
    /// // Resulting BSON: { "category": { "$nin": ["Clothing", "Shoes"] } }
    /// ```
    pub fn nin<V>(mut self, values: Vec<V>) -> Self
    where
        T: MongoComparable<T::Value, V>,
        V: Into<bson::Bson> + Clone,
    {
        let bson_values: Vec<bson::Bson> = values.into_iter().map(|v| v.into()).collect();

        self.ops.push(("$nin", bson_values.into()));

        self
    }

    /// Type-safe "exists" operation for the FieldFilterBuilder.
    ///
    /// Adds an "exists" operation to the current operation builder, which checks if a field
    /// exists in the document and can later be built into a MongoDB filter.
    ///
    /// # Arguments
    /// * `exists` - Whether the field should exist (true) or not exist (false)
    ///
    /// # Returns
    /// Returns self for method chaining by value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::field_filters::FieldFilterBuilder;
    /// use tnuctipun::FieldWitnesses;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses)]
    /// struct User { pub phone_number: Option<String> }
    ///
    /// // Create an operation builder and add an "exists" operation
    /// let filter_doc = FieldFilterBuilder::<user_fields::PhoneNumber, User>::new()
    ///     .exists(true)
    ///     .build();
    /// // Resulting BSON: { "phone_number": { "$exists": true } }
    /// ```
    pub fn exists(mut self, exists: bool) -> Self {
        self.ops.push(("$exists", exists.into()));

        self
    }

    /// Builds the configured operations into a FilterBuilder.
    ///
    /// This method consumes the FieldFilterBuilder and transforms all accumulated
    /// operations into a FilterBuilder by creating a BSON document from the operations
    /// and passing it to the build closure provided during construction.
    ///
    /// # Returns
    ///
    /// Returns a `FilterBuilder<T>` that contains the configured operations,
    /// ready to be used for further filter building or converted to a final BSON document.
    pub fn build(self) -> bson::Document {
        let field_name = F::field_name().to_string();

        if self.ops.is_empty() {
            return bson::doc! {};
        }

        // Special handling for equality: MongoDB allows both { field: value } and { field: { $eq: value } }
        // For simplicity with other operations, we'll use the explicit $eq form
        let mut operations = bson::Document::new();

        for (op_name, value) in self.ops {
            operations.insert(op_name, value);
        }

        // Return document with operations
        bson::doc! { field_name: operations }
    }
}
