# Stage 1: Build
FROM rustlang/rust:nightly-bookworm as builder

# Install cargo-leptos directly from pre-built ARM64 binary
RUN wget -qO- https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-aarch64-unknown-linux-musl.tar.gz | tar -xzv -C /usr/local/cargo/bin/

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Create app directory and set it as the working directory
RUN mkdir -p /app
WORKDIR /app

# Copy dependency files first (for dependency caching)
COPY Cargo.toml Cargo.lock* ./

# Create src directory and dummy files for dependency build
RUN mkdir -p src
RUN echo "fn main() {}" > src/main.rs
RUN echo "" > src/lib.rs

# Set environment variables for faster builds
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Build dependencies only (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release --bin phase-alpha-site --features ssr -j $(nproc)
RUN cargo build --release --lib --features hydrate --target wasm32-unknown-unknown -j $(nproc)

# Remove dummy source files
RUN rm -rf src

# Copy source code (src/, style/, public/ - changes here won't rebuild deps)
COPY src/ ./src/
COPY style/ ./style/
COPY public/ ./public/

# Copy posts separately (new blog posts will only rebuild from this point)
COPY posts/ ./posts/

# Copy remaining config files
COPY Cargo.toml ./

# Build the actual project (only rebuilds if source or posts changed)
RUN cargo leptos build --release

# Stage 2: Runner
FROM debian:bullseye-slim

# Copy the necessary files from the builder stage to the runner stage
COPY --from=builder /app/posts /app/posts
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/target/release/phase-alpha-site /app/phase-alpha-site
COPY --from=builder /app/Cargo.toml /app/Cargo.toml

# Set the working directory
WORKDIR /app

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="phase-alpha-site"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_RELOAD_PORT="3001"

# Expose the necessary port
EXPOSE 3000

# Run the application
CMD [ "/app/phase-alpha-site" ]
