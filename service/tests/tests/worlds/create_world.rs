use crate::{
    data::{hash_password, SeedUser, SeedWorld},
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
        .seed(&user)
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

#[test]
fn test_create_success() {
    let user = SeedUser {
        user_id: uuid::Uuid::parse_str("7da4cb77-8839-4805-b93a-f4c536c8bc85").unwrap(),
        display_name: "Test User".to_owned(),
        username: "testuser".to_owned(),
        password: hash_password("password"),
        ..SeedUser::default()
    };

    run_test()
        .seed(&user)
        .authenticate("testuser", "password")
        .post(
            "/worlds",
            json!({
              "name": "Test World",
              "description": "This is a test world",
              "url_slug": "test-world"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header_regex(
            "Link",
            r#"</users/7da4cb77-8839-4805-b93a-f4c536c8bc85>; rel="author"; title="Test User""#,
        )
        .has_header_regex(
            "Link",
            r#"</worlds/[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}>; rel="self""#,
        )
        .has_json_body(json!({
          "name": "Test World",
          "description": "This is a test world",
          "url_slug": "test-world"
        }));
}

#[test]
fn test_create_duplicate_url_slug() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        password: hash_password("password"),
        ..SeedUser::default()
    };
    let world = SeedWorld {
        owner: user.user_id,
        url_slug: "test-world".to_owned(),
        ..SeedWorld::default()
    };

    run_test()
        .seed(&user)
        .seed(&world)
        .authenticate("testuser", "password")
        .post(
            "/worlds",
            json!({
              "name": "Test World",
              "description": "This is a test world",
              "url_slug": "test-world"
            }),
        )
        .has_status(Status::UnprocessableEntity)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:worlds/problems/duplicate_url_slug",
            "title": "The URL Slug was already present for this user",
            "status": 422
        }));
}

#[test]
fn test_create_minimal() {
    let user = SeedUser {
        user_id: uuid::Uuid::parse_str("7da4cb77-8839-4805-b93a-f4c536c8bc85").unwrap(),
        display_name: "Test User".to_owned(),
        username: "testuser".to_owned(),
        password: hash_password("password"),
        ..SeedUser::default()
    };

    run_test()
        .seed(&user)
        .authenticate("testuser", "password")
        .post(
            "/worlds",
            json!({
              "name": "Test World"
            }),
        )
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header_regex(
          "Link",
          r#"</users/7da4cb77-8839-4805-b93a-f4c536c8bc85>; rel="author"; title="Test User""#,
        )
        .has_header_regex(
          "Link",
          r#"</worlds/[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}>; rel="self""#,
        )
        .has_json_body(json!({
          "name": "Test World",
          "description": "",
          "url_slug": "test-world"
        }));
}
