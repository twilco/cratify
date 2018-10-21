CREATE EXTENSION IF NOT EXISTS dblink;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS users (
	user_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	hashed_password text NOT NULL
);

CREATE TABLE IF NOT EXISTS emails (
  email_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(user_id),
  verified boolean NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS secure_tokens (
  secure_token_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
  email_id uuid NOT NULL REFERENCES emails(email_id),
  expiration timestamp NOT NULL,
  val text NOT NULL
);

DO $$ BEGIN
    CREATE TYPE subscription_type AS ENUM ('immediate', 'periodic');
EXCEPTION
    WHEN duplicate_object THEN RAISE NOTICE 'subscription_type type already exists - skipping.';
END $$;
CREATE TABLE IF NOT EXISTS subscriptions (
	subscription_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	email_id uuid NOT NULL REFERENCES emails(email_id),
	crate_name text NOT NULL,
	subscription_type subscription_type NOT NULL
);

DO $$ BEGIN
    CREATE TYPE periodic_subscription_frequencies AS ENUM ('weekly', 'monthly', 'yearly');
EXCEPTION
    WHEN duplicate_object THEN RAISE NOTICE 'periodic_subscription_frequencies type already exists - skipping.';
END $$;
CREATE TABLE IF NOT EXISTS periodic_subscriptions (
	periodic_subscription_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	subscription_id uuid NOT NULL REFERENCES subscriptions(subscription_id),
	frequency periodic_subscription_frequencies NOT NULL
);

CREATE TABLE IF NOT EXISTS sent (
	sent_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	user_id uuid NOT NULL REFERENCES users(user_id),
	subscription_id uuid NOT NULL REFERENCES subscriptions(subscription_id),
	sent_date_time timestamp NOT NULL
);

CREATE TABLE IF NOT EXISTS sent_versions (
	sent_version_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	sent_id uuid NOT NULL REFERENCES sent(sent_id),
	crate_name text NOT NULL,
	crate_version text NOT NULL
);

DO $$ BEGIN
    CREATE TYPE means_of_transportation AS ENUM ('email');
EXCEPTION
    WHEN duplicate_object THEN RAISE NOTICE 'means_of_transportation type already exists - skipping.';
END $$;
CREATE TABLE IF NOT EXISTS sent_means (
	sent_means_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	email_id uuid NOT NULL REFERENCES emails(email_id),
	means means_of_transportation NOT NULL
);