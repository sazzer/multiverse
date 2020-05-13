use crate::service::TestService;
use insta::{assert_debug_snapshot, assert_json_snapshot};

#[actix_rt::test]
async fn integration_test_healthcheck() {
    let service = TestService::new().await;
    let response = service.request(actix_web::test::TestRequest::get().uri("/health").to_request()).await;

    assert_debug_snapshot!("healthcheck-headers", response.headers());
    assert_json_snapshot!("healthcheck-body", response.to_json().unwrap());
}
