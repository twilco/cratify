/// Various models used throughout the application.  Database models should not go here - use db/models.rs for those.
use actix_web::{HttpRequest, HttpResponse, Responder};
use failure::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Responder for LoginRequest {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct SignupRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Responder for SignupRequest {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct UsernameAvailableRequest {
    pub(crate) username: String,
}

impl Responder for UsernameAvailableRequest {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, _req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct UsernameAvailableResponse {
    pub(crate) available: bool,
}
