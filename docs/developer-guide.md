# Developer's guide to the galaxy (or maybe just Cratify...)

There are a lot of moving pieces that come together to make Cratify work.  This document will cover various aspects of development on Cratify, including setup, build, run details, database management, interesting specifics about our Docker and Docker Compose workflow, and likely much more.

## Project setup

1. Cratify uses Diesel, so having the Diesel CLI installed is handy.  See installation instructions [here](https://github.com/diesel-rs/diesel/tree/master/diesel_cli#installation).
2. Ensure Docker and Docker Compose are installed.
3. Run `git clone git@github.com:twilco/cratify.git`

TODO

## How to run

1. Run `docker-compose build && docker-compose up` - this will start our application, reverse proxy, and database.
2. Go to <http://0.0.0.0:8080/dashboard/> to view your Traefik dashboard.  From there, you can see the URL your app is being served at (probably [cratify.app.localhost]()).
3. When finished, run `docker-compose down`.

TODO

## Environments and deployment

You can run cratify locally without Docker (although you will need to figure out the database on your own), locally with Docker and Docker Compose, and in production with Docker and Docker Compose.

Regardless of environment, Cratify expects certain environment variable to be present (such as `CRATIFY_DATABASE_URL`).  

When running locally without Docker, these should be set in `local.env`, which is checked into git.  When running locally with Docker, those environment variables should be set in `docker-local.env`, which is also checked into git.  When running in production with Docker and Docker Compose, environment variables should be set in `docker-prod.env`.  For this file, I have checked a `prod.env.template` file into git.  Update the environment variables, change the name to `docker-prod.env`, and you should be all set.  Docker Compose takes care of injecting these environment variables into our container for us, so our app has no trouble getting at them.

## Diesel and database migrations

Cratify makes use of [Postgres](https://www.postgresql.org/) as it's database, [Diesel](http://diesel.rs/) as it's ORM, and [R2D2](https://github.com/sfackler/r2d2) as it's database connection pooling mechanism.  We also utilize Diesel's migrations, which run automatically on app startup. These migrations give us the ability to safely transform the database schema as the application evolves.  You can find our migrations [here](/migrations) in the root directory of the project.  If you want to know more about how this works, Diesel's documentation is excellent and can explain it much better than I can - [check it out](http://diesel.rs/guides/getting-started/).

Something interesting to note is the [database/init.sql](/database/init.sql) file.  What is this for, exactly?  Well, the script itself is pretty simple - it just creates a `cratify` database for us if it doesn't already exist...it's how it's used that is interesting.  In official Postgres Docker images, any `.sql` file in the `/docker-entrypoint-initdb.d/` folder will automatically be run on container startup.  This is really handy, because the way we use Diesel assumes that the database has already been created.  By specifiying this `init.sql` as a volume for our Postgres container, we can map this file into the container and ensure we have a database to use when we `docker-compose up` our app!

```.yaml
db:
  image: postgres:11-alpine
  ports:
  # expose 5432 on host machine, and redirect that traffic to port 5432 in this container
    - "5432:5432"
  restart: always
  volumes:
    # ...and here we map our init.sql script to the place Postgres looks for it!
    - ./database/init.sql:/docker-entrypoint-initdb.d/init.sql
    - db_data:/var/lib/postgresql/data
``` 

Want more information?  [Check out this Stack Overflow post.](https://stackoverflow.com/questions/26598738/how-to-create-user-database-in-script-for-docker-postgres)

We use the [diesel-derive-enum](https://github.com/adwhit/diesel-derive-enum) crate to support Postgres enums, since Diesel [does not support it out of the box :(](https://github.com/diesel-rs/diesel/issues/343).  This requires manual modification of [src/schema.rs](), which is tricky because that is what Diesel migrations are supposed to automatically handle for us.  Any changes we make to `schema.rs` will automatically be wiped out by Diesel CLI.  We get around this by giving Diesel a [src/schema.patch]() file, so after it overwrites our schema file it automatically applies the diff we provide.  This works, but means we may need to sometimes update this patch file with [create-schema-patch.sh]().

## Docker

The [Dockerfile](/Dockerfile) itself is well documented, so if you want to know more about how that is built check it out.



