# 1. This tells docker to use the Rust official image
FROM rust:1.64

RUN groupadd docker && useradd -g docker dodo

# create a new empty shell project
RUN USER=root cargo new --bin wik_dps_tp01
WORKDIR /wik_dps_tp01

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/wik_dps_tp01*
RUN cargo build --release

USER dodo
# set the startup command to run your binary
CMD ["./target/release/wik_dps_tp01"]