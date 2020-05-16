use crate::service::{TestResponse, TestService};
use insta::{assert_debug_snapshot, assert_json_snapshot};

#[test]
fn integration_test_healthcheck() {
    let service = TestService::new();
    let client = service.test_client();
    let mut response: TestResponse = client.get("/health").dispatch().into();

    assert_debug_snapshot!("healthcheck-headers", response.headers());
    assert_json_snapshot!("healthcheck-body", response.to_json().unwrap());
}
