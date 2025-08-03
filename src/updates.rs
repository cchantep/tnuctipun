use bson;

use num_traits::Num;
use std::collections::HashMap;

use crate::field_witnesses::{FieldName, HasField};
use crate::path::Path;

pub struct UpdateBuilder<T> {
    pub prefix: Vec<String>,
    clauses: HashMap<UpdateOperation, Vec<(String, bson::Bson)>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for UpdateBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UpdateBuilder<T> {
    /// Creates a new `UpdateBuilder` instance.
    ///
    /// The builder starts with an empty prefix and no update clauses.
    /// This is the foundation for constructing MongoDB update documents
    /// using the fluent API pattern.
    ///
    /// # Returns
    ///
    /// A new `UpdateBuilder` instance ready for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::updates::UpdateBuilder;
    ///
    /// struct User {
    ///     pub name: String,
    ///     pub age: i32,
    /// }
    ///
    /// let builder = UpdateBuilder::<User>::new();
    /// ```
    pub fn new() -> Self {
        UpdateBuilder {
            prefix: Vec::new(),
            clauses: HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a fully qualified field path for the given field name marker type.
    ///
    /// This method constructs the complete dot-notation path for a field by combining
    /// any existing prefix (for nested document updates) with the field name.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Returns
    ///
    /// A `String` containing the fully qualified field path.
    ///
    /// # Examples
    ///
    /// - Without prefix: `"field_name"`
    /// - With prefix `["parent"]`: `"parent.field_name"`
    /// - With nested prefix `["root", "child"]`: `"root.child.field_name"`
    fn field_path<F: FieldName>(&self) -> String {
        if self.prefix.is_empty() {
            F::field_name().to_string()
        } else {
            format!("{}.{}", self.prefix.join("."), F::field_name())
        }
    }

    /// Adds a new clause to the update document for the specified operation.
    ///
    /// This method organizes update clauses by operation type, allowing multiple
    /// fields to be updated with the same operation (e.g., multiple `$set` operations
    /// are combined into a single `$set` document).
    ///
    /// # Parameters
    ///
    /// * `op` - The update operation type (e.g., Set, Unset, Inc)
    /// * `path` - The field path to update
    /// * `clause` - The BSON value for the update
    fn push_clause(&mut self, op: UpdateOperation, path: String, clause: bson::Bson) {
        // If nothing exists for key `op` in clauses, create a new vector
        self.clauses.entry(op).or_default().push((path, clause));
    }

    /// Sets the value of a field in the document.
    ///
    /// This method corresponds to MongoDB's `$set` operator, which sets the value of a field.
    /// If the field does not exist, `$set` will add a new field with the specified value.
    /// If the field does exist, `$set` will replace the existing value.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `V` - A value type that can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `value` - The value to set for the field
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    /// use bson::doc;
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct User {
    ///     pub name: String,
    ///     pub age: i32,
    /// }
    ///
    /// let update_doc = empty::<User>()
    ///     .set::<user_fields::Name, _>("Jane Doe".to_string())
    ///     .set::<user_fields::Age, _>(25)
    ///     .build();
    /// // Results in: { "$set": { "name": "Jane Doe", "age": 25 } }
    /// ```
    pub fn set<F: FieldName, V: Into<bson::Bson>>(&mut self, value: V) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Set, path, value.into());

        self
    }

    /// Removes a field from the document.
    ///
    /// This method corresponds to MongoDB's `$unset` operator, which deletes a particular field.
    /// The operation removes the field completely from the document. If the field does not exist,
    /// the operation has no effect but is not considered an error.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct TempData {
    ///     pub temp_data: String,
    ///     pub obsolete_field: i32,
    /// }
    ///
    /// let update_doc = empty::<TempData>()
    ///     .unset::<tempdata_fields::TempData>()
    ///     .unset::<tempdata_fields::ObsoleteField>()
    ///     .build();
    /// // Results in: { "$unset": { "temp_data": "", "obsolete_field": "" } }
    /// ```
    pub fn unset<F: FieldName>(&mut self) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Unset, path, bson::Bson::Null);

        self
    }

    /// Increments the value of a numeric field by the specified amount.
    ///
    /// This method corresponds to MongoDB's `$inc` operator, which increments a field by a specified value.
    /// The field must contain a numeric value (integer, long, double, or decimal). If the field does not exist,
    /// it is created with the increment value. If the field exists but is not numeric, the operation will fail.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `N` - A numeric type that implements `Num` and can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `value` - The amount to increment the field by (can be negative for decrementing)
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Stats {
    ///     pub count: i32,
    ///     pub score: i32,
    /// }
    ///
    /// let update_doc = empty::<Stats>()
    ///     .inc::<stats_fields::Count, _>(1)
    ///     .inc::<stats_fields::Score, _>(-5)
    ///     .build();
    /// // Results in: { "$inc": { "count": 1, "score": -5 } }
    /// ```
    pub fn inc<F: FieldName, N: Num + Into<bson::Bson>>(&mut self, value: N) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Inc, path, value.into());

