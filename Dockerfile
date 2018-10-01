# select build image
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin cratify
WORKDIR /cratify

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

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
FROM debian:jessie-slim

# TODO: Eliminate this when it is certain it's not needed.  It would be ideal if we didn't have to make OpenSSL from scratch,
# but currently debian:buster-slim, which has the correct version of OpenSSL (1.1.0, not 1.0.0 in jessie) has some major
# libcurl 3 vs. 4 compatibility issues that I was not able to resolve.  https://github.com/curl/curl/issues/2433
# RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libcurl4-openssl-dev openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# Ideally this step would be much simpler, but due to the issues mentioned above we have to do things the hard way.
# TODO: Come back to this and see if we can find a way to do this more simply.

# our rust binary requires libcurl and libssl as shared libraries - let's grab and/or make them
RUN apt-get update
# libcurl here
RUN apt-get install -y libcurl4-openssl-dev
# and libssl next
RUN apt-get install -y wget
RUN apt-get install -y build-essential
RUN apt-get install -y zlib1g-dev
ARG OPENSSL_VERSION=1.1.0g
RUN wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz
RUN tar xvfz openssl-${OPENSSL_VERSION}.tar.gz
RUN cd openssl-${OPENSSL_VERSION} && ./config && make && make install
RUN echo '/usr/local/lib' >> /etc/ld.so.conf
RUN cat /etc/ld.so.conf
RUN ldconfig
RUN echo 'export LD_LIBRARY_PATH=/usr/local/lib' >> ~/.bash_profile && . ~/.bash_profile
RUN openssl version

# we've installed all the things we need, so we can now remove this to save some space
RUN rm -rf /var/lib/apt/lists/*

# copy the build artifact from the build stage
COPY --from=build /cratify/target/release/cratify .

# set the startup command to run your binary
CMD ["./cratify"]