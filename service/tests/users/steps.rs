use super::seed::SeedUser;
use actix_web::http::StatusCode;
use cucumber_rust::steps;
use std::sync::Arc;
use uritemplate::UriTemplate;

steps!(crate::World => {
    given "a user exists with details:" |world, _step| {
        let mut user = SeedUser::default();
        user.username = "known".to_owned();

        world.seed(Arc::new(user));
    };

    when regex r"^I look up the username '(.+)'" (String) |world, username, _step| {
        let url = UriTemplate::new("/usernames/{username}")
            .set("username", username)
            .build();
        world.request(actix_web::test::TestRequest::get().uri(&url).to_request());
    };

    then "the username does not exist" |world, _step| {
        world.assert_problem(StatusCode::NOT_FOUND, "tag:multiverse,2020:users/problems/unknown_username");
    };

    then "the username does exist" |world, _step| {
        world.assert_status_code(StatusCode::NO_CONTENT);
    };
});
