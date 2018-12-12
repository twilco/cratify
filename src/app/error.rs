use actix::MailboxError;
use actix_web::error::{JsonPayloadError, ResponseError};
use actix_web::HttpResponse;
use bcrypt::BcryptError;
use failure::Fail;

/// General application errors that can be converted to a user-facing error.  Users will see these
/// messages, so ensure that no implementation details or otherwise sensitive information is leaked
/// in these errors or in any contributed error messages.
#[derive(Debug, Fail)]
pub(crate) enum CratifyError {
    #[fail(display = "Interval server error occurred.")]
    ActixError { inner: String },

    #[fail(display = "Interval server error occurred.")]
    DbError { inner: DbError },

    #[fail(display = "Internal server error occurred.")]
    EncryptionError { inner: BcryptError },

    #[fail(display = "{}", message)]
    ValidationError { message: String },
}

impl From<JsonPayloadError> for CratifyError {
    fn from(err: JsonPayloadError) -> Self {
        CratifyError::ActixError {
            inner: err.to_string(),
        }
    }
}

impl From<MailboxError> for CratifyError {
    fn from(err: MailboxError) -> Self {
        CratifyError::ActixError {
            inner: err.to_string(),
        }
    }
}

impl From<DbError> for CratifyError {
    fn from(err: DbError) -> Self {
        CratifyError::DbError { inner: err }
    }
}

impl From<ValidationError> for CratifyError {
    fn from(err: ValidationError) -> Self {
        CratifyError::ValidationError {
            message: err.to_string(),
        }
    }
}

impl From<BcryptError> for CratifyError {
    fn from(err: BcryptError) -> Self {
        CratifyError::EncryptionError { inner: err }
    }
}

impl ResponseError for CratifyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CratifyError::ActixError { inner } => {
                error!("{}", inner);
                HttpResponse::InternalServerError()
            }
            CratifyError::DbError { inner } => {
                error!("{}", inner);
                HttpResponse::InternalServerError()
            }
            CratifyError::EncryptionError { inner } => {
                error!("{}", inner);
                HttpResponse::InternalServerError()
            }
            CratifyError::ValidationError { .. } => {
                // All request errors are autologged, and there is no additional "inner" information
                // that would be helpful to know here, so no more logging required.
                HttpResponse::BadRequest()
            }
        }
        .body(ErrorPayload::from_error(self).to_json())
    }
}

#[derive(Debug, Fail)]
pub(crate) enum DbError {
    #[fail(
        display = "there was a connection pool error - occurred when {}.  cause: {:?}",
        occurred_when, cause
    )]
    ConnectionPool {
        #[fail(cause)]
        cause: r2d2::Error,
        occurred_when: String,
    },

    #[fail(
        display = "error retrieving results of a query - {}.  cause: {:?}",
        occurred_when, cause
    )]
    ResultRetrieval {
        #[fail(cause)]
        cause: diesel::result::Error,
        occurred_when: String,
    },
}

/// JSON error payload.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct ErrorPayload {
    message: String,
}

impl ErrorPayload {
    pub fn from_error(e: &CratifyError) -> Self {
        Self {
            message: e.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Debug, Clone, Fail)]
pub(crate) enum ValidationError {
    #[fail(display = "empty username given")]
    EmptyUsername,

    #[fail(display = "empty password given")]
    EmptyPassword,

    #[fail(display = "invalid credentials provided")]
    InvalidCredentials,

    #[fail(display = "username already taken")]
    TakenUsername,
}
