use crate::{
    data::{hash_password, SeedUser},
    tests::run_test,
};
use rocket::http::Status;
use serde_json::json;

#[test]
fn test_create_unauthenticated() {
    run_test()
        .post("/worlds", json!({}))
        .has_status(Status::Forbidden)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/unauthorized",
            "title": "An invalid access token was provided",
            "status": 403
        }));
}

#[test]
fn test_create_empty_body() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        ..SeedUser::default()
    };

    run_test()
        .seed(user)
        .authenticate("testuser", "password")
        .post("/worlds", json!({}))
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:problems/validation_error",
            "title": "A validation error occurred",
            "status": 422,
            "fields": {
              "name": {
                "type": "tag:multiverse,2020:problems/validation_error/missing",
                "title": "The required field was missing"
              },
              "url_slug": {
                "type": "tag:multiverse,2020:problems/validation_error/missing",
                "title": "The required field was missing"
              }
            }
        }));
}
