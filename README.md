# incremental_rust_game

## Setup Rust and WebAssembly Environment
**Install Rust:** [rustup](https://rustup.rs/)

Add the WebAssembly target to your Rust toolchain:

```
rustup target add wasm32-unknown-unknown
```
Install wasm-bindgen CLI:

```
cargo install wasm-bindgen-cli
```

## Install & Start
Make sure you have installed:
- Rust
- Python
- wasm-bindgen-cli

`cd` to root of the repo and run in terminal:
```
.\BUILD.bat
```

Open this URL in any browser:
```
http://localhost:8000/
```

## Farming and Food

The game now tracks food and farms. Click **Collect Food** to manually gather
food or build farms to generate food every second. Farms cost 10 wood and 10
stone each.
