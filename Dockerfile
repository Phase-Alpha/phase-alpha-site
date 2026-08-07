# Stage 1: Build
FROM rustlang/rust:nightly-bookworm AS builder

# Pin to specific nightly version for Leptos 0.8 compatibility
RUN rustup default nightly-2025-08-31

# Install cargo-binstall (ARM64 version) for easier cargo-leptos installation
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-aarch64-unknown-linux-musl.tgz && \
    tar -xvf cargo-binstall-aarch64-unknown-linux-musl.tgz && \
    cp cargo-binstall /usr/local/cargo/bin && \
    rm cargo-binstall-aarch64-unknown-linux-musl.tgz

# Install required tools and Sass compiler
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang wget curl \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/* \
  && wget https://github.com/sass/dart-sass/releases/download/1.86.0/dart-sass-1.86.0-linux-arm64.tar.gz \
  && tar -xzf dart-sass-1.86.0-linux-arm64.tar.gz \
  && cp -r dart-sass/* /usr/local/bin/ \
  && rm -rf dart-sass-1.86.0-linux-arm64.tar.gz dart-sass

# Install cargo-leptos using pre-built binary
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Create app directory and set it as the working directory
RUN mkdir -p /app
WORKDIR /app

# Copy dependency files first (for dependency caching)
COPY Cargo.toml Cargo.lock* rust-toolchain.toml ./

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
FROM debian:bookworm-slim

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
