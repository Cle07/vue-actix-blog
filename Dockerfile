# syntax=docker/dockerfile:1

# Keep the build environment's Debian/glibc generation aligned with runtime.
# Avoid `latest-rust-1`, which can silently change its underlying OS.
FROM lukemathwalker/cargo-chef:latest-rust-1-bookworm AS chef

WORKDIR /app

# Generate a dependency-only Cargo recipe for Docker-layer caching.
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build the Rust backend.
FROM chef AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    lua5.4 \
    liblua5.4-dev \
    pkg-config \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json ./recipe.json

# Cached layer: only reruns when Cargo dependency metadata changes.
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --locked --bin vue-actix

# Build the Vue frontend.
FROM debian:bookworm-slim AS frontend-builder

ENV DEBIAN_FRONTEND=noninteractive
ENV BUN_INSTALL=/opt/bun
ENV PATH="${BUN_INSTALL}/bin:${PATH}"

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Install Bun without relying on an invalid Markdown URL.
RUN curl -fsSL https://bun.sh/install | bash

WORKDIR /app/frontend

# Copy manifests separately so Bun dependency installation can cache.
COPY frontend/package.json frontend/bun.lockb* frontend/bun.lock* ./
RUN bun install --frozen-lockfile

COPY frontend/ ./
RUN bun run build

# Production runtime: same Debian Bookworm family as the Rust builder.
FROM debian:bookworm-slim AS runtime

ENV DEBIAN_FRONTEND=noninteractive \
    HOST=0.0.0.0 \
    PORT=8080

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    lua5.4 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/vue-actix /usr/local/bin/vue-actix
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist
COPY articles/ ./articles/

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/vue-actix"]