        self
    }

    /// Multiplies the value of a numeric field by the specified amount.
    ///
    /// This method corresponds to MongoDB's `$mul` operator, which multiplies the value of a field by a number.
    /// The field must contain a numeric value (integer, long, double, or decimal). If the field does not exist,
    /// it is created with a value of zero (0 * multiplier = 0). If the field exists but is not numeric, the operation will fail.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `N` - A numeric type that implements `Num` and can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `value` - The multiplier to apply to the field value
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Product {
    ///     pub price: f64,
    ///     pub quantity: i32,
    /// }
    ///
    /// let update_doc = empty::<Product>()
    ///     .mul::<product_fields::Price, _>(1.1)      // Increase price by 10%
    ///     .mul::<product_fields::Quantity, _>(2)     // Double the quantity
    ///     .build();
    /// // Results in: { "$mul": { "price": 1.1, "quantity": 2 } }
    /// ```
    pub fn mul<F: FieldName, N: Num + Into<bson::Bson>>(&mut self, value: N) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Mul, path, value.into());

        self
    }

    /// Renames a field in the document.
    ///
    /// This method corresponds to MongoDB's `$rename` operator, which renames a field.
    /// The new field name must differ from the existing field name. If the target field name
    /// already exists, its value will be overwritten. If the source field does not exist,
    /// the operation has no effect.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Parameters
    ///
    /// * `new_name` - The new name for the field
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Document {
    ///     pub old_name: String,
    ///     pub legacy_field: String,
    /// }
    ///
    /// let update_doc = empty::<Document>()
    ///     .rename::<document_fields::OldName>("new_name")
    ///     .rename::<document_fields::LegacyField>("modern_field")
    ///     .build();
    /// // Results in: { "$rename": { "old_name": "new_name", "legacy_field": "modern_field" } }
    /// ```
    pub fn rename<F: FieldName>(&mut self, new_name: &str) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.push_clause(
            UpdateOperation::Rename,
            path,
            bson::Bson::String(new_name.to_string()),
        );

        self
    }

    /// Sets the value of a field to the current date.
    ///
    /// This method corresponds to MongoDB's `$currentDate` operator, which sets the value of a field
    /// to the current date, either as a BSON Date or a BSON Timestamp. The default type is Date.
    /// This is useful for tracking when documents were last modified.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Parameters
    ///
    /// * `date_type` - The type of date value to set (Date or Timestamp)
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::{empty, CurrentDateType}};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Timestamps {
    ///     pub last_modified: String, // In practice, this would be a proper date type
    ///     pub updated_at: String,    // In practice, this would be a proper date type
    /// }
    ///
    /// let update_doc = empty::<Timestamps>()
    ///     .current_date::<timestamps_fields::LastModified>(CurrentDateType::Date)
    ///     .current_date::<timestamps_fields::UpdatedAt>(CurrentDateType::Timestamp)
    ///     .build();
    /// // Results in: { "$currentDate": { "last_modified": "date", "updated_at": "timestamp" } }
    /// ```
    pub fn current_date<F: FieldName>(&mut self, date_type: CurrentDateType) -> &mut Self
    where
        T: HasField<F>,
    {
        let path = self.field_path::<F>();

        self.push_clause(
            UpdateOperation::CurrentDate,
            path,
            bson::Bson::String(date_type.to_string()),
        );

        self
    }

    /// Adds a value to an array field only if it does not already exist.
    ///
    /// This method corresponds to MongoDB's `$addToSet` operator, which adds a value to an array
    /// unless the value is already present, in which case it does nothing to that array.
    /// This ensures array uniqueness without duplicates. If the field is not an array,
    /// the operation will fail. If the field does not exist, it creates a new array with the value.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `V` - A value type that can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `value` - The value to add to the array if it's not already present
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    /// use bson::doc;
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Article {
    ///     pub tags: Vec<String>,
    ///     pub categories: Vec<String>,
    /// }
    ///
    /// let update_doc = empty::<Article>()
    ///     .add_to_set::<article_fields::Tags, _>("rust".to_string())
    ///     .add_to_set::<article_fields::Categories, _>("programming".to_string())
    ///     .build();
    /// // Results in: { "$addToSet": { "tags": "rust", "categories": "programming" } }
    /// ```
    pub fn add_to_set<F: FieldName, V: Into<bson::Bson>>(&mut self, value: V) -> &mut Self
    where
        T: HasField<F>,
        T::Value: IntoIterator<Item = V>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::AddToSet, path, value.into());

        self
    }

    /// Removes the first or last element from an array field.
    ///
    /// This method corresponds to MongoDB's `$pop` operator, which removes either the first
    /// or the last element from an array. The operation fails if the field is not an array.
    /// If the array is empty, the operation has no effect.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Parameters
    ///
    /// * `strategy` - The strategy for which element to remove (`PopStrategy::First` or `PopStrategy::Last`)
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, updates::{empty, PopStrategy}};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Collections {
    ///     pub queue: Vec<String>,
    ///     pub stack: Vec<String>,
    /// }
    ///
    /// let update_doc = empty::<Collections>()
    ///     .pop::<collections_fields::Queue>(PopStrategy::First)    // Remove first element (FIFO)
    ///     .pop::<collections_fields::Stack>(PopStrategy::Last)     // Remove last element (LIFO)
    ///     .build();
    /// // Results in: { "$pop": { "queue": -1, "stack": 1 } }
    /// ```
    pub fn pop<F: FieldName>(&mut self, strategy: PopStrategy) -> &mut Self
    where
        T: HasField<F>,
        T::Value: IntoIterator,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Pop, path, strategy.into());

        self
    }

    /// Removes array elements that match a conditional BSON expression.
    ///
    /// This method corresponds to MongoDB's `$pull` operator with a conditional expression,
    /// which removes from an existing array all instances of values that match the specified condition.
    /// This is useful for complex matching scenarios where you need to remove elements based on
    /// embedded document fields or complex criteria.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    ///
    /// # Parameters
    ///
    /// * `cond` - A BSON expression that defines the condition for elements to be removed
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    /// use bson::{doc, Bson};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Inventory {
    ///     pub items: Vec<bson::Document>,
    /// }
    ///
    /// // Remove all items where quantity is less than 5
    /// let condition = doc! { "quantity": { "$lt": 5 } };
    ///
    /// let update_doc = empty::<Inventory>()
    ///     .pull_expr::<inventory_fields::Items>(Bson::Document(condition))
    ///     .build();
    /// // Results in: { "$pull": { "items": { "quantity": { "$lt": 5 } } } }
    /// ```
    pub fn pull_expr<F: FieldName>(&mut self, cond: bson::Bson) -> &mut Self
    where
        T: HasField<F>,
        T::Value: IntoIterator,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Pull, path, cond);

        self
    }

    /// Removes all array elements that match a specific value.
    ///
    /// This method corresponds to MongoDB's `$pull` operator with a simple value match,
    /// which removes from an existing array all instances of the specified value.
    /// This is the simpler version of `pull_expr` for exact value matching.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `V` - A value type that can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `value` - The exact value to remove from the array
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Content {
    ///     pub tags: Vec<String>,
    ///     pub scores: Vec<i32>,
    /// }
    ///
    /// let update_doc = empty::<Content>()
    ///     .pull::<content_fields::Tags, _>("deprecated".to_string())
    ///     .pull::<content_fields::Scores, _>(0)
    ///     .build();
    /// // Results in: { "$pull": { "tags": "deprecated", "scores": 0 } }
    /// ```
    pub fn pull<F: FieldName, V: Into<bson::Bson>>(&mut self, value: V) -> &mut Self
    where
        T: HasField<F>,
        T::Value: IntoIterator<Item = V>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Pull, path, value.into());

        self
    }

    /// Removes all instances of specified values from an array.
    ///
    /// This method corresponds to MongoDB's `$pullAll` operator, which removes all instances
    /// of the specified values from an existing array. Unlike `$pull`, which removes elements
    /// by specifying a condition, `$pullAll` removes elements that match any of the listed values.
    /// The field must be an array, or the operation will fail.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `I` - An iterable type that can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `values` - An iterable collection of values to remove from the array
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    /// use bson::Bson;
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Lists {
    ///     pub tags: Vec<String>,
    ///     pub numbers: Vec<i32>,
    /// }
    ///
    /// let tags_to_remove = vec!["old".to_string(), "deprecated".to_string(), "unused".to_string()];
    /// let numbers_to_remove = vec![1i32, 3i32, 5i32, 7i32];
    ///
    /// empty::<Lists>()
    ///     .pull_all::<lists_fields::Tags, _>(tags_to_remove)
    ///     .pull_all::<lists_fields::Numbers, _>(numbers_to_remove)
    ///     .build();
    /// // Results in: { "$pullAll": { "tags": ["old", "deprecated", "unused"], "numbers": [1, 3, 5, 7] } }
    /// ```
    pub fn pull_all<F: FieldName, I>(&mut self, values: I) -> &mut Self
    where
        T: HasField<F>,
        I: Into<bson::Bson> + IntoIterator,
        T::Value: IntoIterator<Item = I::Item>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::PullAll, path, values.into());

        self
    }

    /// Appends a value to an array field.
    ///
    /// This method corresponds to MongoDB's `$push` operator, which appends a specified value
    /// to an array. If the field is absent, it creates a new array field with the value as its element.
    /// If the field exists but is not an array, the operation will fail. Unlike `$addToSet`,
    /// `$push` allows duplicate values to be added to the array.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A field name marker type that implements `FieldName`
    /// * `V` - A value type that can be converted into `bson::Bson`
    ///
    /// # Parameters
    ///
    /// * `value` - The value to append to the array
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldName, FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Logging {
    ///     pub logs: Vec<String>,
    ///     pub history: Vec<String>,
    /// }
    ///
    /// let update_doc = empty::<Logging>()
    ///     .push::<logging_fields::Logs, _>("User logged in".to_string())
    ///     .push::<logging_fields::History, _>("Action performed".to_string())
    ///     .build();
    /// // Results in: { "$push": { "logs": "User logged in", "history": "Action performed" } }
    /// ```
    pub fn push<F: FieldName, V: Into<bson::Bson>>(&mut self, value: V) -> &mut Self
    where
        T: HasField<F>,
        T::Value: IntoIterator<Item = V>,
    {
        let path = self.field_path::<F>();

        self.push_clause(UpdateOperation::Push, path, value.into());

        self
    }

    /// Performs nested field updates using a path lookup function.
    ///
    /// This method enables type-safe updates on nested document structures by providing
    /// a lookup function that navigates to the target nested field, and a configuration
    /// function that defines the update operations to apply to that nested context.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The field name marker type for the starting field that implements `FieldName`
    /// * `L` - The lookup function type that navigates from the starting field to the target field
    /// * `G` - The field name marker type for the target nested field that implements `FieldName`
    /// * `U` - The target struct type that contains the nested field and implements `HasField<G>`
    /// * `N` - The configuration function type that defines update operations on the nested field
    ///
    /// # Parameters
    ///
    /// * `lookup` - A function that takes a path to field `F` and returns a path to the target field `G`
    /// * `f` - A function that takes an `UpdateBuilder<U>` for the nested context and returns it configured with update operations
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Address {
    ///     pub street: String,
    ///     pub city: String,
    ///     pub zip_code: String,
    /// }
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct User {
    ///     pub name: String,
    ///     pub home_address: Address,
    ///     pub work_address: Address,
    /// }
    ///
    /// // Update nested field in home address
    /// let update_doc = empty::<User>()
    ///     .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
    ///         |path| path.field::<address_fields::City>(),
    ///         |nested| {
    ///             nested.set::<address_fields::City, _>("San Francisco".to_string());
    ///         }
    ///     )
    ///     .build();
    /// // Results in: { "$set": { "home_address.city": "San Francisco" } }
    /// ```
    ///
    /// # Usage with Multiple Nested Operations
    ///
    /// ```rust,ignore
    /// use nessus::{FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// // Using the same structs from above
    /// let update_doc = empty::<User>()
    ///     .with_lookup::<user_fields::HomeAddress, _, address_fields::City, Address, _>(
    ///         |path| path.field::<address_fields::City>(),
    ///         |mut nested| {
    ///             nested
    ///                 .set::<address_fields::City, _>("Boston".to_string())
    ///                 .set::<address_fields::ZipCode, _>("02101".to_string());
    ///             nested
    ///         }
    ///     )
    ///     .with_lookup::<user_fields::WorkAddress, _, address_fields::Street, Address, _>(
    ///         |path| path.field::<address_fields::Street>(),
    ///         |mut nested| {
    ///             nested.set::<address_fields::Street, _>("123 Business Ave".to_string());
    ///             nested
    ///         }
    ///     )
    ///     .build();
    /// // Results in: {
    /// //   "$set": {
    /// //     "home_address.city": "Boston",
    /// //     "home_address.zip_code": "02101",
    /// //     "work_address.street": "123 Business Ave"
    /// //   }
    /// // }
    /// ```
    pub fn with_lookup<F: FieldName, L, G: FieldName, U: HasField<G>, N>(
        &mut self,
        lookup: L,
        f: N,
    ) -> &mut Self
    where
        T: HasField<F>,
        L: FnOnce(&Path<F, T, T>) -> Path<G, U, T>,
        N: FnOnce(&mut UpdateBuilder<U>),
    {
        // Create a base field path for the lookup
        let base_field: Path<F, T, T> = Path {
            prefix: self.prefix.clone(),
            _marker: std::marker::PhantomData,
        };

        // Resolve the field path using the provided lookup function
        let resolved_field = lookup(&base_field);

        // Create a new UpdateBuilder for the nested field
        let mut nested_builder = UpdateBuilder::<U> {
            prefix: resolved_field.prefix.clone(),
            clauses: HashMap::new(),
            _marker: std::marker::PhantomData,
        };

        f(&mut nested_builder);

        // Merge the nested clauses properly into the main builder
        for (operation, clauses_vec) in nested_builder.clauses {
            self.clauses
                .entry(operation)
                .or_default()
                .extend(clauses_vec);
        }

        self
    }

    /// Convenience method for updating fields using identity lookup.
    ///
    /// This method provides a simpler interface for field updates when you don't need
    /// to navigate to nested fields. It's equivalent to calling `with_lookup` with an
    /// identity function that returns the same field path unchanged.
    ///
    /// This is particularly useful for applying update operations within the current
    /// document context without having to deal with path navigation.
    ///
    /// # Type Parameters
    ///
    /// * `F` - The field name marker type that implements `FieldName`
    /// * `N` - The configuration function type that defines update operations
    ///
    /// # Parameters
    ///
    /// * `f` - A function that takes an `UpdateBuilder<T>` and returns it configured with update operations
    ///
    /// # Returns
    ///
    /// Returns `&mut Self` to allow method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct User {
    ///     pub name: String,
    ///     pub age: i32,
    ///     pub active: bool,
    /// }
    ///
    /// // Apply multiple operations in the same context
    /// let update_doc = empty::<User>()
    ///     .with_field::<user_fields::Name, _>(|nested| {
    ///         nested
    ///             .set::<user_fields::Name, _>("John Doe".to_string())
    ///             .inc::<user_fields::Age, _>(1)
    ///             .set::<user_fields::Active, _>(true);
    ///     })
    ///     .build();
    /// // Results in: {
    /// //   "$set": {
    /// //     "name": "John Doe",
    /// //     "active": true
    /// //   },
    /// //   "$inc": { "age": 1 }
    /// // }
    /// ```
    ///
    /// # Comparison with Direct Method Calls
    ///
    /// The following two approaches are equivalent:
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Product {
    ///     pub name: String,
    ///     pub price: f64,
    /// }
    ///
    /// // Using with_field
    /// let update_doc1 = empty::<Product>()
    ///     .with_field::<product_fields::Name, _>(|nested| {
    ///         nested.set::<product_fields::Name, _>("Widget".to_string());
    ///     })
    ///     .build();
    ///
    /// // Using direct method calls
    /// let update_doc2 = empty::<Product>()
    ///     .set::<product_fields::Name, _>("Widget".to_string())
    ///     .build();
    ///
    /// // Both produce the same result: { "$set": { "name": "Widget" } }
    /// assert_eq!(update_doc1, update_doc2);
    /// ```
    pub fn with_field<F: FieldName, N>(&mut self, f: N) -> &mut Self
    where
        T: HasField<F>,
        N: FnOnce(&mut UpdateBuilder<T>),
    {
        self.with_lookup::<F, _, F, T, _>(|path| path.clone(), f)
    }

    /// Builds the final MongoDB update document.
    ///
    /// This method consumes the accumulated update operations and produces a
    /// `bson::Document` that can be used directly with MongoDB update queries.
    /// All update clauses are organized by operation type (e.g., `$set`, `$inc`)
    /// and combined into their respective operation documents.
    ///
    /// # Returns
    ///
    /// A `bson::Document` containing all the update operations that were added
    /// to this builder, organized by MongoDB update operator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct User {
    ///     pub name: String,
    ///     pub age: i32,
    /// }
    ///
    /// let update_doc = empty::<User>()
    ///     .set::<user_fields::Name, _>("John".to_string())
    ///     .inc::<user_fields::Age, _>(1)
    ///     .build();
    /// // Results in: {
    /// //   "$set": { "name": "John" },
    /// //   "$inc": { "age": 1 }
    /// // }
    /// ```
    ///
    /// # Behavior with Multiple Operations
    ///
    /// Operations of the same type are combined into a single operation document:
    ///
    /// ```rust
    /// use nessus::{FieldWitnesses, updates::empty};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(FieldWitnesses, Serialize, Deserialize)]
    /// struct Stats {
    ///     pub views: i32,
    ///     pub likes: i32,
    /// }
    ///
    /// let update_doc = empty::<Stats>()
    ///     .inc::<stats_fields::Views, _>(1)
    ///     .inc::<stats_fields::Likes, _>(1)
    ///     .build();
    /// // Results in: {
    /// //   "$inc": {
    /// //     "views": 1,
    /// //     "likes": 1
    /// //   }
    /// // }
    /// ```
    pub fn build(&mut self) -> bson::Document {
        let mut doc = bson::Document::new();

        for (op, op_clauses) in &self.clauses {
            let operation = op.as_str();
            let mut operation_doc = bson::Document::new();

            for (field, clause) in op_clauses {
                operation_doc.insert(field.clone(), clause.clone());
            }

            // Insert the operation document into the main document
            doc.insert(operation, operation_doc);
        }

        doc
    }
}

// ---

/// MongoDB update operations that can be performed on documents.
///
/// This enum represents the various update operators available in MongoDB,
/// each corresponding to a specific type of modification that can be applied
/// to document fields during update operations.
///
/// # Examples
///
/// ```rust
/// use nessus::updates::UpdateOperation;
///
/// let set_op = UpdateOperation::Set;
///
/// assert_eq!(format!("{}", set_op), "$set");
///
/// let inc_op = UpdateOperation::Inc;
///
/// assert_eq!(format!("{}", inc_op), "$inc");
/// ```
#[derive(Eq, Hash, PartialEq)]
pub enum UpdateOperation {
    /// Sets the value of a field in a document.
    ///
    /// Corresponds to MongoDB's `$set` operator, which sets the value of a field.
    /// If the field does not exist, `$set` will add a new field with the specified value.
    Set,

    /// Removes the specified field from a document.
    ///
    /// Corresponds to MongoDB's `$unset` operator, which deletes a particular field.
    /// The specified value in the `$unset` expression does not impact the operation.
    Unset,

    /// Increments the value of a field by the specified amount.
    ///
    /// Corresponds to MongoDB's `$inc` operator, which increments a field by a specified value.
    /// The field must contain a numeric value. If the field does not exist, it is created with the increment value.
    Inc,

    /// Multiplies the value of a field by the specified amount.
    ///
    /// Corresponds to MongoDB's `$mul` operator, which multiplies the value of a field by a number.
    /// The field must contain a numeric value. If the field does not exist, it is created with a value of zero.
    Mul,

    /// Renames a field.
    ///
    /// Corresponds to MongoDB's `$rename` operator, which renames a field.
    /// The new field name must differ from the existing field name.
    Rename,

    /// Sets the value of a field to the current date.
    ///
    /// Corresponds to MongoDB's `$currentDate` operator, which sets the value of a field to the current date,
    /// either as a Date or a timestamp. The default type is Date.
    CurrentDate,

    /// Adds elements to an array only if they do not already exist in the set.
    ///
    /// Corresponds to MongoDB's `$addToSet` operator, which adds a value to an array unless the value
    /// is already present, in which case it does nothing to that array.
    AddToSet,

    /// Removes the first or last item of an array.
    ///
    /// Corresponds to MongoDB's `$pop` operator, which removes the first or last element of an array.
    /// Pass -1 to remove the first element, or 1 to remove the last element.
    Pop,

    /// Removes all array elements that match a specified query.
    ///
    /// Corresponds to MongoDB's `$pull` operator, which removes from an existing array all instances
    /// of a value or values that match a specified condition.
    Pull,

    /// Removes all matching values from an array.
    ///
    /// Corresponds to MongoDB's `$pullAll` operator, which removes all instances of the specified
    /// values from an existing array. Unlike `$pull`, which removes elements by specifying a condition,
    /// `$pullAll` removes elements that match any of the listed values.
    PullAll,

    /// Appends a specified value to an array.
    ///
    /// Corresponds to MongoDB's `$push` operator, which appends a specified value to an array.
    /// If the field is absent, it creates a new array field with the value as its element.
    Push,
}

impl UpdateOperation {
    /// Returns the MongoDB operator string for this update operation.
    pub const fn as_str(&self) -> &'static str {
        match self {
            UpdateOperation::Set => "$set",
            UpdateOperation::Unset => "$unset",
            UpdateOperation::Inc => "$inc",
            UpdateOperation::Mul => "$mul",
            UpdateOperation::Rename => "$rename",
            UpdateOperation::CurrentDate => "$currentDate",
            UpdateOperation::AddToSet => "$addToSet",
            UpdateOperation::Pop => "$pop",
            UpdateOperation::Pull => "$pull",
            UpdateOperation::PullAll => "$pullAll",
            UpdateOperation::Push => "$push",
        }
    }
}

/// Converts `UpdateOperation` variants to their corresponding MongoDB operator strings.
///
/// This implementation allows `UpdateOperation` enum variants to be converted to the
/// string representations expected by MongoDB update operations.
impl std::fmt::Display for UpdateOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ---

/// Type of date value to use with MongoDB's `$currentDate` operator.
///
/// This enum specifies whether to set a field to the current date as a BSON Date
/// or as a BSON Timestamp. The choice affects both the storage format and the
/// precision of the date value in MongoDB.
///
/// # Examples
///
/// ```rust
/// use nessus::updates::CurrentDateType;
///
/// let date_type = CurrentDateType::Date;
///
/// assert_eq!(format!("{}", date_type), "date");
///
/// let timestamp_type = CurrentDateType::Timestamp;
///
/// assert_eq!(format!("{}", timestamp_type), "timestamp");
/// ```
///
/// # Usage with UpdateBuilder
///
/// ```rust
/// use nessus::updates::{UpdateBuilder, CurrentDateType};
/// // Assuming you have appropriate field witnesses set up
///
/// let mut builder = UpdateBuilder::<()>::new();
/// // This would set a field to the current date
/// // builder.current_date::<SomeField>(CurrentDateType::Date);
/// ```
pub enum CurrentDateType {
    /// Sets the field to the current date as a BSON Date.
    ///
    /// This is the default behavior and stores the date with millisecond precision.
    /// BSON Date values are stored as 64-bit integers representing milliseconds
    /// since the Unix epoch (January 1, 1970, 00:00:00 UTC).
    Date,

    /// Sets the field to the current date as a BSON Timestamp.
    ///
    /// BSON Timestamps are MongoDB's internal timestamp type, primarily used
    /// for internal MongoDB operations. They consist of a 32-bit timestamp
    /// (seconds since epoch) and a 32-bit incrementing ordinal for operations
    /// within a given second.
    ///
    /// Note: BSON Timestamps are different from BSON Date and are mainly
    /// intended for internal MongoDB use cases.
    Timestamp,
}

/// Converts `CurrentDateType` variants to their string representations expected by MongoDB.
///
/// This implementation allows `CurrentDateType` enum variants to be converted to the
/// string values that MongoDB expects in `$currentDate` operations.
///
/// # Examples
///
/// ```rust
/// use nessus::updates::CurrentDateType;
///
/// assert_eq!(format!("{}", CurrentDateType::Date), "date");
/// assert_eq!(format!("{}", CurrentDateType::Timestamp), "timestamp");
/// ```
impl std::fmt::Display for CurrentDateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CurrentDateType::Date => "date",
            CurrentDateType::Timestamp => "timestamp",
        };

        write!(f, "{s}")
    }
}

