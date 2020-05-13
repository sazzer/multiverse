use crate::{service::TestService};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use serde_json::json;

#[actix_rt::test]
async fn integration_test_register_no_fields() {
    let service = TestService::new().await;

    let response = service.request(actix_web::test::TestRequest::post().uri("/users").set_json(&json!({})).to_request()).await;

    assert_debug_snapshot!("register-no_fields-headers", response.headers());
    assert_json_snapshot!("register-no_fields-body", response.to_json().unwrap());
}
