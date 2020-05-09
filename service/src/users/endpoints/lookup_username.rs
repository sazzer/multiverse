use actix_web::{get, web, Either, HttpResponse, Responder};

/// Actix handler to see if a username is already registered or not
///
/// # Parameters
/// - `path` - The details of the parameters from the URL
///
/// # Returns
/// If the username is known then an empty response.
/// If the username is not registered then an RFC-7807 problem response indicating this.
#[tracing::instrument(name = "GET /usernames/{username}", skip())]
#[get("/usernames/{username}")]
pub async fn lookup_username(path: web::Path<(String,)>) -> Either<impl Responder, String> {
    tracing::info!("Hello");

    match path.0.as_ref() {
        "sazzer" => Either::A(HttpResponse::NoContent()),
        _ => Either::B("Oops".to_owned()),
    }
}
