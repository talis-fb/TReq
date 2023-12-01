# ------------
# Build
# ------------
FROM rust:1-bullseye AS BUILD

WORKDIR /app

RUN cargo init
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

COPY src/ src/
COPY tests/ tests/

RUN cargo build --release

# ------------
# Runner
# ------------
FROM debian:bullseye AS RUNNER

COPY --from=BUILD /app/target/release/treq /usr/local/bin/treq

RUN chmod +x /usr/local/bin/treq

RUN useradd -ms /bin/bash wizard
USER wizard
WORKDIR /home/wizard

CMD ["/bin/bash"]
