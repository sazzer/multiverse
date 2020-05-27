use crate::tests::run_test;
use rocket::http::Status;
use serde_json::json;

#[test]
fn test_health() {
    run_test()
        .get("/health")
        .has_status(Status::Ok)
        .has_header("Content-Type", "application/json")
        .has_json_body(json!({
            "healthy": true,
            "components": {
                "db": {
                    "healthy": true
                }
            }
        }));
}
