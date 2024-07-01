# Stage 1: Build
FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-leptos and add wasm32 target
RUN cargo install --locked cargo-leptos && rustup target add wasm32-unknown-unknown

# Create app directory and set it as the working directory
RUN mkdir -p /app
WORKDIR /app

# Copy all files to the working directory
COPY . .

# Update cargo and build the project
RUN cargo update && cargo leptos build --release -vv

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
