FROM rust:1.58.0 as build

# create a new empty shell project
RUN USER=root cargo new --bin fundamentals
WORKDIR /fundamentals

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/fundamentals*
RUN cargo build --release

# our final base
FROM rust:1.58.0

# copy the build artifact from the build stage
COPY --from=build /fundamentals/target/release/fundamentals .

# set the startup command to run your binary
CMD ["./fundamentals"]