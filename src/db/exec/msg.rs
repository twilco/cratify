use crate::app::error::CratifyError;
use crate::db::model::User;
use ::actix::prelude::*;

pub(crate) struct CreateUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Message for CreateUser {
    type Result = Result<User, CratifyError>;
}

pub(crate) struct IsUsernameAvailable {
    pub(crate) username: String,
}

impl Message for IsUsernameAvailable {
    type Result = Result<bool, CratifyError>;
}

pub(crate) struct AreCredentialsValid {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Message for AreCredentialsValid {
    type Result = Result<bool, CratifyError>;
}
