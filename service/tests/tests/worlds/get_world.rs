use crate::{
    data::{SeedUser, SeedWorld},
    tests::run_test,
};
use rocket::http::Status;
use serde_json::json;

#[test]
fn test_get_unknown_world() {
    run_test()
        .get("/worlds/d6c124d5-1060-48ef-aeb8-e591a8261c2b")
        .has_status(Status::NotFound)
        .has_header("Content-Type", "application/problem+json")
        .has_json_body(json!({
            "type": "tag:multiverse,2020:worlds/problems/unknown_world_id",
            "title": "The requested world ID was unknown",
            "status": 404
        }));
}

#[test]
fn test_get_known_world() {
    let user = SeedUser {
        user_id: uuid::Uuid::parse_str("7da4cb77-8839-4805-b93a-f4c536c8bc85").unwrap(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };
    let world = SeedWorld {
        world_id: uuid::Uuid::parse_str("d6c124d5-1060-48ef-aeb8-e591a8261c2b").unwrap(),
        owner: user.user_id,
        name: "Test World".to_owned(),
        description: "This is a test world".to_owned(),
        url_slug: "test-world".to_owned(),
        ..SeedWorld::default()
    };

    run_test()
        .seed(user)
        .seed(world)
        .get("/worlds/d6c124d5-1060-48ef-aeb8-e591a8261c2b")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header_regex(
            "Link",
            r#"</users/7da4cb77-8839-4805-b93a-f4c536c8bc85>; rel="author"; title="Test User""#,
        )
        .has_header_regex(
            "Link",
            r#"</worlds/d6c124d5-1060-48ef-aeb8-e591a8261c2b>; rel="self""#,
        )
        .has_json_body(json!({
          "name": "Test World",
          "description": "This is a test world",
          "url_slug": "test-world"
        }));
}
