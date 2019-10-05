# select build image
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin rocket-server
WORKDIR /rocket-server

# copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/rocket-server*
RUN cargo build --release

# our final base
FROM alpine:latest

WORKDIR /rocket-server

# copy the build artifact from the build stage
COPY --from=build /rocket-server/target/release/rocket-server .

# set the startup command to run your binary
ENTRYPOINT ["./rocket-server"]