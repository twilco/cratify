/// Where our API handlers go.
use crate::app::error::{CratifyError, UserError};
use crate::app::model::*;
use crate::db::exec::msg::{AreCredentialsValid, CreateUser, IsUsernameAvailable};
use crate::AppState;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{AsyncResponder, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use futures::future::Future;

type ApiResponse = Box<Future<Item = HttpResponse, Error = CratifyError>>;

/// Handle a user signup request.
pub(crate) fn signup(req: &HttpRequest<AppState>) -> ApiResponse {
    let db = req.state().db_addr.clone();
    let req_clone = req.clone();
    req.json()
        .from_err()
        .and_then(move |sr: SignupRequest| {
            db.send(IsUsernameAvailable {
                username: sr.username.clone(),
            })
            .from_err()
            .and_then(move |res| {
                res.and_then(move |available| {
                    if available {
                        db.send(CreateUser {
                            username: sr.username.clone(),
                            password: sr.password.clone(),
                        })
                        .from_err()
                        .and_then(|res| res.and_then(|new_user| {
                            if let Some(user_id) = req_clone.identity() {
                                info!("signup request received while user '{}' logged in.  logging them out and logging in new user '{}'", user_id, new_user.user_id)
                            }
                            req_clone.forget();
                            req_clone.remember(new_user.user_id.to_string());
                            Ok(HttpResponse::Ok().finish())
                        }))
                        .wait()
                    } else {
                        Err(UserError::TakenUsername.into())
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
                res.and_then(|user| {
                    // we only get here if the authentication was successful.  otherwise an error
                    // is returned, and this `and_then` is skipped.
                    info!("user '{}' logged in", user.user_id);
                    req_clone.remember(user.user_id.to_string());
                    Ok(HttpResponse::Ok().finish())
                })
            })
        })
        .responder()
}

/// Log user out.
pub(crate) fn logout(req: &HttpRequest<AppState>) -> HttpResponse {
    if let Some(user_uuid) = req.identity() {
        info!("user '{}' logged out", user_uuid);
        req.forget();
        HttpResponse::Ok().finish()
    } else {
        CratifyError::from(UserError::SuperfluousLogout).error_response()
    }
}
