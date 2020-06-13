# Build the code first
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /multiverse

RUN apt-get update && apt-get install -y libclang-dev build-essential clang libssl-dev pkg-config

RUN rustup default nightly-2020-06-09

# We do a non-code build to get all the dependencies. This allows us to benefit from layer caching
COPY Cargo.lock Cargo.toml docker/pre-build.sh /multiverse/
RUN ./pre-build.sh
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /multiverse/target -name "*multiverse*" | xargs rm -rf

# Finally, we do a full build of our code
RUN rm -rf src
COPY migrations /multiverse/migrations/
COPY src /multiverse/src/
RUN cargo build --release


# Next build a container with the build artifact but no code
FROM bitnami/minideb:buster

WORKDIR /multiverse

RUN apt-get update && apt-get install -y curl

ENV PORT=8000
ENV DATABASE_URL=

COPY --from=builder /multiverse/target/release/multiverse /multiverse/multiverse

RUN /multiverse/multiverse test

CMD ["/multiverse/multiverse"]

HEALTHCHECK CMD curl --fail http://localhost:$PORT/health || exit 1