// ---

/// Strategy for removing elements from an array using MongoDB's `$pop` operator.
///
/// This enum specifies which end of an array to remove an element from when using
/// the `$pop` update operation. MongoDB's `$pop` operator removes either the first
/// or the last element from an array field.
///
/// # Examples
///
/// ```rust
/// use nessus::updates::PopStrategy;
/// use bson::Bson;
///
/// let first_strategy = PopStrategy::First;
/// let first_bson: Bson = first_strategy.into();
/// // This converts to Bson::Int32(-1)
///
/// let last_strategy = PopStrategy::Last;
/// let last_bson: Bson = last_strategy.into();
/// // This converts to Bson::Int32(1)
/// ```
///
/// # Usage with UpdateBuilder
///
/// ```rust
/// use nessus::updates::{UpdateBuilder, PopStrategy};
/// // Assuming you have appropriate field witnesses set up
///
/// let mut builder = UpdateBuilder::<()>::new();
/// // This would remove the first element from an array field
/// // builder.pop::<SomeArrayField>(PopStrategy::First);
/// ```
pub enum PopStrategy {
    /// Remove the first element from the array.
    ///
    /// Corresponds to MongoDB's `$pop` with value `-1`, which removes the first
    /// (leftmost) element from an array. This is equivalent to a "shift" operation
    /// in many programming languages.
    First,

