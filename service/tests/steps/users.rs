use actix_web::http::StatusCode;
use cucumber_rust::steps;

steps!(crate::World => {
    when regex r"^I look up the username '(.+)'" (String) |world, username, _step| {
        let url = format!("/usernames/{}", username);
        world.request(actix_web::test::TestRequest::get().uri(&url).to_request());
    };

    then "the username does not exist" |world, _step| {
        world.assert_problem(StatusCode::NOT_FOUND, "tag:multiverse,2020:users/problems/unknown_username");
    };
});
