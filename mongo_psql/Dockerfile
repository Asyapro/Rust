FROM rust AS builder

# copy over your manifests

# this build step will cache your dependencies

# copy your source tree
COPY ./Cargo.toml /books/Cargo.toml
COPY ./actix_web_dev /books/actix_web_dev
COPY ./src /books/src
WORKDIR /books
# build for release
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && \
    apt-get --assume-yes install \
        make \
        libpq5 \
        libpq-dev \
        -qqy \
        --no-install-recommends
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /books/target/release/books /books/books
COPY ./migrations ./books/migrations/
WORKDIR /books/
EXPOSE 8088


CMD ["/books/books"]
