# Build context is the repo root
# Deploy: git archive --format=tar.gz -o /tmp/deploy.tar.gz HEAD

# Stage 1: Build all Rust artifacts (WASM + backend binary)
FROM rust:1.88-bookworm AS rust-builder
RUN cargo install wasm-pack --locked
WORKDIR /app
COPY playground/Cargo.toml playground/Cargo.lock ./
COPY playground/wasm/ wasm/
COPY playground/backend/ backend/
RUN cd wasm && wasm-pack build --target web --release --out-dir /app/wasm-pkg
RUN cargo build --release -p solscript-playground

# Stage 2: Build Vue frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app
COPY playground/frontend/package*.json ./
RUN npm ci
COPY playground/frontend/ ./
COPY --from=rust-builder /app/wasm-pkg/ ./src/wasm-pkg/
RUN npm run build

# Stage 3: Build Astro site
FROM node:20-alpine AS astro-builder
WORKDIR /app
COPY website/package*.json ./
RUN npm ci
COPY website/ ./
RUN npm run build

# Stage 4: Runtime (lean — no Solana toolchain yet)
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=rust-builder /app/target/release/solscript-playground /usr/local/bin/
COPY --from=frontend-builder /app/dist/ /app/playground/
COPY --from=astro-builder /app/dist/ /app/astro/

ENV PLAYGROUND_DIR=/app/playground
ENV ASTRO_DIR=/app/astro
ENV PORT=3000
ENV RUST_LOG=info
EXPOSE 3000
CMD ["solscript-playground"]
