# Hello Tauri

A [Tauri 2](https://v2.tauri.app/) app. The window shows a centered Say Hello button. The button opens a card with the new-meeting illustration, the words "Howdy!", and a Bye button that closes the card. The Trunk page and the native shell both live in this directory.

These are the current stable Tauri 2 crates. Tauri 3 is still an alpha, so this example does not use it.

- `tauri` 2.12.5
- `tauri-build` 2.7.4
- `tauri-plugin-opener` 2.7.3
- `tauri-cli` 2.12.4

## Checkout

`dialtone` is private. Check out this repository, then build from this directory.

```bash
git clone git@github.com:nxndps/dialtone-rust.git
cd dialtone-rust
```

`gh repo clone nxndps/dialtone-rust` is the same checkout when `gh` can read the repository.

## Prerequisites

- Rust, edition 2021, and Cargo.
- The wasm target: `rustup target add wasm32-unknown-unknown`.
- Trunk: `cargo install trunk`.
- The Tauri CLI. The steps are in [Install the Tauri CLI](#install-the-tauri-cli). `cargo tauri` is that binary. It is not a crate in this workspace.
- Zig 0.13 or newer on `PATH` (`brew install zig`). `.cargo/config.toml` points the host linker at `scripts/zig-cc`. Run Cargo from this directory so that file applies. Leave `CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER` unset. Do not set `SDKROOT=/`. The wasm target still links with Rust's `rust-lld`.
- Xcode or its command line tools. Tauri compiles Swift and links WebKit against the macOS SDK. Zig does not replace that SDK. `scripts/zig-cc` adds the SDK `usr/lib` directory on every link, because Zig 0.16 does not find `libiconv` or frameworks from `-isysroot`.

Check the shell on the host. Do not pass `--target wasm32-unknown-unknown` for the `hello-tauri` package. That target is for the UI crate, `hello-tauri-ui`.

```bash
cd examples/hello-tauri
cargo check -p hello-tauri
```

## Install the Tauri CLI

Install CLI 2.12.4. This compile can take several minutes.

```bash
cargo install tauri-cli --version "2.12.4" --locked
```

That writes `cargo-tauri` to `~/.cargo/bin`. Cargo runs it as `cargo tauri` when that directory is on `PATH`. Check the install:

```bash
cargo tauri --version
```

The command prints `tauri-cli 2.12.4`. If Cargo reports `no such command: tauri`, add Cargo's bin directory and open a new terminal:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Run

`cargo tauri dev` has to run from this directory so the CLI finds `src-tauri/tauri.conf.json` and Cargo loads `.cargo/config.toml`.

```bash
cd examples/hello-tauri
cargo tauri dev
```

The dev command runs `trunk serve` here. Trunk listens on http://127.0.0.1:1420, then the window opens on that page. Port 1420 is the port in the Tauri Leptos template.

The first shell compile builds `wry` and `tao`. That can take several minutes.

`cargo tauri build` runs `trunk build --release` in this directory and bundles `dist/`. The shell is a workspace member, so the release executable is at the repository root, not under `src-tauri/target`. Trunk does not need to stay running. From this directory:

```bash
cargo tauri build
./scripts/name-dmg
../../target/release/hello-tauri
```

Tauri writes the disk image as `Hello Dialtone_0.1.0_aarch64.dmg`. `./scripts/name-dmg` renames it to `target/release/bundle/dmg/Hello-Dialtone.dmg`.

The Windows zip is a separate cross-compile. It needs the `x86_64-pc-windows-msvc` Rust target, [cargo-xwin](https://github.com/rust-cross/cargo-xwin) 0.23.1, and LLVM (`brew install llvm lld`). `scripts/windows-zip` puts `/opt/homebrew/opt/llvm/bin` and `/opt/homebrew/opt/lld/bin` on `PATH`. `cargo-xwin` downloads the Microsoft CRT and Windows SDK into `~/.cache/xwin`. From this directory:

```bash
rustup target add x86_64-pc-windows-msvc
./scripts/windows-zip
```

The zip is `target/release/bundle/windows/Hello-Dialtone.zip`. It contains `Hello-Dialtone.exe`. Windows 10 and Windows 11 supply the WebView2 runtime that executable uses. The macOS shell still links with Zig. This Windows build uses `clang-cl` from LLVM.

The macOS app bundle is `target/release/bundle/macos/Hello Dialtone.app`. Open it with:

```bash
open "../../target/release/bundle/macos/Hello Dialtone.app"
```

The executable inside that app is `Contents/MacOS/Hello Dialtone`.

## How it is wired

`src/` is the Leptos page. `src-tauri/` is the native shell. Both are members of the repository workspace. `beforeDevCommand` is `trunk serve`. `beforeBuildCommand` is `trunk build --release`. Tauri runs those commands in this directory. `devUrl` is `http://127.0.0.1:1420`. `frontendDist` is `../dist`, which is relative to `src-tauri` and is this directory's `dist/`.

The content security policy is null, as in the Tauri Leptos template, so the wasm bundle and Dialtone styles load. The bundle icons are the globe favicon in this directory.

## Developer lifecycle

`cargo tauri dev` waits until Trunk has compiled the page, then shows the window. Leave the process running. Trunk watches `src`, `index.html`, `tailwind.css`, and `favicon.svg`, rebuilds, and live-reloads the window. It ignores `src-tauri`. Edits under `src-tauri` rebuild the shell.
