CREATE EXTENSION IF NOT EXISTS dblink;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

DO
$do$
BEGIN
   IF EXISTS (SELECT 1 FROM pg_database WHERE datname = 'cratify') THEN
      RAISE NOTICE 'Database already exists';
   ELSE
      PERFORM dblink_exec('dbname=' || current_database()  -- current db
                        , 'CREATE DATABASE cratify');
   END IF;
END
$do$;

CREATE TABLE IF NOT EXISTS users (
	user_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	email_addr text NOT NULL,
	hashed_password text NOT NULL
);

DO $$ BEGIN
    CREATE TYPE subscription_type AS ENUM ('immediate', 'periodic');
EXCEPTION
    WHEN duplicate_object THEN RAISE NOTICE 'subscription_type type already exists - skipping.';
END $$;
CREATE TABLE IF NOT EXISTS subscriptions (
	subscription_id uuid NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
	user_id uuid NOT NULL REFERENCES users(user_id),
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
	user_id uuid NOT NULL REFERENCES users(user_id),
	means means_of_transportation NOT NULL
);