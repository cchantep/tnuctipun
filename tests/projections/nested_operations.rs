//! Tests for basic nested field projections (simplified to avoid complex API issues)

use super::test_fixtures::*;
use tnuctipun::projection::empty;

#[test]
fn projection_basic_include() {
    // Test basic include functionality
    let doc = empty::<User>().includes::<user_fields::Name>().build();

    let expected = bson::doc! {
        "name": 1
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_basic_exclude() {
    // Test basic exclude functionality
    let doc = empty::<User>().excludes::<user_fields::Id>().build();

    let expected = bson::doc! {
        "id": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_multiple_fields() {
    // Test multiple field operations
    let doc = empty::<User>()
        .includes::<user_fields::Name>()
        .includes::<user_fields::Email>()
        .excludes::<user_fields::Id>()
        .build();

    let expected = bson::doc! {
        "name": 1,
        "email": 1,
        "id": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_with_field_context_chains_operations() {
    let doc = empty::<User>()
        .with_field::<user_fields::Name, _>(|nested| {
            nested
                .includes::<user_fields::Name>()
                .excludes::<user_fields::Email>();
        })
        .build();

    let expected = bson::doc! {
        "name": 1,
        "email": 0
    };

    assert_eq!(doc, expected);
}

#[test]
fn projection_with_lookup_projects_nested_fields() {
    let doc = empty::<UserWithProfile>()
        .with_lookup::<userwithprofile_fields::Profile, _, profile_fields::AvatarUrl, Profile, _>(
            |path| path.field::<profile_fields::AvatarUrl>(),
            |nested| {
                nested
                    .includes::<profile_fields::AvatarUrl>()
                    .excludes::<profile_fields::Bio>();
            },
        )
        .build();

    let expected = bson::doc! {
        "profile.avatar_url": 1,
        "profile.bio": 0
    };

    assert_eq!(doc, expected);
}
