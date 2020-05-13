use crate::{service::TestService};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use serde_json::json;
use rstest::rstest;

#[rstest(name, username, display_name, email_address, avatar_url, password,
    case("no_fields", None, None, None, None, None),
    case("all_blank", Some(""), Some(""), Some(""), Some(""), Some("")),
    case("all_whitespace", Some(" "), Some(" "), Some(" "), Some(" "), Some(" ")),
    case("no_username", None, Some("Display Name"), Some("test@example.com"), Some("http://example.com"), Some("Password")),
    case("no_email", Some("username"), Some("Display Name"), None, Some("http://example.com"), Some("Password")),
    case("no_password", Some("username"), Some("Display Name"), Some("test@example.com"), Some("http://example.com"), None),
)]
#[actix_rt::test]
async fn integration_test_register_validation_failure(name: &str, username: Option<&str>, display_name: Option<&str>, email_address: Option<&str>, avatar_url: Option<&str>, password: Option<&str>) {
    let service = TestService::new().await;

    let body = json!({
        "username": username,
        "display_name": display_name,
        "email_address": email_address,
        "avatar_url": avatar_url,
        "password": password
    });
    let response = service.request(actix_web::test::TestRequest::post().uri("/users").set_json(&body).to_request()).await;

    assert_debug_snapshot!(format!("register-{}-headers", name), response.headers());
    assert_json_snapshot!(format!("register-{}-body", name), response.to_json().unwrap());
}
