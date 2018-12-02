use crate::app::models::*;
use crate::db::exec::msg::{CreateUser, IsUsernameAvailable};
use crate::AppState;
/// Where our API handlers go.
use actix_web::{AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse, Json};
use futures::future::Future;

/// Handle a user signup request.
pub(crate) fn signup(
    req: &HttpRequest<AppState>,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let db = req.state().db_addr.clone();
    req.json()
        .from_err()
        .and_then(move |sr: SignupRequest| {
            db.send(IsUsernameAvailable {
                username: sr.username.clone(),
            })
            .from_err()
            .and_then(move |res| match res {
                Ok(available) => {
                    if available {
                        db.send(CreateUser {
                            username: sr.username.clone(),
                            password: sr.password.clone(),
                        })
                        .from_err::<failure::Error>()
                        .and_then(move |res| match res {
                            Ok(_) => Ok(HttpResponse::Ok().finish()),
                            Err(e) => {
                                error!("couldn't save user: {}", e);
                                Ok(HttpResponse::InternalServerError().finish())
                            }
                        });
                    }
                    info!(
                        "attempted signup with unavailable username of {}",
                        sr.username
                    );
                    Ok(HttpResponse::BadRequest().finish())
                }
                Err(e) => {
                    error!("error determining username availability: {}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            })
        })
        .responder()
}

/// Determine if username is available.
pub(crate) fn username_available(
    req: &HttpRequest<AppState>,
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let db = req.state().db_addr.clone();
    req.json()
        .from_err()
        .and_then(move |u: UsernameAvailableRequest| {
            db.send(IsUsernameAvailable {
                username: u.username,
            })
            .from_err()
            .and_then(|res| match res {
                Ok(available) => {
                    info!("available {:?}", available);
                    Ok(HttpResponse::Ok().json(UsernameAvailableResponse { available }))
                }
                Err(e) => {
                    error!("couldn't determine username availability: {}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            })
        })
        .responder()
}
