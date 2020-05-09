use actix_web::http::StatusCode;
use cucumber_rust::steps;
use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
};
use serde_json::json;

steps!(crate::World => {
    when "I check the health of the system" |world, _step| {
        world.request(actix_web::test::TestRequest::get().uri("/health").to_request());
    };

    then "the system is healthy" |world, _step| {
        let response = world.last_response();
        assert_that!(&response.map(|r| r.status), maybe_some(eq(StatusCode::OK)));
    };

    then regex r"^the component '(.*)' is healthy" (String) |world, component, _step| {
        let value = world.extract_response_value(format!("$.components.{}.healthy", component));

        assert_that!(&value, maybe_some(eq(json!(true))));
    };
});
