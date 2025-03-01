# docker build --build-arg pkg=cargo_package_name -t app  .

# FROM rust:1.85.0
FROM docker.io/rust:1.85.0 AS build

## cargo package name: customize here or provide via --build-arg
ARG pkg=rocket-app

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

################################################################################

FROM docker.io/debian:bookworm-slim

WORKDIR /app

## copy the main binary
COPY --from=build /build/main ./

## copy runtime assets which may or may not exist
COPY --from=build /build/Rocket.tom[l] ./static
COPY --from=build /build/stati[c] ./static
COPY --from=build /build/template[s] ./templates

## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

EXPOSE 8080

CMD ./main

# RUN rustup default nightly
# RUN cargo install cargo-watch

# CMD ["sh", "-c", "cargo watch -x run > cargo.log 2>&1"]

