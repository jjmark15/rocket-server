# select build image
FROM rustlang/rust:nightly as build

# create a new empty shell project
RUN USER=root cargo new --bin rocket-server
WORKDIR /rocket-server

# copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/rocket_service
RUN cargo build --release --offline

# our final base
FROM debian:stable-slim

WORKDIR /rocket-server

# copy the build artifact from the build stage
COPY --from=build /rocket-server/target/release/rocket_service .

# set the startup command to run your binary
ENTRYPOINT ["./rocket_service"]

EXPOSE 8000