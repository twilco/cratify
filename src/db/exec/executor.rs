use crate::app::error::CratifyError;
use crate::app::error::DbError;
use crate::app::error::UserError;
use crate::db::exec::msg::*;
use crate::db::model::{NewUser, User};
use crate::db::schema::users;
use ::actix::prelude::*;
use ::actix::Handler;
use bcrypt::*;
use diesel::dsl::{exists, select};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub(crate) struct DbExecutor(pub(crate) Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<AreCredentialsValid> for DbExecutor {
    type Result = Result<User, CratifyError>;

    /// If credentials are valid return our User.  Otherwise, return either the
    /// [UserError::InvalidCredentials](enum.UserError.html) into a [CratifyError](enum.CratifyError.html)
    /// or some other, likely unexpected, error into a [CratifyError](enum.CratifyError.html).
    fn handle(&mut self, msg: AreCredentialsValid, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        match users.filter(username.eq(msg.username)).first::<User>(
            &self
                .0
                .get()
                .map_err(|e| {
                    return DbError::ConnectionPool {
                        cause: e,
                        occurred_when: "retrieving user to validate credentials".to_owned(),
                    };
                })
                .unwrap(),
        ) {
            Ok(user) => {
                match verify(&msg.password, &user.hashed_password) {
                    Ok(matches) => {
                        if matches {
                            return Ok(user);
                        } else {
                            return Err(UserError::InvalidCredentials.into());
                        }
                    }
                    Err(e) => return Err(CratifyError::EncryptionError { inner: e }),
                };
            }
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    return Err(UserError::InvalidCredentials.into());
                }
                _ => {
                    return Err(DbError::ResultRetrieval {
                        cause: err,
                        occurred_when: "retrieving user to validate credentials".to_owned(),
                    }
                    .into());
                }
            },
        }
    }
}

impl Handler<CreateUser> for DbExecutor {
    type Result = Result<User, CratifyError>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        if msg.username.is_empty() {
            return Err(UserError::EmptyUsername.into());
        } else if msg.password.is_empty() {
            return Err(UserError::EmptyPassword.into());
        }

        let hashed_password = hash(&msg.password, DEFAULT_COST)?;
        let new_user = NewUser {
            username: &msg.username,
            hashed_password: &hashed_password,
        };

        let new_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(
                &self
                    .0
                    .get()
                    .map_err(|e| {
                        return Err::<User, CratifyError>(
                            DbError::ConnectionPool {
                                cause: e,
                                occurred_when: "getting conn for user creation".to_owned(),
                            }
                            .into(),
                        );
                    })
                    .unwrap(),
            )
            .map_err(|e| {
                return Err::<User, CratifyError>(
                    DbError::ResultRetrieval {
                        cause: e,
                        occurred_when: "creating user".to_owned(),
                    }
                    .into(),
                );
            })
            .unwrap();

        Ok(new_user)
    }
}

impl Handler<IsUsernameAvailable> for DbExecutor {
    type Result = Result<bool, CratifyError>;

    fn handle(&mut self, msg: IsUsernameAvailable, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        let username_exists: bool = select(exists(users.filter(username.eq(msg.username))))
            .first(
                &self
                    .0
                    .get()
                    .map_err(|e| {
                        return DbError::ConnectionPool {
                            cause: e,
                            occurred_when: "getting conn for username existence query".to_owned(),
                        };
                    })
                    .unwrap(),
            )
            .map_err(|e| {
                return DbError::ResultRetrieval {
                    cause: e,
                    occurred_when: "determining existence of username".to_owned(),
                };
            })
            .unwrap();

        Ok(!username_exists)
    }
}
