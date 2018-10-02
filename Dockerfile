# select build image
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin cratify
WORKDIR /cratify

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# purge curl and libcurl to get curl-sys to use the bundled curl rather than the system curl.  without this, we get lots
# of curl compatibility issues
RUN apt-get purge curl 'libcurl*' -y

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# touch the real main.rs to prevent Docker from using the one created in `cargo new --bin cratify`
RUN touch src/main.rs

# build for release
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt-get update
RUN apt-get install -y --no-install-recommends ca-certificates libcurl4-openssl-dev openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# we've installed all the things we need, so we can now remove this to save some space
RUN rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /cratify/target/release/cratify .

EXPOSE 8000
ENV ROCKET_ADDRESS 0.0.0.0
# set the startup command to run your binary
CMD ["./cratify"]