<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```

Then run
```bash
cargo leptos new --git leptos-rs/start-axum
```

to generate a new project template.

```bash
cd phase-alpha-site
```

to go to your newly created project.  
Feel free to explore the project structure, but the best place to start with your application code is in `src/app.rs`.  
Addtionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

## Configuration

The contact form is protected by [Cloudflare Turnstile](https://developers.cloudflare.com/turnstile/).
Create a widget in the Cloudflare dashboard to obtain a sitekey and a secret key, then set both
variables below. Note that they are consumed at different times.

### `TURNSTILE_SECRET_KEY` (runtime)

Read from `.env` by the server when it validates a submitted token against Cloudflare's siteverify
endpoint. This is a secret and is never sent to the browser.

```sh
TURNSTILE_SECRET_KEY="0x..."
```

### `TURNSTILE_SITE_KEY` (build time)

The sitekey is public. It is compiled into both the server binary and the WASM bundle, so it must be
set when building rather than when running. Baking it in this way guarantees the server-rendered
markup and the hydrated markup cannot disagree about which widget to render.

```bash
TURNSTILE_SITE_KEY="0x..." cargo leptos build --release
```

For the container image it is a build argument:

```bash
docker build --build-arg TURNSTILE_SITE_KEY="0x..." -t phase-alpha-site .
```

If it is left unset the build falls back to Cloudflare's test sitekey, `1x00000000000000000000AA`,
which always passes. That is convenient for local development but provides no protection, so make
sure a real sitekey is supplied for production builds. Cloudflare publishes a matching test secret
key, `1x0000000000000000000000000000000AA`, for local use.

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
phase-alpha-site
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="phase-alpha-site"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.
