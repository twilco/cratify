/// The DAO for all our models.

use crate::db::schema;
use crate::db::models::*;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub fn create_user<'a>(conn: &PgConnection, email: &'a str, hashed_password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        email,
        hashed_password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("error saving new user")
}
