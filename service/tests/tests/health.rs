use crate::service::TestService;
use insta::{assert_debug_snapshot, assert_json_snapshot};

#[actix_rt::test]
async fn integration_test_healthcheck() {
    let service = TestService::new().await;
    let response = service.request(actix_web::test::TestRequest::get().uri("/health").to_request()).await;

    assert_debug_snapshot!(response.headers(), @r###""HTTP/1.1 200 OK\ncontent-type: application/json""###);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "components": {
        "db": {
          "healthy": true
        }
      },
      "healthy": true
    }
    "###);
}
