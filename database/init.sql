/* To be run automtically by the `docker-compose`d Postgres database. */

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