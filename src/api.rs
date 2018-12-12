/// Where our API handlers go.
use crate::app::error::{CratifyError, ValidationError};
use crate::app::model::*;
use crate::db::exec::msg::{AreCredentialsValid, CreateUser, IsUsernameAvailable};
use crate::AppState;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse};
use futures::future::Future;

type ApiResponse = Box<Future<Item = HttpResponse, Error = CratifyError>>;

/// Handle a user signup request.
pub(crate) fn signup(req: &HttpRequest<AppState>) -> ApiResponse {
    let db = req.state().db_addr.clone();
    req.json()
        .from_err()
        .and_then(move |sr: SignupRequest| {
            db.send(IsUsernameAvailable {
                username: sr.username.clone(),
            })
            .from_err()
            .and_then(move |res| {
                res.and_then(|available| {
                    if available {
                        db.send(CreateUser {
                            username: sr.username.clone(),
                            password: sr.password.clone(),
                        })
                        .from_err()
                        .and_then(|res| res.and_then(|_| Ok(HttpResponse::Ok().finish())))
                        .wait()
                    } else {
                        Err(ValidationError::TakenUsername.into())
                    }
                })
            })
        })
        .responder()
}

/// Determine if username is available.
pub(crate) fn username_available(req: &HttpRequest<AppState>) -> ApiResponse {
    let db = req.state().db_addr.clone();
    req.json()
        .from_err()
        .and_then(move |u: UsernameAvailableRequest| {
            db.send(IsUsernameAvailable {
                username: u.username,
            })
            .from_err()
            .and_then(|res| {
                res.and_then(|available| {
                    Ok(HttpResponse::Ok().json(UsernameAvailableResponse { available }))
                })
            })
        })
        .responder()
}

/// Log user in.
pub(crate) fn login(req: &HttpRequest<AppState>) -> ApiResponse {
    let db = req.state().db_addr.clone();
    let req_clone = req.clone();
    req.json()
        .from_err()
        .and_then(move |lr: LoginRequest| {
            db.send(AreCredentialsValid {
                username: lr.username.clone(),
                password: lr.password.clone(),
            })
            .from_err()
            .and_then(move |res| {
                res.and_then(|login_successful| {
                    if login_successful {
                        req_clone.remember(lr.username);
                        Ok(HttpResponse::Ok().finish())
                    } else {
                        Err(ValidationError::InvalidCredentials.into())
                    }
                })
            })
        })
        .responder()
}
