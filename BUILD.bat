rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/incremental_rust_game.wasm --out-dir ./pkg --web

python -m http.server 8000