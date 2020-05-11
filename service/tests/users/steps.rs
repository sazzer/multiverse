use super::seed::SeedUser;
use crate::world::VerticalTable;
use actix_web::http::StatusCode;
use cucumber_rust::steps;
use std::sync::Arc;
use uritemplate::UriTemplate;

steps!(crate::World => {
    given "a user exists with details:" |world, step| {
        let table = VerticalTable::from(step);

        let mut user = SeedUser::default();
        table.if_present("Username", |value| user.username = value.clone());
        table.if_present("Display Name", |value| user.display_name = value.clone());
        table.if_present("Email Address", |value| user.email_address = value.clone());
        table.if_present("Avatar URL", |value| user.avatar_url = Some(value.clone()));

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
