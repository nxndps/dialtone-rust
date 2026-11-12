# Hello world

A Trunk app with a centered Say Hello button. The button opens a card. `dialtone` is a private crate. It is not on crates.io. Check out the repository, then build this example from this directory so Cargo picks up the Zig linker.

## Checkout

Your GitHub account needs read access to `nxndps/dialtone-rust`.

```bash
git clone git@github.com:nxndps/dialtone-rust.git
cd dialtone-rust
```

HTTPS works when `gh` can read the private repository:

```bash
gh repo clone nxndps/dialtone-rust
cd dialtone-rust
```

## Prerequisites

- Rust, edition 2021, and Cargo.
- The wasm target: `rustup target add wasm32-unknown-unknown`.
- Trunk: `cargo install trunk`.
- Zig 0.13 or newer on `PATH` (`brew install zig`). Host proc-macros and build scripts link with `scripts/zig-cc`. That replaces Xcode's `cc`. You do not install Xcode or the command line tools for this build.
- Leave `CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER` unset. `.cargo/config.toml` sets `SDKROOT=/` so `rustc` does not call `xcrun`.

The wasm target still links with Rust's `rust-lld`. Zig is only the host linker.

## Build

Run Trunk from this directory. A shell started at the repository root does not use this folder's `.cargo/config.toml`.

```bash
cd examples/hello-world
trunk serve --address 127.0.0.1
```

Open http://127.0.0.1:8084/.

## Developer lifecycle

`trunk serve` compiles the app before it serves the page. The first open waits on that compile. Leave the process running. After the app is built, Trunk watches `src`, `index.html`, `tailwind.css`, and `favicon.svg`. Saving one of those files rebuilds the app and live-reloads the browser.

`trunk build --release` writes the static site to `dist/`. That command compiles once and does not stay running to reload.

## Desktop window

[`examples/hello-tauri`](../hello-tauri/README.md) is a desktop app with this same page. That folder has its own Trunk UI and shell. Run `cargo tauri dev` from `examples/hello-tauri`.

## Use the crate in another app

Keep this checkout. Point the app at `dialtone` with a path dependency. A git dependency checks the crate out into Cargo's cache, and Trunk then has no stable path to `dialtone/assets`.

```toml
[dependencies]
dialtone = { path = "../dialtone-rust/dialtone" }
leptos = { version = "0.8", features = ["csr"] }
getrandom = { version = "0.3", features = ["wasm_js"] }
console_error_panic_hook = "0.1"
```

Copy these pieces from this example into the app:

- `tailwind.css`, with the import paths adjusted to the checkout
- the two `copy-dir` links in `index.html`
- `scripts/zig-cc` and `.cargo/config.toml`
- `<html class="dialtone-theme-light" data-dt-mode="light">`

Import components from the crate. Do not add `mod dialtone` inside the app.

```rust
use dialtone::{DtButton, DtCard, DtStack, DtText};
use leptos::prelude::*;
```

Build from the app directory so the Zig linker config applies.
