use crate::schema::*;
use diesel::sql_types::Uuid;
use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug)]
pub enum PeriodicSubscriptionFrequencies {
    Weekly,
    Monthly,
    Yearly
}

#[derive(DbEnum, Debug)]
pub enum SubscriptionType {
    Immediate,
    Periodic
}

#[derive(DbEnum, Debug)]
pub enum MeansOfTransportation {
    Email
}

#[derive(Queryable)]
pub struct User {
    pub user_id: Uuid,
    pub email_addr: String,
    pub hashed_password: String,
}