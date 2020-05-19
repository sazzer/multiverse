use crate::{
    data::SeedUser,
    service::{TestResponse, TestService},
};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use rstest::rstest;
use serde_json::json;

#[rstest(
    name,
    username,
    display_name,
    email_address,
    avatar_url,
    password,
    case("no_fields", None, None, None, None, None),
    case("all_blank", Some(""), Some(""), Some(""), Some(""), Some("")),
    case(
        "all_whitespace",
        Some(" "),
        Some(" "),
        Some(" "),
        Some(" "),
        Some(" ")
    ),
    case(
        "no_username",
        None,
        Some("Display Name"),
        Some("test@example.com"),
        Some("http://example.com"),
        Some("Password")
    ),
    case(
        "no_email",
        Some("username"),
        Some("Display Name"),
        None,
        Some("http://example.com"),
        Some("Password")
    ),
    case(
        "no_password",
        Some("username"),
        Some("Display Name"),
        Some("test@example.com"),
        Some("http://example.com"),
        None
    )
)]
#[test]
fn integration_test_register_validation_failure(
    name: &str,
    username: Option<&str>,
    display_name: Option<&str>,
    email_address: Option<&str>,
    avatar_url: Option<&str>,
    password: Option<&str>,
) {
    let service = TestService::new();

    let body = serde_json::to_string(&json!({
        "username": username,
        "display_name": display_name,
        "email_address": email_address,
        "avatar_url": avatar_url,
        "password": password
    }))
    .unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/register").body(&body).dispatch().into();

    assert_debug_snapshot!(
        format!("register_validation_failure-{}-headers", name),
        response.headers()
    );
    assert_json_snapshot!(
        format!("register_validation_failure-{}-body", name),
        response.to_json().unwrap()
    );
}

#[test]
fn integration_test_register_duplicate_username() {
    let service = TestService::new();

    service.seed(SeedUser {
        username: "username".to_owned(),
        ..SeedUser::default()
    });

    let body = serde_json::to_string(&json!({
        "username": "username",
        "display_name": "display_name",
        "email_address": "test@example.com",
        "password": "password"
    }))
    .unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/register").body(&body).dispatch().into();

    assert_debug_snapshot!("register_duplicate_username-headers", response.headers());
    assert_json_snapshot!(
        "register_duplicate_username-body",
        response.to_json().unwrap()
    );
}

#[test]
fn integration_test_register_success() {
    let service = TestService::new();

    let body = serde_json::to_string(&json!({
        "username": "username",
        "display_name": "display_name",
        "email_address": "test@example.com",
        "password": "password"
    }))
    .unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/register").body(&body).dispatch().into();

    assert_debug_snapshot!("register_success-headers", response.headers());
    assert_json_snapshot!("register_success-body", response.to_json().unwrap(), {
        ".token.token" => "[access_token]",
        ".token.valid_until" => "[access_token_expiry]"
    });
}
