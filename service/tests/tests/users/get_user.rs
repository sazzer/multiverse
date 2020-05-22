use crate::{
    data::{hash_password, SeedUser},
    service::{login, TestResponse, TestService},
};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use rstest::rstest;
use uritemplate::UriTemplate;
use uuid::Uuid;

#[rstest(
    test_name,
    username,
    case("simple", "known"),
    case("symbols1", "!@#$%^&*()_"),
    case("symbols2", ",.;'\\[]<>?:\"|{}")
)]
#[test]
fn integration_test_get_known_user(test_name: &str, username: &str) {
    let service = TestService::new();

    let user = service.seed(SeedUser {
        username: username.to_owned(),
        password: hash_password("password"),
        display_name: username.to_owned(),
        email_address: format!("{}@example.com", username),
        avatar_url: Some(format!("http://example.com/{}", username)),
        version: "c7040ef8-df74-4106-a017-16fc17fcaa91".parse().unwrap(),
        ..SeedUser::default()
    });

    let client = service.test_client();

    let authorization = login(&client, username, "password");

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();
    let mut response: TestResponse = client
        .get(url)
        .header(rocket::http::Header::new("Authorization", authorization))
        .dispatch()
        .into();

    assert_debug_snapshot!(
        format!("get_known_user-{}-headers", test_name),
        response.headers()
    );
    assert_json_snapshot!(
        format!("get_known_user-{}-body", test_name),
        response.to_json().unwrap()
    );
}

#[test]
fn integration_test_get_known_user_unauthenticated() {
    let service = TestService::new();

    let user = service.seed(SeedUser {
        username: "username".to_owned(),
        display_name: "display name".to_owned(),
        email_address: "email@example.com".to_owned(),
        avatar_url: Some("http://example.com/avatar".to_owned()),
        version: "c7040ef8-df74-4106-a017-16fc17fcaa91".parse().unwrap(),
        ..SeedUser::default()
    });

    let client = service.test_client();

    let url = UriTemplate::new("/users/{id}")
        .set("id", user.user_id.to_string())
        .build();
    let mut response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!("get_known_user_unauthenticated-headers", response.headers());
    assert_json_snapshot!(
        "get_known_user_unauthenticated-body",
        response.to_json().unwrap()
    );
}

#[test]
fn integration_test_get_unknown_user() {
    let service = TestService::new();

    let url = UriTemplate::new("/users/{id}")
        .set("id", Uuid::new_v4().to_string())
        .build();
    let client = service.test_client();
    let mut response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!("get_unknown_user-headers", response.headers());
    assert_json_snapshot!("get_unknown_user-body", response.to_json().unwrap());
}
