# https://docs.docker.com/develop/develop-images/multistage-build/
# use nightly as our build image in this multistage build.
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin cratify
WORKDIR /cratify

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# purge curl and libcurl to get the curl-sys crate (which is depended on by our dependencies) to use the bundled curl
# rather than the system curl.  without this, we get lots of curl compatibility issues
RUN apt-get purge curl 'libcurl*' -y

# this build step will cache our dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy our source tree
COPY ./src ./src

# and our migrations
COPY ./migrations ./migrations

# touch the real main.rs to prevent Docker from using the one created in `cargo new --bin cratify`
RUN touch src/main.rs

# build for release
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates libcurl4-openssl-dev openssl libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

# we've installed all the things we need, so we can now remove this to save some space
RUN rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /cratify/target/release/cratify .

# by default, rocket runs on 8000.  let's expose that port
EXPOSE 8000

# by default, rocket runs on localhost.  localhost in a docker container means nothing outside the container can connect
# to it, which is not helpful in a web application.  0.0.0.0 does not have the same problem, so have rocket run on that
# address instead.
ENV ROCKET_ADDRESS 0.0.0.0
# set the startup command to run our binary
CMD ["./cratify"]