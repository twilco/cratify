use crate::db::models::{NewUser, User};
use ::actix::prelude::*;
use ::actix::Handler;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use failure::Error;
use r2d2::Pool;

pub(crate) struct DbExecutor(pub(crate) Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub(crate) struct CreateUser {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Message for CreateUser {
    type Result = Result<User, Error>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let new_user = NewUser {
            username: &msg.username,
            hashed_password: &msg.password,
        };

        use crate::db::schema::users;
        let new_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&self.0.get()?)
            .expect("error saving new user");

        Ok(new_user)
    }
}
