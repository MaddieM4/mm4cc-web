FROM rust:1.76-bookworm AS builder

WORKDIR /app/
COPY Cargo.toml .
COPY Cargo.lock .

# Dummy system to allow Docker caching
# Original idea: https://stackoverflow.com/a/49664709
RUN mkdir src \
  && echo "fn main() {}" > src/main.rs \
  && cargo build --release

# Now build the real app
RUN rm -r src target/release/mm4cc-web
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release # /app/target/release/mm4cc-web

FROM debian:bookworm-slim
RUN apt-get update \
  && apt-get upgrade -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mm4cc-web /app/mm4cc-web

WORKDIR /app/
CMD ["/app/mm4cc-web"]
