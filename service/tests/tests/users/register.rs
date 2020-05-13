use crate::{service::TestService};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use serde_json::json;

#[actix_rt::test]
async fn integration_test_register_no_fields() {
    let service = TestService::new().await;

    let response = service.request(actix_web::test::TestRequest::post().uri("/users").set_json(&json!({})).to_request()).await;

    assert_debug_snapshot!(response.headers(), @r###""HTTP/1.1 422 Unprocessable Entity\ncontent-type: application/problem+json""###);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "fields": {
        "email_address": {
          "title": "The required field was missing",
          "type": "tag:multiverse,2020:problems/validation_error/missing"
        },
        "password": {
          "title": "The required field was missing",
          "type": "tag:multiverse,2020:problems/validation_error/missing"
        },
        "username": {
          "title": "The required field was missing",
          "type": "tag:multiverse,2020:problems/validation_error/missing"
        }
      },
      "status": 422,
      "title": "A validation error occurred",
      "type": "tag:multiverse,2020:problems/validation_error"
    }
    "###);
}