    /// Remove the last element from the array.
    ///
    /// Corresponds to MongoDB's `$pop` with value `1`, which removes the last
    /// (rightmost) element from an array. This is equivalent to a "pop" operation
    /// in many programming languages.
    Last,
}

/// Converts `PopStrategy` variants to their corresponding BSON values expected by MongoDB.
///
/// This implementation allows `PopStrategy` enum variants to be converted to the
/// BSON integer values that MongoDB expects for the `$pop` operation:
/// - `PopStrategy::First` converts to `Bson::Int32(-1)`
/// - `PopStrategy::Last` converts to `Bson::Int32(1)`
///
/// # Examples
///
/// ```rust
/// use nessus::updates::PopStrategy;
/// use bson::Bson;
///
/// let strategy = PopStrategy::First;
/// let bson_value: Bson = strategy.into();
///
/// assert_eq!(bson_value, Bson::Int32(-1));
///
/// let strategy = PopStrategy::Last;
/// let bson_value: Bson = strategy.into();
///
/// assert_eq!(bson_value, Bson::Int32(1));
/// ```
impl From<PopStrategy> for bson::Bson {
    fn from(strategy: PopStrategy) -> Self {
        match strategy {
            PopStrategy::First => bson::Bson::Int32(-1),
            PopStrategy::Last => bson::Bson::Int32(1),
        }
    }
}

