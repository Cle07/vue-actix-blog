FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y \
    lua5.4 \
    liblua5.4-dev \
    pkg-config \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin vue-actix

# Frontend builder stage
FROM debian:bookworm-slim AS frontend-builder
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    && rm -rf /var/lib/apt/lists/*
# Install Bun
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:$PATH"
WORKDIR /app
COPY frontend/ ./frontend/
WORKDIR /app/frontend
RUN bun run build

# Runtime stage
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y \
    lua5.4 \
    liblua5.4-dev \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
# Set environment variables for binding to all interfaces
ENV HOST=0.0.0.0
ENV PORT=8080
COPY --from=builder /app/target/release/vue-actix /usr/local/bin
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist
COPY articles/ ./articles/
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/vue-actix"]
