table! {
    email_fulfillments (email_fulfillment_id) {
        email_fulfillment_id -> Uuid,
        fulfillment_id -> Uuid,
        email_id -> Uuid,
    }
}

table! {
    emails (email_id) {
        email_id -> Uuid,
        user_id -> Uuid,
        verified -> Bool,
    }
}

table! {
    fulfillments (fulfillment_id) {
        fulfillment_id -> Uuid,
        subscription_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::Uuid;
    use crate::db::models::PeriodicSubscriptionFrequencyMapping;
    periodic_subscriptions (periodic_subscription_id) {
        periodic_subscription_id -> Uuid,
        subscription_id -> Uuid,
        frequency -> PeriodicSubscriptionFrequencyMapping,
    }
}

table! {
    secure_tokens (secure_token_id) {
        secure_token_id -> Uuid,
        email_id -> Uuid,
        expiration -> Timestamp,
        val -> Text,
    }
}

table! {
    sent (sent_id) {
        sent_id -> Uuid,
        user_id -> Uuid,
        subscription_id -> Uuid,
        sent_date_time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::Uuid;
    use crate::db::models::MeansOfTransportationMapping;
    sent_means (sent_means_id) {
        sent_means_id -> Uuid,
        email_id -> Uuid,
        means -> MeansOfTransportationMapping,
    }
}

table! {
    sent_versions (sent_version_id) {
        sent_version_id -> Uuid,
        sent_id -> Uuid,
        crate_name -> Text,
        crate_version -> Text,
    }
}

table! {
    use diesel::sql_types::Uuid;
    use diesel::sql_types::Text;
    use crate::db::models::SubscriptionTypeMapping;
    subscriptions (subscription_id) {
        subscription_id -> Uuid,
        user_id -> Uuid,
        crate_name -> Text,
        crate_version -> Text,
        subscription_type -> SubscriptionTypeMapping,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Text,
        hashed_password -> Text,
    }
}

joinable!(email_fulfillments -> emails (email_id));
joinable!(email_fulfillments -> fulfillments (fulfillment_id));
joinable!(emails -> users (user_id));
joinable!(fulfillments -> subscriptions (subscription_id));
joinable!(periodic_subscriptions -> subscriptions (subscription_id));
joinable!(secure_tokens -> emails (email_id));
joinable!(sent -> subscriptions (subscription_id));
joinable!(sent -> users (user_id));
joinable!(sent_means -> emails (email_id));
joinable!(sent_versions -> sent (sent_id));
joinable!(subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    email_fulfillments,
    emails,
    fulfillments,
    periodic_subscriptions,
    secure_tokens,
    sent,
    sent_means,
    sent_versions,
    subscriptions,
    users,
);
