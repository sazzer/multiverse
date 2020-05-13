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

    assert_debug_snapshot!(response.headers(), @r###""HTTP/1.1 204 No Content\n""###);
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

    assert_debug_snapshot!(response.headers(), @r###""HTTP/1.1 404 Not Found\ncontent-type: application/problem+json""###);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "status": 404,
      "title": "The requested username was unknown",
      "type": "tag:multiverse,2020:users/problems/unknown_username"
    }
    "###);

}
