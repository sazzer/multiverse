use crate::{data::SeedUser, tests::run_test};
use galvanic_assert::{assert_that, matchers::*};
use insta::assert_json_snapshot;
use rocket::http::Status;
use serde_json::json;

#[test]
fn test_register_empty_body() {
    run_test()
        .post("/register", json!({}))
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/validation_error",
            "title": "A validation error occurred",
            "status": 422,
            "fields": {
                "username": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                },
                "password": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                },
                "email_address": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                }
            }
        }));
}

#[test]
fn test_register_blank_body() {
    run_test()
        .post(
            "/register",
            json!({
                "username": "",
                "password": "",
                "email_address": ""
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/validation_error",
            "title": "A validation error occurred",
            "status": 422,
            "fields": {
                "username": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                },
                "password": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                },
                "email_address": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                }
            }
        }));
}

#[test]
fn test_register_whitespace_body() {
    run_test()
        .post(
            "/register",
            json!({
                "username": "  ",
                "password": "  ",
                "email_address": "  "
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/validation_error",
            "title": "A validation error occurred",
            "status": 422,
            "fields": {
                "username": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                },
                "password": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                },
                "email_address": {
                    "type": "tag:multiverse,2020:problems/validation_error/missing",
                    "title": "The required field was missing"
                }
            }
        }));
}

#[test]
fn test_register_success_minimal_data() {
    run_test()
        .post(
            "/register",
            json!({
                "username": "testuser",
                "password": "password",
                "email_address": "testuser@example.com"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .assert_json_body(|body| {
            assert_json_snapshot!(body, {
                ".token.token" => "[access_token]",
                ".token.valid_until" => "[access_token_expiry]"
            }, @r###"
            {
              "token": {
                "token": "[access_token]",
                "valid_until": "[access_token_expiry]"
              },
              "user": {
                "display_name": "testuser",
                "email_address": "testuser@example.com",
                "username": "testuser"
              }
            }
            "###);
        })
        .assert_database(|mut conn| {
            let user = conn
                .query_one("SELECT * FROM users WHERE username = $1", &[&"testuser"])
                .unwrap();

            assert_that!(&user.get("username"), eq("testuser"));
            assert_that!(&user.get("display_name"), eq("testuser"));
            assert_that!(&user.get("email_address"), eq("testuser@example.com"));
            assert_that!(&user.get("avatar_url"), eq(Option::<&str>::None));
        });
}

#[test]
fn test_register_success_full_data() {
    run_test()
        .post(
            "/register",
            json!({
                "username": "testuser",
                "password": "password",
                "display_name": "Test User",
                "avatar_url": "http://example.com/testuser.png",
                "email_address": "testuser@example.com"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .assert_json_body(|body| {
            assert_json_snapshot!(body, {
                ".token.token" => "[access_token]",
                ".token.valid_until" => "[access_token_expiry]"
            }, @r###"
            {
              "token": {
                "token": "[access_token]",
                "valid_until": "[access_token_expiry]"
              },
              "user": {
                "avatar_url": "http://example.com/testuser.png",
                "display_name": "Test User",
                "email_address": "testuser@example.com",
                "username": "testuser"
              }
            }
            "###);
        })
        .assert_database(|mut conn| {
            let user = conn
                .query_one("SELECT * FROM users WHERE username = $1", &[&"testuser"])
                .unwrap();

            assert_that!(&user.get("username"), eq("testuser"));
            assert_that!(&user.get("display_name"), eq("Test User"));
            assert_that!(&user.get("email_address"), eq("testuser@example.com"));
            assert_that!(
                &user.get("avatar_url"),
                eq(Some("http://example.com/testuser.png"))
            );
        });
}

#[test]
fn test_register_duplicate_username() {
    run_test()
        .seed(SeedUser {
            username: "testuser".to_owned(),
            display_name: "Old Display Name".to_owned(),
            email_address: "old@example.com".to_owned(),
            ..SeedUser::default()
        })
        .post(
            "/register",
            json!({
                "username": "testuser",
                "password": "password",
                "email_address": "testuser@example.com"
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/duplicate_username",
            "title": "The username is already registered",
            "status": 422
        }))
        .assert_database(|mut conn| {
            let user = conn
                .query_one("SELECT * FROM users WHERE username = $1", &[&"testuser"])
                .unwrap();

            assert_that!(&user.get("display_name"), eq("Old Display Name"));
            assert_that!(&user.get("email_address"), eq("old@example.com"));
        });
}
