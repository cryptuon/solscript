# Build context is playground/ directory
# Deploy: git archive --format=tar.gz -o /tmp/deploy.tar.gz HEAD:playground

# Stage 1: Build all Rust artifacts (WASM + backend binary)
FROM rust:1.88-bookworm AS rust-builder
RUN cargo install wasm-pack --locked
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY wasm/ wasm/
COPY backend/ backend/
RUN cd wasm && wasm-pack build --target web --release --out-dir /app/wasm-pkg
RUN cargo build --release -p solscript-playground

# Stage 2: Build Vue frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
COPY --from=rust-builder /app/wasm-pkg/ ./src/wasm-pkg/
RUN npm run build

# Stage 3: Runtime (lean — no Solana toolchain yet)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /app/target/release/solscript-playground /usr/local/bin/
COPY --from=frontend-builder /app/dist/ /app/static/

ENV STATIC_DIR=/app/static
ENV PORT=3000
ENV RUST_LOG=info
EXPOSE 3000
CMD ["solscript-playground"]
