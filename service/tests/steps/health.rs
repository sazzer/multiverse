use actix_web::http::StatusCode;
use cucumber_rust::steps;
use serde_json::json;

steps!(crate::World => {
    when "I check the health of the system" |world, _step| {
        world.request(actix_web::test::TestRequest::get().uri("/health").to_request());
    };

    then "the system is healthy" |world, _step| {
        world.assert_status_code(StatusCode::OK);
    };

    then regex r"^the component '(.*)' is healthy" (String) |world, component, _step| {
        world.assert_json_value(format!("$.components.{}.healthy", component), json!(true));
    };
});
