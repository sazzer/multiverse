use crate::{data::SeedUser, tests::run_test};
use rocket::http::Status;
use rstest::rstest;
use serde_json::json;
use uritemplate::UriTemplate;

#[rstest(
    username,
    case("simple"),
    case("!@#$%^&*()_"),
    case(",.;'\\[]<>?:\"|{}")
)]
#[test]
fn test_lookup_unknown_username(username: &str) {
    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();

    run_test()
        .get(url)
        .has_status(Status::NotFound)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:users/problems/unknown_username",
            "title": "The requested username was unknown",
            "status": 404
        }));
}

#[rstest(
    username,
    case("simple"),
    case("!@#$%^&*()_"),
    case(",.;'\\[]<>?:\"|{}")
)]
#[test]
fn test_lookup_known_username(username: &str) {
    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();

    run_test()
        .seed(&SeedUser {
            username: username.to_owned(),
            ..SeedUser::default()
        })
        .get(url)
        .has_status(Status::NoContent)
        .has_header("Cache-Control", "private, max-age=3600");
}
