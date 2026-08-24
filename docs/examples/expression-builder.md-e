---
title: Expression Builder
layout: page
nav_exclude: true
---

This guide introduces `expr::ExprBuilder`, then shows how to reuse typed expressions in filters (`$expr`), projections, and updates.

## Building Typed Expressions

`ExprBuilder` creates BSON expression trees with compile-time constraints.

```rust
use tnuctipun::{expr, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub price: f64,
    pub stock: i32,
    pub reserved: i32,
    pub min_stock: i32,
}

fn expression_basics() {
    let b = expr::empty::<Product>();

    let stock = b.select::<product_fields::Stock>();
    let reserved = b.select::<product_fields::Reserved>();

    // available = stock - reserved
    let available = b.subtract(stock, reserved);

    // low_stock = available < min_stock
    let low_stock = b.lt(available, b.select::<product_fields::MinStock>());

    assert_eq!(
        low_stock.into_bson(),
        bson::doc! {
            "$lt": [
                { "$subtract": ["$stock", "$reserved"] },
                "$min_stock"
            ]
        }
        .into()
    );
}
```

See also: [Finding Documents - Expression Builder (`$expr`)](../user-guide/03-finding-documents.md#expression-builder-expr)

## Using Expressions in Filters (`$expr`)

`FilterBuilder::expr` inserts a typed boolean expression under MongoDB's `$expr` operator.

```rust
use tnuctipun::{expr, filters, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    pub name: String,
    pub stock: i32,
    pub reserved: i32,
    pub min_stock: i32,
}

fn filter_with_expr() {
    let b = expr::empty::<Product>();

    let low_stock = b.lt(
        b.subtract(
            b.select::<product_fields::Stock>(),
            b.select::<product_fields::Reserved>(),
        ),
        b.select::<product_fields::MinStock>(),
    );

    let filter_doc = filters::empty::<Product>()
        .eq::<product_fields::Name, _>("Widget".to_string())
        .expr(low_stock)
        .and();

    // Produces a query containing "$expr": { ... }
    println!("{}", filter_doc);
}
```

See also: [Finding Documents](../user-guide/03-finding-documents.md)

## Using Expressions in Projections

`projection::ProjectionBuilder::project_expr` lets you compute projected field values using typed expressions.

```rust
use tnuctipun::{expr, projection, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct InventoryView {
    pub stock: i32,
    pub reserved: i32,
    pub available: i32,
}

fn projection_with_expr() {
    let b = expr::empty::<InventoryView>();

    let mut proj = projection::empty::<InventoryView>();
    proj.project_expr::<inventoryview_fields::Available, _>(
        b.subtract(
            b.select::<inventoryview_fields::Stock>(),
            b.select::<inventoryview_fields::Reserved>(),
        ),
    );

    let projection_doc = proj.build();
    println!("{}", projection_doc);
}
```

See also: [Finding Documents - Projections](../user-guide/03-finding-documents.md#projections)

## Using Expressions in Updates

`updates::UpdateBuilder::set_expr` applies an expression as the new value of a field.

```rust
use tnuctipun::{expr, updates, FieldWitnesses, MongoComparable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct ScoreCard {
    pub score: i32,
    pub bonus: i32,
}

fn update_with_expr() {
    let b = expr::empty::<ScoreCard>();

    let update_doc = updates::empty::<ScoreCard>()
        .set_expr::<scorecard_fields::Score, _>(
            b.add(
                b.select::<scorecard_fields::Score>(),
                vec![b.select::<scorecard_fields::Bonus>()],
            ),
        )
        .build();

    println!("{}", update_doc);
}
```

See also: [Updating Documents - Expression-Based Updates](../user-guide/04-updating-documents.md#expression-based-updates)

## Notes on Type Constraints

- Equality comparisons (`eq`, `ne`) use `MongoComparable` bounds.
- Ordering comparisons (`gt`, `gte`, `lt`, `lte`) require `MongoOrdered` evidence.
- If an expression uses an unsupported ordering type pair, compilation fails early.

See also: [Derive Macros - MongoComparable Macro](../user-guide/05-derive-macros.md#mongocomparable-macro)
