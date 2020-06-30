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
            user_id: "e22ecfe0-bf55-42fb-9308-e1104b97a4d7".parse().unwrap(),
            username: username.to_owned(),
            display_name: "Test User".to_owned(),
            ..SeedUser::default()
        })
        .get(url)
        .has_status(Status::NoContent)
        .has_header("Cache-Control", "private, max-age=3600")
        .has_header_regex(
            "Link",
            r#"</users/e22ecfe0-bf55-42fb-9308-e1104b97a4d7>; rel="related"; title="Test User""#,
        );
}
