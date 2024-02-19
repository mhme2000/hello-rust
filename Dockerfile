# Rust as the base image
FROM rust:1.76-slim-buster as build
    
# 1. Create a new empty shell project
RUN USER=root cargo new --bin rinha
WORKDIR /rinha

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rinha*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /rinha/target/release/rinha .

CMD ["./rinha"]