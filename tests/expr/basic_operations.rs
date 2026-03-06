use bson::doc;
use tnuctipun::expr;

use super::test_fixtures::*;

#[test]
fn select_and_literal_builds_expected_bson() {
    let b = expr::empty::<ExprUser>();
    let age = b.select::<expruser_fields::Age>();
    let lit = b.from(21);

    assert_eq!(age.as_bson(), &bson::Bson::String("$age".to_string()));
    assert_eq!(age.into_bson(), "$age".into());
    assert_eq!(lit.into_bson(), 21.into());
}

#[test]
fn arithmetic_operator_builds_expected_document() {
    let b = expr::empty::<ExprUser>();
    let expr = b.add(
        b.select::<expruser_fields::Age>(),
        vec![b.select::<expruser_fields::Score>()],
    );

    assert_eq!(expr.into_bson(), doc! { "$add": ["$age", "$score"] }.into());
}

#[test]
fn conditional_operator_builds_expected_document() {
    let b = expr::empty::<ExprUser>();
    let is_adult = b.gte(b.select::<expruser_fields::Age>(), b.from(18));
    let label = b.cond(
        is_adult,
        b.from("adult".to_string()),
        b.from("minor".to_string()),
    );

    assert_eq!(
        label.into_bson(),
        doc! {
            "$cond": {
                "if": { "$gte": ["$age", 18] },
                "then": "adult",
                "else": "minor"
            }
        }
        .into()
    );
}
