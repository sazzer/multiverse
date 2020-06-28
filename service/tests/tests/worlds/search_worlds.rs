use crate::{
    data::{SeedUser, SeedWorld},
    tests::run_test,
};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use serde_json::json;
use std::collections::HashMap;

struct TestData {
    user1: SeedUser,
    user2: SeedUser,
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
        let user2 = SeedUser {
            user_id: uuid::Uuid::parse_str("00000000-0000-0000-0001-000000000002").unwrap(),
            display_name: "Second User".to_owned(),
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
            owner: user2.user_id,
            name: "Fourth World".to_owned(),
            description: "This is a test world".to_owned(),
            url_slug: "fourth-world".to_owned(),
            ..SeedWorld::default()
        };

        Self {
            user1,
            user2,
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
        .seed_many(&[&data.user1, &data.world1])
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
        .seed_many(&[&data.user1, &data.world1])
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
        .seed_many(&[&data.user1, &data.world1])
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
        .seed_many(&[&data.user1, &data.world1])
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
        .seed_many(&[&data.user1, &data.world1])
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

macro_rules! test_list_many_worlds {
  ($($name:ident: $value:literal -> ($w1:ident,$w2:ident,$w3:ident),)*) => {
  $(
      #[test]
      fn $name() {
        let data = TestData::default();

        let mut worlds = HashMap::new();
        worlds.insert("world1", json!({
          "name": "First World",
          "description": "This is a test world",
          "url_slug": "first-world"
        }));
        worlds.insert("world2", json!({
          "name": "Second World",
          "description": "This is a test world",
          "url_slug": "second-world"
        }));
        worlds.insert("world3", json!({
          "name": "Fourth World",
          "description": "This is a test world",
          "url_slug": "fourth-world"
        }));

        run_test()
            .seed_many(&[
                &data.user1,
                &data.user2,
                &data.world1,
                &data.world2,
                &data.world3,
            ])
            .get($value)
            .has_status(Status::Ok)
            .has_header("Content-Type", "application/json")
            .has_header_regex(
                "Link",
                format!(r#"</worlds/{}>; rel="item"; anchor="\#/entries/0""#, data.$w1.world_id),
            )
            .has_header_regex(
                "Link",
                format!(r#"</worlds/{}>; rel="item"; anchor="\#/entries/1""#, data.$w2.world_id),
            )
            .has_header_regex(
                "Link",
                format!(r#"</worlds/{}>; rel="item"; anchor="\#/entries/2""#, data.$w3.world_id),
            )
            .has_json_body(json!({
              "entries": [worlds[stringify!($w1)], worlds[stringify!($w2)], worlds[stringify!($w3)]],
              "pagination": {
                "offset": 0,
                "total": 3
              }
            }));
      }
  )*
  }
}

test_list_many_worlds! {
  test_list_many_worlds_default_ordering: "/worlds" -> (world1,world2,world3),
  test_list_many_worlds_name_ascending: "/worlds?sort=+name" -> (world1,world3,world2),
  test_list_many_worlds_name_descending: "/worlds?sort=-name" -> (world2,world3,world1),
  test_list_many_worlds_name_default: "/worlds?sort=name" -> (world1,world3,world2),
  test_list_many_worlds_created_ascending: "/worlds?sort=+created" -> (world1,world2,world3),
  test_list_many_worlds_created_descending: "/worlds?sort=-created" -> (world3,world2,world1),
  test_list_many_worlds_created_default: "/worlds?sort=created" -> (world3,world2,world1),
  test_list_many_worlds_updated_ascending: "/worlds?sort=+updated" -> (world3,world2,world1),
  test_list_many_worlds_updated_descending: "/worlds?sort=-updated" -> (world1,world2,world3),
  test_list_many_worlds_updated_default: "/worlds?sort=updated" -> (world1,world2,world3),
  test_list_many_worlds_owner_ascending: "/worlds?sort=+owner" -> (world1,world2,world3),
  test_list_many_worlds_owner_descending: "/worlds?sort=-owner" -> (world3,world1,world2),
  test_list_many_worlds_owner_default: "/worlds?sort=owner" -> (world1,world2,world3),

}
