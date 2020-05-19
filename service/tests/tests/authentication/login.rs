use crate::{
    data::{hash_password, SeedUser},
    service::{TestResponse, TestService},
};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use serde_json::json;

#[test]
fn integration_test_login_no_username() {
    let service = TestService::new();

    let body = serde_json::to_string(&json!({})).unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/login").body(&body).dispatch().into();

    assert_debug_snapshot!("login_no_username-headers", response.headers());
    assert_json_snapshot!("login_no_username-body", response.to_json().unwrap());
}

#[test]
fn integration_test_login_unknown_username() {
    let service = TestService::new();

    let body = serde_json::to_string(&json!({
        "username": "username",
        "password": "password"
    }))
    .unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/login").body(&body).dispatch().into();

    assert_debug_snapshot!("login_unknown_username-headers", response.headers());
    assert_json_snapshot!("login_unknown_username-body", response.to_json().unwrap());
}

#[test]
fn integration_test_login_incorrect_password() {
    let service = TestService::new();

    service.seed(SeedUser {
        username: "username".to_owned(),
        password: hash_password("password"),
        ..Default::default()
    });

    let body = serde_json::to_string(&json!({
        "username": "username",
        "password": "incorrect"
    }))
    .unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/login").body(&body).dispatch().into();

    assert_debug_snapshot!("login_incorrect_password-headers", response.headers());
    assert_json_snapshot!("login_incorrect_password-body", response.to_json().unwrap());
}

#[test]
fn integration_test_login_success() {
    let service = TestService::new();

    service.seed(SeedUser {
        username: "username".to_owned(),
        password: hash_password("password"),
        display_name: "display_name".to_owned(),
        email_address: "test@example.com".to_owned(),
        ..Default::default()
    });

    let body = serde_json::to_string(&json!({
        "username": "username",
        "password": "password",
    }))
    .unwrap();
    let client = service.test_client();
    let mut response: TestResponse = client.post("/login").body(&body).dispatch().into();

    assert_debug_snapshot!("login_success-headers", response.headers());
    assert_json_snapshot!("login_success-body", response.to_json().unwrap(), {
        ".token.token" => "[access_token]",
        ".token.valid_until" => "[access_token_expiry]"
    });
}
