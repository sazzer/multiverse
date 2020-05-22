use crate::service::TestResponse;
use rocket::local::Client;
use serde_json::json;

pub fn login<U, P>(client: &Client, username: U, password: P) -> String
where
    U: Into<String>,
    P: Into<String>,
{
    let body = serde_json::to_string(&json!({
        "username": username.into(),
        "password": password.into(),
    }))
    .unwrap();

    let mut login_response: TestResponse = client.post("/login").body(&body).dispatch().into();
    let login_body = login_response.to_json().unwrap();
    let token = login_body
        .pointer("/token/token")
        .and_then(|v| v.as_str())
        .unwrap();

    format!("Bearer {}", token)
}
