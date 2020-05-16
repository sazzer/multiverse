use crate::service::TestService;
use insta::{assert_debug_snapshot, assert_json_snapshot};

#[test]
fn integration_test_healthcheck() {
    let service = TestService::new();
    let response = service.request(
        actix_web::test::TestRequest::get()
            .uri("/health")
            .to_request(),
    );

    assert_debug_snapshot!("healthcheck-headers", response.headers());
    assert_json_snapshot!("healthcheck-body", response.to_json().unwrap());
}
