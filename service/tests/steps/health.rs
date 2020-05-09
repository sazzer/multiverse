use actix_web::http::StatusCode;
use cucumber_rust::steps;

steps!(crate::World => {
    when "I check the health of the system" |world, _step| {
        world.request(actix_web::test::TestRequest::get().uri("/health").to_request());
    };

    then "the system is healthy" |world, _step| {
        let response = world.last_response();
        assert!(response.is_some());

        let response = response.unwrap();
        assert_eq!(StatusCode::OK, response.status);
    };
});