/// Creates a new empty `UpdateBuilder` instance.
///
/// This function provides a convenient way to start a fluent chain of update operations
/// without needing to explicitly call `UpdateBuilder::new()` and assign it to a mutable variable.
///
/// # Type Parameters
///
/// * `T` - The target struct type that implements the necessary field witness traits
///
/// # Examples
///
/// ```rust
/// use nessus::{FieldWitnesses, updates::empty};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(FieldWitnesses, Serialize, Deserialize)]
/// struct User {
///     pub id: String,
///     pub name: String,
///     pub email: String,
///     pub age: i32,
/// }
///
/// // Use method chaining (recommended)
/// let update_doc = empty::<User>()
///     .set::<user_fields::Name, _>("John Doe".to_string())
///     .inc::<user_fields::Age, _>(1)
///     .unset::<user_fields::Email>()
///     .build();
/// // Results in: {
/// //   "$set": { "name": "John Doe" },
/// //   "$inc": { "age": 1 },
/// //   "$unset": { "email": null }
/// // }
/// ```
pub fn empty<T>() -> UpdateBuilder<T> {
    UpdateBuilder::new()
}

// ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_witnesses::FieldName;

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

    #[test]
    fn test_field_path_empty_prefix() {
        let builder = UpdateBuilder::<()>::new();
        let path = builder.field_path::<TestFieldName>();

        assert_eq!(path, "test_field");
    }

    #[test]
    fn test_field_path_single_prefix() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("parent".to_string());

        let path = builder.field_path::<TestFieldName>();

        assert_eq!(path, "parent.test_field");
    }

    #[test]
    fn test_field_path_multiple_prefix() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("root".to_string());
        builder.prefix.push("parent".to_string());
        builder.prefix.push("child".to_string());

        let path = builder.field_path::<TestFieldName>();

        assert_eq!(path, "root.parent.child.test_field");
    }

    #[test]
    fn test_field_path_different_field_types() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("prefix".to_string());

        let path1 = builder.field_path::<TestFieldName>();

        assert_eq!(path1, "prefix.test_field");

        let path2 = builder.field_path::<AnotherFieldName>();

        assert_eq!(path2, "prefix.another_field");
    }

    #[test]
    fn test_field_path_nested_field_name() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("outer".to_string());

        let path = builder.field_path::<NestedFieldName>();

        assert_eq!(path, "outer.nested.field");
    }

    #[test]
    fn test_field_path_empty_string_prefix() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("".to_string());

        let path = builder.field_path::<TestFieldName>();

        assert_eq!(path, ".test_field");
    }

    #[test]
    fn test_field_path_consistency_across_multiple_calls() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("consistent".to_string());

        let path1 = builder.field_path::<TestFieldName>();
        let path2 = builder.field_path::<TestFieldName>();

        assert_eq!(path1, path2);
        assert_eq!(path1, "consistent.test_field");
    }

    #[test]
    fn test_field_path_deeply_nested_prefix() {
        let mut builder = UpdateBuilder::<()>::new();

        // Simulate a deeply nested structure
        for i in 0..10 {
            builder.prefix.push(format!("level{i}"));
        }

        let path = builder.field_path::<TestFieldName>();

        assert_eq!(
            path,
            "level0.level1.level2.level3.level4.level5.level6.level7.level8.level9.test_field"
        );
    }

    #[test]
    fn test_field_path_special_characters_in_prefix() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("with-dash".to_string());
        builder.prefix.push("with_underscore".to_string());
        builder.prefix.push("with123numbers".to_string());

        let path = builder.field_path::<TestFieldName>();

        assert_eq!(path, "with-dash.with_underscore.with123numbers.test_field");
    }

    #[test]
    fn test_field_path_with_numeric_string_prefix() {
        let mut builder = UpdateBuilder::<()>::new();

        builder.prefix.push("0".to_string());
        builder.prefix.push("123".to_string());

        let path = builder.field_path::<TestFieldName>();

        assert_eq!(path, "0.123.test_field");
    }
}
