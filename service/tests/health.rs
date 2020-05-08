mod service;
use actix_web::test;
use insta::{assert_display_snapshot, assert_json_snapshot};

#[actix_rt::test]
async fn integration_test_health() {
    let service = service::TestService::new().await;

    let response = service.request(test::TestRequest::get().uri("/health").to_request()).await;

    assert_display_snapshot!(response.headers());
    assert_json_snapshot!(response.to_json().unwrap());
}