# incremental_rust_game

## Setup Rust and WebAssembly Environment
**Install Rust:** Ensure Rust is installed on your system. If not, install it via [rustup](https://rustup.rs/).

**Add wasm32 Target:** Add the WebAssembly target to your Rust toolchain.

```
rustup target add wasm32-unknown-unknown
```
**Install wasm-bindgen CLI:** This tool facilitates communication between Wasm modules and JS.

```
cargo install wasm-bindgen-cli
```

## Install & Start
Make sure you have installed:
- Rust
- Python

`cd` to root of the repo and run in terminal:
```
.\BUILD.bat
```

Open this url in any browser:
```
http://localhost:8000/
```