use crate::{
    data::{SeedUser, hash_password},
    service::{TestResponse, TestService},
};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use rstest::rstest;
use uritemplate::UriTemplate;
use serde_json::json;
use rocket::http::Header;

#[rstest(
    test_name,
    username,
    case("simple", "known"),
    case("symbols1", "!@#$%^&*()_"),
    case("symbols2", ",.;'\\[]<>?:\"|{}")
)]
#[test]
fn integration_test_get_known_user_by_username(test_name: &str, username: &str) {
    let service = TestService::new();

    service.seed(SeedUser {
        username: username.to_owned(),
        password: hash_password("password"),
        display_name: username.to_owned(),
        email_address: format!("{}@example.com", username),
        avatar_url: Some(format!("http://example.com/{}", username)),
        version: "c7040ef8-df74-4106-a017-16fc17fcaa91".parse().unwrap(),
        ..SeedUser::default()
    });

    let client = service.test_client();

    let body = serde_json::to_string(&json!({
        "username": username,
        "password": "password",
    }))
    .unwrap();
    let mut login_response: TestResponse = client.post("/login").body(&body).dispatch().into();
    let login_body = login_response.to_json().unwrap();
    let token = login_body.pointer("/token/token").and_then(|v| v.as_str()).unwrap();

    let url = UriTemplate::new("/users/{username}")
        .set("username", username)
        .build();
    let mut response: TestResponse = client.get(url)
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch()
        .into();

    assert_debug_snapshot!(
        format!("get_known_user_by_username-{}-headers", test_name),
        response.headers()
    );
    assert_json_snapshot!(
        format!("get_known_user_by_username-{}-body", test_name),
        response.to_json().unwrap()
    );
}

#[test]
fn integration_test_get_known_user_by_username_unauthenticated() {
    let service = TestService::new();

    service.seed(SeedUser {
        username: "username".to_owned(),
        display_name: "display name".to_owned(),
        email_address: "email@example.com".to_owned(),
        avatar_url: Some("http://example.com/avatar".to_owned()),
        version: "c7040ef8-df74-4106-a017-16fc17fcaa91".parse().unwrap(),
        ..SeedUser::default()
    });

    let client = service.test_client();

    let mut response: TestResponse = client.get("/users/username")
        .dispatch()
        .into();

    assert_debug_snapshot!(
        "get_known_user_by_username_authenticated-headers",
        response.headers()
    );
    assert_json_snapshot!(
        "get_known_user_by_username_authenticated-body",
        response.to_json().unwrap()
    );
}

#[rstest(
    test_name,
    username,
    case("simple", "known"),
    case("symbols1", "!@#$%^&*()_"),
    case("symbols2", ",.;'\\[]<>?:\"|{}")
)]
#[test]
fn integration_test_get_unknown_user_by_username(test_name: &str, username: &str) {
    let service = TestService::new();

    let url = UriTemplate::new("/users/{username}")
        .set("username", username)
        .build();
    let client = service.test_client();
    let mut response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!(
        format!("get_unknown_user_by_username-{}-headers", test_name),
        response.headers()
    );
    assert_json_snapshot!(
        format!("get_unknown_user_by_username-{}-body", test_name),
        response.to_json().unwrap()
    );
}

#[test]
fn integration_test_head_known_user_by_username() {
    let service = TestService::new();

    service.seed(SeedUser {
        username: "known".to_owned(),
        display_name: "known".to_owned(),
        email_address: "known@example.com".to_owned(),
        avatar_url: Some("http://example.com/known".to_owned()),
        version: "c7040ef8-df74-4106-a017-16fc17fcaa91".parse().unwrap(),
        ..SeedUser::default()
    });

    let client = service.test_client();
    let mut response: TestResponse = client.head("/users/known").dispatch().into();

    assert_debug_snapshot!("head_known_user_by_username-headers", response.headers());
    assert_eq!(response.to_string(), "");
}

#[test]
fn integration_test_head_unknown_user_by_username() {
    let service = TestService::new();

    let client = service.test_client();
    let mut response: TestResponse = client.head("/users/unknown").dispatch().into();

    assert_debug_snapshot!("head_unknown_user_by_username-headers", response.headers());
    assert_eq!(response.to_string(), "");
}
