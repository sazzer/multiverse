use crate::{
    data::SeedUser,
    service::{TestResponse, TestService},
};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use rstest::rstest;
use uritemplate::UriTemplate;

#[rstest(
    username,
    case("known"),
    case("!@#$%^&*()_"),
    case(",.;'\\[]<>?:\"|{}")
)]
#[test]
fn integration_test_lookup_known_username(username: &str) {
    let service = TestService::new();

    let user = SeedUser {
        username: username.to_owned(),
        ..SeedUser::default()
    };
    service.seed(user);

    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();
    let client = service.test_client();
    let response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!(
        format!("lookup_known_username-{}-headers", username),
        response.headers()
    );
}

#[rstest(
    username,
    case("unknown"),
    case("!@#$%^&*()_"),
    case(",.;'\\[]<>?:\"|{}")
)]
#[test]
fn integration_test_lookup_unknown_username(username: &str) {
    let service = TestService::new();

    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();
    let client = service.test_client();
    let mut response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!(
        format!("lookup_unknown_username-{}-headers", username),
        response.headers()
    );
    assert_json_snapshot!(
        format!("lookup_unknown_username-{}-body", username),
        response.to_json().unwrap()
    );
}
