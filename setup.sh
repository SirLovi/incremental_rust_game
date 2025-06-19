# setup.sh  (paste this in the “Setup script” box)

# 1) Make sure the WASM target exists
rustup target add wasm32-unknown-unknown

# 2) Make sure the CLI helper is present (locked = reproducible)
cargo install --locked wasm-bindgen-cli >/dev/null 2>&1 || true
