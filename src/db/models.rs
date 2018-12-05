use crate::db::schema::*;

use chrono::prelude::*;
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(Clone, Debug, DbEnum, Eq, PartialEq)]
pub enum PeriodicSubscriptionFrequency {
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Clone, Debug, DbEnum, Eq, PartialEq)]
pub enum SubscriptionType {
    Immediate,
    Periodic,
}

#[derive(Clone, Debug, DbEnum, Eq, PartialEq)]
pub enum MeansOfTransportation {
    Email,
}

#[derive(Debug, Eq, Identifiable, PartialEq, Queryable)]
#[primary_key(user_id)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub hashed_password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub hashed_password: &'a str,
}

#[derive(Debug, Eq, PartialEq, Queryable)]
pub struct Subscription {
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub crate_name: String,
    pub subscription_type: SubscriptionType,
}

#[derive(Debug, Eq, PartialEq, Queryable)]
pub struct PeriodicSubscription {
    pub periodic_subscription_id: Uuid,
    pub subscription_id: Uuid,
    pub frequency: PeriodicSubscriptionFrequency,
}

// TODO: Remove this - adding it now just to suppress the annoying warning.
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Queryable)]
#[table_name = "sent"]
pub struct Sent {
    pub sent_id: Uuid,
    pub user_id: Uuid,
    pub subscription_id: Uuid,
    pub sent_date_time: NaiveDateTime,
}

#[derive(Debug, Eq, PartialEq, Queryable)]
pub struct SentVersion {
    pub sent_version_id: Uuid,
    pub sent_id: Uuid,
    pub crate_name: String,
    pub crate_version: String,
}

// TODO: Remove this - adding it now just to suppress the annoying warning.
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Queryable)]
#[table_name = "sent_means"]
pub struct SentMeans {
    pub sent_means_id: Uuid,
    pub user_id: Uuid,
    pub means: MeansOfTransportation,
}
