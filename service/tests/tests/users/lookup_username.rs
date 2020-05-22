use crate::{
    data::SeedUser,
    service::{TestResponse, TestService},
};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use rstest::rstest;
use uritemplate::UriTemplate;

#[rstest(
    test_name,
    username,
    case("simple", "known"),
    case("symbols1", "!@#$%^&*()_"),
    case("symbols2", ",.;'\\[]<>?:\"|{}")
)]
#[test]
fn integration_test_lookup_known_user_by_username(test_name: &str, username: &str) {
    let service = TestService::new();

    service.seed(SeedUser {
        username: username.to_owned(),
        ..SeedUser::default()
    });

    let client = service.test_client();

    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();
    let mut response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!(
        format!("lookup_known_user_by_username-{}-headers", test_name),
        response.headers()
    );
    assert_eq!(response.to_string(), "");
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

    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();
    let client = service.test_client();
    let mut response: TestResponse = client.get(url).dispatch().into();

    assert_debug_snapshot!(
        format!("lookup_unknown_user_by_username-{}-headers", test_name),
        response.headers()
    );
    assert_json_snapshot!(
        format!("lookup_unknown_user_by_username-{}-body", test_name),
        response.to_json().unwrap()
    );
}
