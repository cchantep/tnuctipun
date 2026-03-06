use crate::field_witnesses::{FieldName, HasField};
use crate::mongo_comparable::{MongoComparable, MongoOrdered};
use crate::path::Path;

use num_traits::Num;

/// Typed MongoDB aggregation expression.
///
/// `Expr<T, V>` carries:
/// - `T`: root document type used for field witness constraints
/// - `V`: expression value type used for compile-time operator constraints
#[derive(Clone, Debug, PartialEq)]
pub struct Expr<T, V> {
    repr: bson::Bson,
    _marker: std::marker::PhantomData<(T, V)>,
}

impl<T, V> Expr<T, V> {
    /// Creates an expression from a raw BSON representation.
    ///
    /// This bypasses most static guarantees and is intended as an escape hatch
    /// for unsupported operators.
    pub fn unsafe_raw(repr: bson::Bson) -> Self {
        Self {
            repr,
            _marker: std::marker::PhantomData,
        }
    }

    /// Borrows the BSON representation of the expression.
    pub fn as_bson(&self) -> &bson::Bson {
        &self.repr
    }

    /// Consumes the expression and returns its BSON representation.
    pub fn into_bson(self) -> bson::Bson {
        self.repr
    }

    /// Widens the expression value type while keeping the same BSON representation.
    pub fn widen<U>(self) -> Expr<T, U> {
        Expr {
            repr: self.repr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T, V> From<Expr<T, V>> for bson::Bson {
    fn from(expr: Expr<T, V>) -> Self {
        expr.into_bson()
    }
}

/// Builder for typed MongoDB aggregation expressions.
#[derive(Default)]
pub struct ExprBuilder<T> {
    prefix: Vec<String>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> ExprBuilder<T> {
    /// Creates a new expression builder.
    pub fn new() -> Self {
        Self {
            prefix: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates a literal expression from a BSON-convertible value.
    pub fn from<V>(&self, value: V) -> Expr<T, V>
    where
        V: Into<bson::Bson>,
    {
        Expr::unsafe_raw(value.into())
    }

    /// Creates a raw expression from BSON.
    pub fn unsafe_expr<V>(&self, value: bson::Bson) -> Expr<T, V> {
        Expr::unsafe_raw(value)
    }

    /// Returns a field reference expression like `$field` or `$prefix.field`.
    pub fn select<F>(&self) -> Expr<T, T::Value>
    where
        F: FieldName,
        T: HasField<F>,
    {
        let path = if self.prefix.is_empty() {
            F::field_name().to_string()
        } else {
            format!("{}.{}", self.prefix.join("."), F::field_name())
        };

        Expr::unsafe_raw(bson::Bson::String(format!("${path}")))
    }

    /// Creates a nested expression builder using the same lookup pattern as other builders.
    pub fn with_lookup<F: FieldName, L, G: FieldName, U: HasField<G>>(
        &self,
        lookup: L,
    ) -> ExprBuilder<U>
    where
        T: HasField<F>,
        L: FnOnce(&Path<F, T, T>) -> Path<G, U, T>,
    {
        let base_field: Path<F, T, T> = Path {
            prefix: self.prefix.clone(),
            _marker: std::marker::PhantomData,
        };

        let resolved = lookup(&base_field);

        ExprBuilder {
            prefix: resolved.prefix,
            _marker: std::marker::PhantomData,
        }
    }

    /// Convenience nested builder using identity lookup.
    pub fn with_field<F: FieldName>(&self) -> ExprBuilder<T>
    where
        T: HasField<F>,
    {
        self.with_lookup::<F, _, F, T>(|path| path.clone())
    }

    /// Compares two expressions for equality using MongoDB's `$eq` operator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let condition = b.eq(b.select::<user_fields::Age>(), b.from(18));
    /// assert_eq!(condition.into_bson(), bson::doc! { "$eq": ["$age", 18] }.into());
    /// ```
    pub fn eq<A, B>(&self, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, bool>
    where
        T: MongoComparable<A, B>,
    {
        binary_op("$eq", expr1, expr2)
    }

    /// Compares two expressions for inequality using MongoDB's `$ne` operator.
    pub fn ne<A, B>(&self, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, bool>
    where
        T: MongoComparable<A, B>,
    {
        binary_op("$ne", expr1, expr2)
    }

    /// Returns true if `expr1` is greater than `expr2` using `$gt`.
    ///
    /// Requires [`MongoOrdered`] evidence in addition to [`MongoComparable`],
    /// so only orderable type pairs can be used.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub score: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let condition = b.gt(b.select::<user_fields::Score>(), b.from(10));
    /// assert_eq!(condition.into_bson(), bson::doc! { "$gt": ["$score", 10] }.into());
    /// ```
    pub fn gt<A, B>(&self, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, bool>
    where
        T: MongoComparable<A, B> + MongoOrdered<A, B>,
    {
        binary_op("$gt", expr1, expr2)
    }

    /// Returns true if `expr1` is greater than or equal to `expr2` using `$gte`.
    ///
    /// Requires [`MongoOrdered`] evidence in addition to [`MongoComparable`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let condition = b.gte(b.select::<user_fields::Age>(), b.from(18));
    /// assert_eq!(condition.into_bson(), bson::doc! { "$gte": ["$age", 18] }.into());
    /// ```
    pub fn gte<A, B>(&self, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, bool>
    where
        T: MongoComparable<A, B> + MongoOrdered<A, B>,
    {
        binary_op("$gte", expr1, expr2)
    }

    /// Returns true if `expr1` is less than `expr2` using `$lt`.
    ///
    /// Requires [`MongoOrdered`] evidence in addition to [`MongoComparable`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let condition = b.lt(b.select::<user_fields::Age>(), b.from(65));
    /// assert_eq!(condition.into_bson(), bson::doc! { "$lt": ["$age", 65] }.into());
    /// ```
    pub fn lt<A, B>(&self, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, bool>
    where
        T: MongoComparable<A, B> + MongoOrdered<A, B>,
    {
        binary_op("$lt", expr1, expr2)
    }

    /// Returns true if `expr1` is less than or equal to `expr2` using `$lte`.
    ///
    /// Requires [`MongoOrdered`] evidence in addition to [`MongoComparable`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub score: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let condition = b.lte(b.select::<user_fields::Score>(), b.from(100));
    /// assert_eq!(condition.into_bson(), bson::doc! { "$lte": ["$score", 100] }.into());
    /// ```
    pub fn lte<A, B>(&self, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, bool>
    where
        T: MongoComparable<A, B> + MongoOrdered<A, B>,
    {
        binary_op("$lte", expr1, expr2)
    }

    /// Combines boolean expressions with MongoDB's `$and`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    ///     pub score: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.and(
    ///     b.gt(b.select::<user_fields::Age>(), b.from(18)),
    ///     vec![b.gte(b.select::<user_fields::Score>(), b.from(100))],
    /// );
    ///
    /// assert_eq!(
    ///     expr.into_bson(),
    ///     bson::doc! {
    ///         "$and": [
    ///             { "$gt": ["$age", 18] },
    ///             { "$gte": ["$score", 100] }
    ///         ]
    ///     }
    ///     .into()
    /// );
    /// ```
    pub fn and(&self, head: Expr<T, bool>, tail: Vec<Expr<T, bool>>) -> Expr<T, bool> {
        nary_op("$and", head, tail)
    }

    /// Combines boolean expressions with MongoDB's `$or`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    ///     pub score: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.or(
    ///     b.lt(b.select::<user_fields::Age>(), b.from(18)),
    ///     vec![b.gte(b.select::<user_fields::Score>(), b.from(90))],
    /// );
    ///
    /// assert_eq!(
    ///     expr.into_bson(),
    ///     bson::doc! {
    ///         "$or": [
    ///             { "$lt": ["$age", 18] },
    ///             { "$gte": ["$score", 90] }
    ///         ]
    ///     }
    ///     .into()
    /// );
    /// ```
    pub fn or(&self, head: Expr<T, bool>, tail: Vec<Expr<T, bool>>) -> Expr<T, bool> {
        nary_op("$or", head, tail)
    }

    /// Negates a boolean expression with MongoDB's `$not`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.not(b.eq(b.select::<user_fields::Age>(), b.from(18)));
    ///
    /// assert_eq!(
    ///     expr.into_bson(),
    ///     bson::doc! { "$not": { "$eq": ["$age", 18] } }.into()
    /// );
    /// ```
    pub fn not(&self, expr: Expr<T, bool>) -> Expr<T, bool> {
        unary_op("$not", expr)
    }

    /// Sums numeric expressions with MongoDB's `$add`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub age: i32,
    ///     pub score: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.add(b.select::<user_fields::Age>(), vec![b.select::<user_fields::Score>()]);
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$add": ["$age", "$score"] }.into());
    /// ```
    pub fn add<N>(&self, head: Expr<T, N>, tail: Vec<Expr<T, N>>) -> Expr<T, N>
    where
        N: Num,
    {
        nary_op("$add", head, tail)
    }

    /// Subtracts two numeric expressions with MongoDB's `$subtract`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub score: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.subtract(b.select::<user_fields::Score>(), b.from(5));
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$subtract": ["$score", 5] }.into());
    /// ```
    pub fn subtract<N>(&self, minuend: Expr<T, N>, subtrahend: Expr<T, N>) -> Expr<T, N>
    where
        N: Num,
    {
        binary_op("$subtract", minuend, subtrahend)
    }

    /// Multiplies numeric expressions with MongoDB's `$multiply`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct Item {
    ///     pub price: i32,
    ///     pub quantity: i32,
    /// }
    ///
    /// let b = expr::empty::<Item>();
    /// let expr = b.multiply(
    ///     b.select::<item_fields::Price>(),
    ///     vec![b.select::<item_fields::Quantity>()],
    /// );
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$multiply": ["$price", "$quantity"] }.into());
    /// ```
    pub fn multiply<N>(&self, head: Expr<T, N>, tail: Vec<Expr<T, N>>) -> Expr<T, N>
    where
        N: Num,
    {
        nary_op("$multiply", head, tail)
    }

    /// Divides two numeric expressions with MongoDB's `$divide`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct Item {
    ///     pub total: i32,
    ///     pub count: i32,
    /// }
    ///
    /// let b = expr::empty::<Item>();
    /// let expr = b.divide(b.select::<item_fields::Total>(), b.select::<item_fields::Count>());
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$divide": ["$total", "$count"] }.into());
    /// ```
    pub fn divide<N>(&self, dividend: Expr<T, N>, divisor: Expr<T, N>) -> Expr<T, N>
    where
        N: Num,
    {
        binary_op("$divide", dividend, divisor)
    }

    /// Computes the modulus of two numeric expressions with MongoDB's `$mod`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub age: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.modulo(b.select::<user_fields::Age>(), b.from(2));
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$mod": ["$age", 2] }.into());
    /// ```
    pub fn modulo<N>(&self, dividend: Expr<T, N>, divisor: Expr<T, N>) -> Expr<T, N>
    where
        N: Num,
    {
        binary_op("$mod", dividend, divisor)
    }

    /// Builds a conditional expression with MongoDB's `$cond`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::{FieldWitnesses, MongoComparable};
    ///
    /// #[derive(FieldWitnesses, MongoComparable)]
    /// struct User {
    ///     pub age: i32,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.cond(
    ///     b.gte(b.select::<user_fields::Age>(), b.from(18)),
    ///     b.from("adult".to_string()),
    ///     b.from("minor".to_string()),
    /// );
    ///
    /// assert_eq!(
    ///     expr.into_bson(),
    ///     bson::doc! {
    ///         "$cond": {
    ///             "if": { "$gte": ["$age", 18] },
    ///             "then": "adult",
    ///             "else": "minor"
    ///         }
    ///     }
    ///     .into()
    /// );
    /// ```
    pub fn cond<V>(
        &self,
        condition: Expr<T, bool>,
        if_true: Expr<T, V>,
        if_false: Expr<T, V>,
    ) -> Expr<T, V> {
        let doc = bson::doc! {
            "$cond": {
                "if": condition.into_bson(),
                "then": if_true.into_bson(),
                "else": if_false.into_bson(),
            }
        };

        Expr::unsafe_raw(doc.into())
    }

    /// Returns `replacement` when `expr` is null using MongoDB's `$ifNull`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub nickname: String,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.if_null(
    ///     b.select::<user_fields::Nickname>(),
    ///     b.from("anonymous".to_string()),
    /// );
    ///
    /// assert_eq!(
    ///     expr.into_bson(),
    ///     bson::doc! { "$ifNull": ["$nickname", "anonymous"] }.into()
    /// );
    /// ```
    pub fn if_null<V>(&self, expr: Expr<T, V>, replacement: Expr<T, V>) -> Expr<T, V> {
        binary_op("$ifNull", expr, replacement)
    }

    /// Concatenates string expressions with MongoDB's `$concat`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub first_name: String,
    ///     pub last_name: String,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.concat(
    ///     b.select::<user_fields::FirstName>(),
    ///     vec![b.from(" ".to_string()), b.select::<user_fields::LastName>()],
    /// );
    ///
    /// assert_eq!(
    ///     expr.into_bson(),
    ///     bson::doc! { "$concat": ["$first_name", " ", "$last_name"] }.into()
    /// );
    /// ```
    pub fn concat(&self, head: Expr<T, String>, tail: Vec<Expr<T, String>>) -> Expr<T, String> {
        nary_op("$concat", head, tail)
    }

    /// Uppercases a string expression using MongoDB's `$toUpper`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.to_upper(b.select::<user_fields::Name>());
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$toUpper": "$name" }.into());
    /// ```
    pub fn to_upper(&self, expr: Expr<T, String>) -> Expr<T, String> {
        unary_op("$toUpper", expr)
    }

    /// Lowercases a string expression using MongoDB's `$toLower`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tnuctipun::expr;
    /// use tnuctipun::FieldWitnesses;
    ///
    /// #[derive(FieldWitnesses)]
    /// struct User {
    ///     pub name: String,
    /// }
    ///
    /// let b = expr::empty::<User>();
    /// let expr = b.to_lower(b.select::<user_fields::Name>());
    ///
    /// assert_eq!(expr.into_bson(), bson::doc! { "$toLower": "$name" }.into());
    /// ```
    pub fn to_lower(&self, expr: Expr<T, String>) -> Expr<T, String> {
        unary_op("$toLower", expr)
    }
}

/// Creates an empty expression builder.
pub fn empty<T>() -> ExprBuilder<T> {
    ExprBuilder::new()
}

fn unary_op<T, A, R>(operator: &str, expr: Expr<T, A>) -> Expr<T, R> {
    let doc = bson::doc! { operator: expr.into_bson() };

    Expr::unsafe_raw(doc.into())
}

fn binary_op<T, A, B, R>(operator: &str, expr1: Expr<T, A>, expr2: Expr<T, B>) -> Expr<T, R> {
    let doc = bson::doc! { operator: [expr1.into_bson(), expr2.into_bson()] };

    Expr::unsafe_raw(doc.into())
}

fn nary_op<T, A, R>(operator: &str, head: Expr<T, A>, tail: Vec<Expr<T, A>>) -> Expr<T, R> {
    let mut all = Vec::with_capacity(1 + tail.len());
    all.push(head.into_bson());
    all.extend(tail.into_iter().map(Expr::into_bson));

    let doc = bson::doc! { operator: all };

    Expr::unsafe_raw(doc.into())
}
