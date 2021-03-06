use crate::{
    data::{hash_password, SeedUser},
    tests::run_test,
};
use insta::assert_json_snapshot;
use rocket::http::Status;
use serde_json::json;
use uuid::Uuid;

#[test]
fn test_login_unknown_user() {
    run_test()
        .post(
            "/login",
            json!({
                "username": "username",
                "password": "password"
            }),
        )
        .has_status(Status::Unauthorized)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/authentication_error",
            "title": "Invalid Username or Password",
            "status": 401
        }));
}

#[test]
fn test_login_empty_body() {
    run_test()
        .post("/login", json!({}))
        .has_status(Status::Unauthorized)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/authentication_error",
            "title": "Invalid Username or Password",
            "status": 401
        }));
}

#[test]
fn test_login_invalid_password() {
    run_test()
        .seed(&SeedUser {
            username: "testuser".to_owned(),
            password: hash_password("password"),
            ..SeedUser::default()
        })
        .post(
            "/login",
            json!({
                "username": "testuser",
                "password": "incorrect"
            }),
        )
        .has_status(Status::Unauthorized)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/authentication_error",
            "title": "Invalid Username or Password",
            "status": 401
        }));
}

#[test]
fn test_login_missing_password() {
    run_test()
        .seed(&SeedUser {
            username: "testuser".to_owned(),
            password: hash_password("password"),
            ..SeedUser::default()
        })
        .post(
            "/login",
            json!({
                "username": "testuser"
            }),
        )
        .has_status(Status::Unauthorized)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/authentication_error",
            "title": "Invalid Username or Password",
            "status": 401
        }));
}

#[test]
fn test_login_success() {
    run_test()
        .seed(&SeedUser {
            user_id: Uuid::parse_str("c23462c2-7096-4677-9663-231cd9bca08a").unwrap(),
            username: "testuser".to_owned(),
            password: hash_password("password"),
            display_name: "Test User".to_owned(),
            email_address: "testuser@example.com".to_owned(),
            ..SeedUser::default()
        })
        .post(
            "/login",
            json!({
                "username": "testuser",
                "password": "password"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header(
            "Link",
            "</users/c23462c2-7096-4677-9663-231cd9bca08a>; rel=\"related\"",
        )
        .assert_json_body(|body| {
            assert_json_snapshot!(body, {
                ".token" => "[access_token]",
                ".valid_until" => "[access_token_expiry]"
            }, @r###"
            {
              "display_name": "Test User",
              "token": "[access_token]",
              "user_id": "c23462c2-7096-4677-9663-231cd9bca08a",
              "valid_until": "[access_token_expiry]"
            }
            "###);
        });
}
