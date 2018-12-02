use crate::db::exec::msg::*;
use crate::db::models::{NewUser, User};
use ::actix::prelude::*;
use ::actix::Handler;
use bcrypt::*;
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use failure::{bail, Error};
use r2d2::Pool;

pub(crate) struct DbExecutor(pub(crate) Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let hashed_password = hash(&msg.password, DEFAULT_COST)?;
        let new_user = NewUser {
            username: &msg.username,
            hashed_password: &hashed_password,
        };

        use crate::db::schema::users;
        let new_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&self.0.get()?)
            .expect("error saving new user");

        Ok(new_user)
    }
}

impl Handler<IsUsernameAvailable> for DbExecutor {
    type Result = Result<bool, Error>;

    fn handle(&mut self, msg: IsUsernameAvailable, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users;
        use crate::db::schema::users::dsl::*;
        // TODO: This feels like a super awkward/verbose way to determine if the specified username
        //       exists...there must be a better way, but I haven't been able to find it.
        let mut username_exists_vec: Vec<bool> =
            select(exists(users.filter(username.eq(msg.username))))
                .get_results(&self.0.get()?)
                .expect("couldn't determine if username exists");

        match username_exists_vec.len() {
            n if n < 1 => bail!("zero results from username exists query"),
            _ => Ok(!username_exists_vec.remove(0)), // position zero should have our 'exists' bool
        }
    }
}
