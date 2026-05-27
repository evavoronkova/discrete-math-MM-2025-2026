FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libfontconfig-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

COPY NetAnalisys/Cargo.toml NetAnalisys/Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo fetch

RUN cargo build --release --locked && \
    rm -f src/main.rs target/release/deps/NetAnalisys*

COPY NetAnalisys/ ./

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r netanalisys && useradd -r -g netanalisys -d /app -s /sbin/nologin netanalisys

WORKDIR /app

COPY --from=builder /build/target/release/NetAnalisys ./

USER netanalisys

CMD ["./NetAnalisys"]
