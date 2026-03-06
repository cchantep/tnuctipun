use bson::doc;
use tnuctipun::expr;
use tnuctipun::filters;
use tnuctipun::projection;
use tnuctipun::updates;

use super::test_fixtures::*;

#[test]
fn filter_expr_clause_is_embedded_under_dollar_expr() {
    let b = expr::empty::<ExprUser>();
    let condition = b.gt(b.select::<expruser_fields::Age>(), b.from(18));

    let filter = filters::empty::<ExprUser>().expr(condition).and();

    assert_eq!(filter, doc! { "$expr": { "$gt": ["$age", 18] } });
}

#[test]
fn projection_project_expr_sets_typed_path() {
    let mut proj = projection::empty::<ExprUser>();
    let b = expr::empty::<ExprUser>();

    proj.project_expr::<expruser_fields::Name, _>(b.to_upper(b.select::<expruser_fields::Name>()));

    assert_eq!(proj.build(), doc! { "name": { "$toUpper": "$name" } });
}

#[test]
fn update_set_expr_uses_set_bucket() {
    let mut upd = updates::empty::<ExprUser>();
    let b = expr::empty::<ExprUser>();

    upd.set_expr::<expruser_fields::Score, _>(
        b.add(b.select::<expruser_fields::Score>(), vec![b.from(1)]),
    );

    assert_eq!(
        upd.build(),
        doc! { "$set": { "score": { "$add": ["$score", 1] } } }
    );
}
