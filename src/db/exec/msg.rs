use crate::db::models::{NewUser, User};
use ::actix::prelude::*;
use failure::Error;

pub(crate) struct CreateUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

pub(crate) struct IsUsernameAvailable {
    pub(crate) username: String,
}

impl Message for IsUsernameAvailable {
    type Result = Result<bool, Error>;
}
