use actix_web::test;
use insta::{assert_display_snapshot, assert_json_snapshot};
use crate::integration::service::TestService;

#[actix_rt::test]
async fn lookup_unknown_username() {
    let service = TestService::new().await;

    let response = service
        .request(test::TestRequest::get().uri("/usernames/unknown").to_request())
        .await;

    assert_display_snapshot!(response.headers());
    assert_json_snapshot!(response.to_json().unwrap());
}

#[actix_rt::test]
async fn lookup_known_username() {
    let service = TestService::new().await;

    // TODO: Seed user

    let response = service
        .request(test::TestRequest::get().uri("/usernames/known").to_request())
        .await;

    assert_display_snapshot!(response.headers());
}
