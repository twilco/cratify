/// Where our API handlers will go.
use actix_web::{AsyncResponder, Error, FromRequest, HttpRequest, HttpResponse, Json, Responder};
use crate::app::models::*;
use crate::AppState;
use futures::future::{result, Future};

/// Handle a user signup request.
pub(crate) fn signup(
    req: &HttpRequest<AppState>,
) -> Box<Future<Item = Json<SignupRequest>, Error = Error>> {
    let signup = Json::<SignupRequest>::extract(&req);
    result(signup.wait()).responder()
}
