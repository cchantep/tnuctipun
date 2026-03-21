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

#[test]
fn comparison_and_boolean_operators_build_expected_documents() {
    let b = expr::empty::<ExprUser>();

    let eq = b.eq(b.select::<expruser_fields::Age>(), b.from(18));
    let ne = b.ne(
        b.select::<expruser_fields::Name>(),
        b.from("john".to_string()),
    );
    let lt = b.lt(b.select::<expruser_fields::Score>(), b.from(100));
    let lte = b.lte(b.select::<expruser_fields::Score>(), b.from(200));
    let and = b.and(
        b.gt(b.select::<expruser_fields::Age>(), b.from(18)),
        vec![b.gte(b.select::<expruser_fields::Score>(), b.from(100))],
    );
    let or = b.or(
        b.lt(b.select::<expruser_fields::Age>(), b.from(18)),
        vec![b.gte(b.select::<expruser_fields::Score>(), b.from(100))],
    );
    let not = b.not(b.eq(
        b.select::<expruser_fields::Name>(),
        b.from("anonymous".to_string()),
    ));

    assert_eq!(eq.into_bson(), doc! { "$eq": ["$age", 18] }.into());
    assert_eq!(ne.into_bson(), doc! { "$ne": ["$name", "john"] }.into());
    assert_eq!(lt.into_bson(), doc! { "$lt": ["$score", 100] }.into());
    assert_eq!(lte.into_bson(), doc! { "$lte": ["$score", 200] }.into());
    assert_eq!(
        and.into_bson(),
        doc! { "$and": [{ "$gt": ["$age", 18] }, { "$gte": ["$score", 100] }] }.into()
    );
    assert_eq!(
        or.into_bson(),
        doc! { "$or": [{ "$lt": ["$age", 18] }, { "$gte": ["$score", 100] }] }.into()
    );
    assert_eq!(
        not.into_bson(),
        doc! { "$not": { "$eq": ["$name", "anonymous"] } }.into()
    );
}

#[test]
fn arithmetic_and_string_operators_build_expected_documents() {
    let b = expr::empty::<ExprUser>();

    let subtract = b.subtract(b.select::<expruser_fields::Score>(), b.from(5));
    let multiply = b.multiply(
        b.select::<expruser_fields::Age>(),
        vec![b.select::<expruser_fields::Score>()],
    );
    let divide = b.divide(b.select::<expruser_fields::Score>(), b.from(2));
    let modulo = b.modulo(b.select::<expruser_fields::Score>(), b.from(2));
    let if_null = b.if_null(
        b.select::<expruser_fields::Name>(),
        b.from("unknown".to_string()),
    );
    let concat = b.concat(
        b.select::<expruser_fields::Name>(),
        vec![b.from("!".to_string())],
    );
    let upper = b.to_upper(b.select::<expruser_fields::Name>());
    let lower = b.to_lower(b.select::<expruser_fields::Name>());

    assert_eq!(
        subtract.into_bson(),
        doc! { "$subtract": ["$score", 5] }.into()
    );
    assert_eq!(
        multiply.into_bson(),
        doc! { "$multiply": ["$age", "$score"] }.into()
    );
    assert_eq!(divide.into_bson(), doc! { "$divide": ["$score", 2] }.into());
    assert_eq!(modulo.into_bson(), doc! { "$mod": ["$score", 2] }.into());
    assert_eq!(
        if_null.into_bson(),
        doc! { "$ifNull": ["$name", "unknown"] }.into()
    );
    assert_eq!(
        concat.into_bson(),
        doc! { "$concat": ["$name", "!"] }.into()
    );
    assert_eq!(upper.into_bson(), doc! { "$toUpper": "$name" }.into());
    assert_eq!(lower.into_bson(), doc! { "$toLower": "$name" }.into());
}

#[test]
fn raw_widen_and_nested_builder_operators_build_expected_documents() {
    let b = expr::empty::<ExprUser>();
    let raw = b.unsafe_expr::<i32>(doc! { "$literal": 7 }.into());
    let widened = b.from(7_i32).widen::<i64>();

    assert_eq!(raw.into_bson(), doc! { "$literal": 7 }.into());
    assert_eq!(widened.into_bson(), 7.into());

    let nested = expr::empty::<ExprUserWithAddress>()
        .with_lookup::<
            expruserwithaddress_fields::Address,
            _,
            expraddress_fields::City,
            ExprAddress,
        >(|path| path.field::<expraddress_fields::City>())
        .select::<expraddress_fields::City>();

    assert_eq!(nested.into_bson(), "$address.city".into());

    let with_field_identity = expr::empty::<ExprUserWithAddress>()
        .with_field::<expruserwithaddress_fields::Address>()
        .select::<expruserwithaddress_fields::Name>();

    assert_eq!(with_field_identity.into_bson(), "$name".into());
}
