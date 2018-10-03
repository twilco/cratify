# Developer's guide to the galaxy (or maybe just Cratify...)

There are a lot of moving pieces that come together to make Cratify work.  This document will cover various aspects of development on Cratify, including setup, build, and run details, database management, interesting specifics about our Docker and Docker Compose workflow, and likely much more.

## Project setup

TODO

## How to run

TODO

## Diesel and database migrations



Cratify makes use of Postgres as its database, and Diesel as its ORM.  We also utilize Diesel's migrations, which run automatically on app startup. These migrations give us the ability to safely transform the database schema as the application evolves.  You can find our migrations [here](/migrations) in the root directory of the project.  If you want to know more about how this works, Diesel's documentation is excellent and can explain it much better than I can - [check it out](http://diesel.rs/guides/getting-started/).

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

## Docker

The [Dockerfile](/Dockerfile) itself is well documented, so if you want to know more about how that is built check it out.



