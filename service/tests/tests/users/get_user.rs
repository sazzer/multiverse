use crate::{
    data::{hash_password, SeedUser},
    tests::run_test,
};
use rocket::http::Status;
use serde_json::json;
use uritemplate::UriTemplate;
use uuid::Uuid;

#[test]
fn test_lookup_unknown_user() {
    let url = UriTemplate::new("/users/{id}")
        .set("id", Uuid::new_v4().to_string())
        .build();

    run_test()
        .get(url)
        .has_status(Status::NotFound)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/unknown_user_id",
            "title": "The requested user ID was unknown",
            "status": 404
        }));
}

#[test]
fn test_lookup_known_user_unauthenticated() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };
    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();

    run_test()
        .seed(user)
        .get(url)
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "Test User"
        }));
}

#[test]
fn test_lookup_known_user_authenticated() {
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
        .seed(user)
        .authenticate("testuser", "password")
        .get(url)
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_json_body(json!({
            "username": "testuser",
            "display_name": "Test User",
            "email_address": "testuser@example.com"
        }));
}
