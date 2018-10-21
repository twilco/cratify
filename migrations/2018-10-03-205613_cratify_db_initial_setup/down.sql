/* Drop things in the opposite order that they were created. */

DROP TABLE IF EXISTS sent_means;
DROP TYPE IF EXISTS means_of_transportation;

DROP TABLE IF EXISTS sent_versions;
DROP TABLE IF EXISTS sent;
DROP TABLE IF EXISTS periodic_subscriptions;
DROP TYPE IF EXISTS periodic_subscription_frequencies;
DROP TABLE IF EXISTS subscriptions;
DROP TYPE IF EXISTS subscription_type;
DROP TABLE IF EXISTS secure_tokens;
DROP TABLE IF EXISTS emails;
DROP TABLE IF EXISTS users;

DROP EXTENSION IF EXISTS dblink;
DROP EXTENSION IF EXISTS pgcrypto;

