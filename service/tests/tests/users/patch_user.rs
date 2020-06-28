use crate::{
    data::{hash_password, SeedUser},
    tests::run_test,
};
use galvanic_assert::{assert_that, matchers::*};
use rocket::http::Status;
use serde_json::json;
use uritemplate::UriTemplate;
use uuid::Uuid;

#[test]
fn test_patch_unauthenticated() {
    let url = UriTemplate::new("/users/{id}")
        .set("id", Uuid::new_v4().to_string())
        .build();

    run_test()
        .patch(url, json!({}))
        .has_status(Status::Forbidden)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/unauthorized",
            "title": "An invalid access token was provided",
            "status": 403
        }));
}

#[test]
fn test_patch_wrong_user() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", Uuid::new_v4().to_string())
        .build();

    run_test()
        .seed(&user)
        .authenticate("testuser", "password")
        .patch(url, json!({}))
        .has_status(Status::Forbidden)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/unauthorized",
            "title": "An invalid access token was provided",
            "status": 403
        }));
}

#[test]
fn test_patch_user_no_changes() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(&url, json!({}))
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", format!("<{}>; rel=\"self\"", url))
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "Test User",
            "email_address": "testuser@example.com"
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("Test User"));
            assert_that!(&user_row.get("email_address"), eq("testuser@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), eq(user.password));
            assert_that!(&user_row.get("avatar_url"), eq(Option::<&str>::None));

            assert_that!(&user_row.get("version"), not(eq(user.version)));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), not(eq(user.updated)));
        });
}

#[test]
fn test_patch_user_changes_all() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(
            &url,
            json!({
                "password": "new",
                "old_password": "password",
                "email_address": "new@example.com",
                "display_name": "New Name",
                "avatar_url": "http://example.com/avatar"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", format!("<{}>; rel=\"self\"", url))
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "New Name",
            "email_address": "new@example.com",
            "avatar_url": "http://example.com/avatar"
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("New Name"));
            assert_that!(&user_row.get("email_address"), eq("new@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), not(eq(user.password)));
            assert_that!(&user_row.get("avatar_url"), eq("http://example.com/avatar"));

            assert_that!(&user_row.get("version"), not(eq(user.version)));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), not(eq(user.updated)));
        });
}

#[test]
fn test_patch_user_changes_no_password() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(
            url,
            json!({
                "email_address": "new@example.com",
                "display_name": "New Name",
                "avatar_url": "http://example.com/avatar"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "New Name",
            "email_address": "new@example.com",
            "avatar_url": "http://example.com/avatar"
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("New Name"));
            assert_that!(&user_row.get("email_address"), eq("new@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), eq(user.password));
            assert_that!(&user_row.get("avatar_url"), eq("http://example.com/avatar"));

            assert_that!(&user_row.get("version"), not(eq(user.version)));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), not(eq(user.updated)));
        });
}

#[test]
fn test_patch_user_change_password() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(
            url,
            json!({
                "password": "new",
                "old_password": "password",
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "Test User",
            "email_address": "testuser@example.com",
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("Test User"));
            assert_that!(&user_row.get("email_address"), eq("testuser@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), not(eq(user.password)));
            assert_that!(&user_row.get("avatar_url"), eq(Option::<&str>::None));

            assert_that!(&user_row.get("version"), not(eq(user.version)));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), not(eq(user.updated)));
        });
}

#[test]
fn test_patch_user_change_password_no_old() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(
            url,
            json!({
                "password": "new",
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "status": 422,
            "type": "tag:multiverse,2020:users/problems/invalid_old_password",
            "title": "Old Password incorrect when changing password",
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("Test User"));
            assert_that!(&user_row.get("email_address"), eq("testuser@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), eq(user.password));
            assert_that!(&user_row.get("avatar_url"), eq(Option::<&str>::None));

            assert_that!(&user_row.get("version"), eq(user.version));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), eq(user.updated));
        });
}

#[test]
fn test_patch_user_change_password_wrong_old() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(
            url,
            json!({
                "password": "new",
                "old_password": "incorrect"
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "status": 422,
            "type": "tag:multiverse,2020:users/problems/invalid_old_password",
            "title": "Old Password incorrect when changing password",
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("Test User"));
            assert_that!(&user_row.get("email_address"), eq("testuser@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), eq(user.password));
            assert_that!(&user_row.get("avatar_url"), eq(Option::<&str>::None));

            assert_that!(&user_row.get("version"), eq(user.version));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), eq(user.updated));
        });
}

#[test]
fn test_patch_user_invalid() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(
            url,
            json!({
                "email_address": "",
                "password": ""
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "status": 422,
            "type": "tag:multiverse,2020:problems/validation_error",
            "title": "A validation error occurred",
            "fields": {
                "email_address": {
                    "title": "The required field was missing",
                    "type": "tag:multiverse,2020:problems/validation_error/missing"
                },
                "password":{
                    "title": "The required field was missing",
                    "type": "tag:multiverse,2020:problems/validation_error/missing"
                }
            }
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("display_name"), eq("Test User"));
            assert_that!(&user_row.get("email_address"), eq("testuser@example.com"));
            assert_that!(&user_row.get("username"), eq("testuser"));
            assert_that!(&user_row.get("password"), eq(user.password));
            assert_that!(&user_row.get("avatar_url"), eq(Option::<&str>::None));

            assert_that!(&user_row.get("version"), eq(user.version));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), eq(user.updated));
        });
}

#[test]
fn test_patch_clear_avatar_url() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        email_address: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        avatar_url: Some("http://example.com".to_owned()),
        ..SeedUser::default()
    };

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(&user.clone())
        .authenticate("testuser", "password")
        .patch(url, json!({ "avatar_url": null }))
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "Test User",
            "email_address": "testuser@example.com"
        }))
        .assert_database(|mut conn| {
            let user_row = conn
                .query_one("SELECT * FROM users WHERE user_id = $1", &[&user.user_id])
                .unwrap();

            assert_that!(&user_row.get("avatar_url"), eq(Option::<&str>::None));

            assert_that!(&user_row.get("version"), not(eq(user.version)));
            assert_that!(&user_row.get("created"), eq(user.created));
            assert_that!(&user_row.get("updated"), not(eq(user.updated)));
        });
}
