# ==============================================================================
# Base Stage
# ==============================================================================

ARG RUST_VERSION=1.97
FROM rust:${RUST_VERSION}-slim AS base

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ==============================================================================
# Development Stage
# ==============================================================================

ARG RUST_VERSION=1.97
FROM base AS dev

RUN rustup component add rustfmt clippy

RUN apt-get update && apt-get install -y --no-install-recommends \
	netcat-openbsd \
	xz-utils \
	gosu \
	&& rm -rf /var/lib/apt/lists/*

ADD --chmod=755 \
	https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-gnu.tar.xz \
	/tmp/cargo-watch.tar.xz

RUN tar -xf /tmp/cargo-watch.tar.xz -C /usr/local/bin --strip-components=1 \
	&& rm /tmp/cargo-watch.tar.xz

ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID appuser \
	&& useradd -m -u $UID -g appuser appuser

CMD ["/bin/bash", "/app/apps/ldapi/config/scripts/entrypoint.sh"]

# ==============================================================================
# Build Stage
# ==============================================================================

ARG RUST_VERSION=1.97
FROM base AS builder

COPY . .

RUN cargo build --release -p ldapi --bin ldapi

# ==============================================================================
# Production Stage
# ==============================================================================

FROM debian:trixie-slim AS prod

RUN apt-get update && apt-get install -y --no-install-recommends \
	ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -m -U appuser
USER appuser

COPY --from=builder /app/target/release/ldapi ./server
COPY --from=builder /app/apps/ldapi/config ./config

CMD ["./server"]
