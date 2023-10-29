
FROM rustlang/rust:nightly-bullseye as builder

RUN cargo install --locked cargo-leptos

RUN rustup target add wasm32-unknown-unknown

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo update
RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/public /app/public

COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/target/server/release/phase-alpha-site /app/site/
COPY --from=builder /app/Cargo.toml /app/

WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="phase-alpha-site"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="127.0.0.1:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD [ "/app/site/phase-alpha-site" ]
