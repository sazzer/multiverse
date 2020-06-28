use crate::{
    data::{SeedUser, SeedWorld},
    tests::run_test,
};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use serde_json::json;

struct TestData {
    user1: SeedUser,
    world1: SeedWorld,
    world2: SeedWorld,
    world3: SeedWorld,
}

impl Default for TestData {
    fn default() -> Self {
        let user1 = SeedUser {
            user_id: uuid::Uuid::parse_str("00000000-0000-0000-0001-000000000001").unwrap(),
            display_name: "First User".to_owned(),
            ..SeedUser::default()
        };
        let world1 = SeedWorld {
            world_id: uuid::Uuid::parse_str("00000000-0000-0000-0002-000000000001").unwrap(),
            created: "2020-01-01T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
            updated: "2020-02-03T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
            owner: user1.user_id,
            name: "First World".to_owned(),
            description: "This is a test world".to_owned(),
            url_slug: "first-world".to_owned(),
            ..SeedWorld::default()
        };
        let world2 = SeedWorld {
            world_id: uuid::Uuid::parse_str("00000000-0000-0000-0002-000000000002").unwrap(),
            created: "2020-01-02T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
            updated: "2020-02-02T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
            owner: user1.user_id,
            name: "Second World".to_owned(),
            description: "This is a test world".to_owned(),
            url_slug: "second-world".to_owned(),
            ..SeedWorld::default()
        };
        let world3 = SeedWorld {
            world_id: uuid::Uuid::parse_str("00000000-0000-0000-0002-000000000003").unwrap(),
            created: "2020-01-03T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
            updated: "2020-02-01T12:00:09Z".parse::<DateTime<Utc>>().unwrap(),
            owner: user1.user_id,
            name: "Fourth World".to_owned(),
            description: "This is a test world".to_owned(),
            url_slug: "fourth-world".to_owned(),
            ..SeedWorld::default()
        };

        Self {
            user1,
            world1,
            world2,
            world3,
        }
    }
}

#[test]
fn test_list_no_worlds() {
    run_test()
        .get("/worlds")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", "")
        .has_json_body(json!({
          "entries": [],
          "pagination": {
            "offset": 0,
            "total": 0
          }
        }));
}

#[test]
fn test_list_no_worlds_with_offset() {
    run_test()
        .get("/worlds?offset=5")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", "")
        .has_json_body(json!({
          "entries": [],
          "pagination": {
            "offset": 5,
            "total": 0
          }
        }));
}

#[test]
fn test_list_one_world() {
    let data = TestData::default();
    run_test()
        .seed(data.user1)
        .seed(data.world1)
        .get("/worlds")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header_regex(
            "Link",
            r#"</worlds/00000000-0000-0000-0002-000000000001>; rel="item"; anchor="\#/entries/0""#,
        )
        .has_json_body(json!({
          "entries": [
            {
              "name": "First World",
              "description": "This is a test world",
              "url_slug": "first-world"
            }
          ],
          "pagination": {
            "offset": 0,
            "total": 1
          }
        }));
}

#[test]
fn test_list_one_world_offset() {
    let data = TestData::default();
    run_test()
        .seed(data.user1)
        .seed(data.world1)
        .get("/worlds?offset=5")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", "")
        .has_json_body(json!({
          "entries": [],
          "pagination": {
            "offset": 5,
            "total": 1
          }
        }));
}

#[test]
fn test_list_one_count_zero() {
    let data = TestData::default();
    run_test()
        .seed(data.user1)
        .seed(data.world1)
        .get("/worlds?count=0")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", "")
        .has_json_body(json!({
          "entries": [],
          "pagination": {
            "offset": 0,
            "total": 1
          }
        }));
}

#[test]
fn test_list_one_world_matching_owner() {
    let data = TestData::default();
    run_test()
        .seed(data.user1)
        .seed(data.world1)
        .get("/worlds?owner=/users/00000000-0000-0000-0001-000000000001")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header_regex(
            "Link",
            r#"</worlds/00000000-0000-0000-0002-000000000001>; rel="item"; anchor="\#/entries/0""#,
        )
        .has_json_body(json!({
          "entries": [
            {
              "name": "First World",
              "description": "This is a test world",
              "url_slug": "first-world"
            }
          ],
          "pagination": {
            "offset": 0,
            "total": 1
          }
        }));
}

#[test]
fn test_list_one_world_not_matching_owner() {
    let data = TestData::default();
    run_test()
        .seed(data.user1)
        .seed(data.world1)
        .get("/worlds?owner=/users/00000000-0000-0000-0001-000000000099")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header("Link", "")
        .has_json_body(json!({
          "entries": [],
          "pagination": {
            "offset": 0,
            "total": 0
          }
        }));
}

#[test]
fn test_list_many_worlds() {
    let data = TestData::default();
    run_test()
        .seed(data.user1)
        .seed(data.world1)
        .seed(data.world2)
        .seed(data.world3)
        .get("/worlds")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_header_regex(
            "Link",
            r#"</worlds/00000000-0000-0000-0002-000000000001>; rel="item"; anchor="\#/entries/0""#,
        )
        .has_header_regex(
            "Link",
            r#"</worlds/00000000-0000-0000-0002-000000000002>; rel="item"; anchor="\#/entries/1""#,
        )
        .has_header_regex(
            "Link",
            r#"</worlds/00000000-0000-0000-0002-000000000003>; rel="item"; anchor="\#/entries/2""#,
        )
        .has_json_body(json!({
          "entries": [
            {
              "name": "First World",
              "description": "This is a test world",
              "url_slug": "first-world"
            },
            {
              "name": "Second World",
              "description": "This is a test world",
              "url_slug": "second-world"
            },
            {
              "name": "Fourth World",
              "description": "This is a test world",
              "url_slug": "fourth-world"
            }
          ],
          "pagination": {
            "offset": 0,
            "total": 3
          }
        }));
}
