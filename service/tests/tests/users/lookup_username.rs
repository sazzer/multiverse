use crate::{service::TestService, data::SeedUser};
use insta::{assert_debug_snapshot, assert_json_snapshot};
use uritemplate::UriTemplate;
use rstest::rstest;

#[rstest(username,
    case("known"),
    case("!@#$%^&*()_"),
    case(",.;'\\[]<>?:\"|{}"),
)]
#[actix_rt::test]
async fn integration_test_lookup_known_username(username: &str) {
    let service = TestService::new().await;

    let user = SeedUser {
        username: username.to_owned(),
        ..Default::default()
    };
    service.seed(user).await;

    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();
    let response = service.request(actix_web::test::TestRequest::get().uri(&url).to_request()).await;

    assert_debug_snapshot!(format!("lookup_known_username-{}-headers", username), response.headers());
}


#[rstest(username,
    case("unknown"),
    case("!@#$%^&*()_"),
    case(",.;'\\[]<>?:\"|{}"),
)]
#[actix_rt::test]
async fn integration_test_lookup_unknown_username(username: &str) {
    let service = TestService::new().await;

    let url = UriTemplate::new("/usernames/{username}")
        .set("username", username)
        .build();
    let response = service.request(actix_web::test::TestRequest::get().uri(&url).to_request()).await;

    assert_debug_snapshot!(format!("lookup_unknown_username-{}-headers", username), response.headers());
    assert_json_snapshot!(format!("lookup_unknown_username-{}-body", username), response.to_json().unwrap());

}
