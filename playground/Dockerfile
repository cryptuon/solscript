# Build context is playground/ directory

# Stage 1: Build WASM
FROM rust:1.88-bookworm AS wasm-builder
RUN cargo install wasm-pack --locked
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY wasm/ wasm/
COPY backend/ backend/
RUN cd wasm && wasm-pack build --target web --release --out-dir ../frontend/src/wasm-pkg

# Stage 2: Build Vue frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
COPY --from=wasm-builder /app/frontend/src/wasm-pkg/ ./src/wasm-pkg/
RUN npm run build

# Stage 3: Build Axum backend
FROM rust:1.88-bookworm AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY wasm/ wasm/
COPY backend/ backend/
RUN cargo build --release -p solscript-playground

# Stage 4: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)" && \
    echo 'export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"' >> /root/.bashrc
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

# Install Rust toolchain for cargo-build-sbf
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.88.0
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy backend binary
COPY --from=backend-builder /app/target/release/solscript-playground /usr/local/bin/

# Copy frontend static files
COPY --from=frontend-builder /app/dist/ /app/static/

ENV STATIC_DIR=/app/static
ENV PORT=3000
ENV RUST_LOG=info

EXPOSE 3000

CMD ["solscript-playground"]
