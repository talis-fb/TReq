# ------------
# Build
# ------------
FROM rust:1-bullseye AS BUILD

WORKDIR /app

RUN apt-get update && apt-get install -y musl musl-tools
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo init
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY src/ src/
COPY tests/ tests/

RUN cargo build --release --target x86_64-unknown-linux-musl

# ------------
# Runner
# ------------
FROM alpine:latest AS RUNNER

RUN apk --no-cache add fish git httpie shadow

COPY --from=BUILD /app/target/x86_64-unknown-linux-musl/release/treq /usr/local/bin/treq

RUN chmod +x /usr/local/bin/treq

RUN adduser -D -u 1000 wizard
USER wizard
RUN chsh -s /usr/bin/fish wizard

WORKDIR /home/wizard
