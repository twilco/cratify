table! {
    periodic_subscriptions (periodic_subscription_id) {
        periodic_subscription_id -> Uuid,
        subscription_id -> Uuid,
        frequency -> Periodic_subscription_frequencies,
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
    sent_means (sent_means_id) {
        sent_means_id -> Uuid,
        user_id -> Uuid,
        means -> Means_of_transportation,
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
    subscriptions (subscription_id) {
        subscription_id -> Uuid,
        user_id -> Uuid,
        crate_name -> Text,
        subscription_type -> Subscription_type,
    }
}

table! {
    users (user_id) {
        user_id -> Uuid,
        email_addr -> Text,
        hashed_password -> Text,
    }
}

joinable!(periodic_subscriptions -> subscriptions (subscription_id));
joinable!(sent -> subscriptions (subscription_id));
joinable!(sent -> users (user_id));
joinable!(sent_means -> users (user_id));
joinable!(sent_versions -> sent (sent_id));
joinable!(subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    periodic_subscriptions,
    sent,
    sent_means,
    sent_versions,
    subscriptions,
    users,
);
