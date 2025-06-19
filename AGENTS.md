# AGENTS.md – Agent playbook for *sirlovi-incremental_rust_game*

This file describes repeatable tasks a Large-Language-Model agent should know
about when working in this repository.

## Prerequisites

| Tool / Runtime | Minimum version | Notes |
| -------------- | --------------- | ----- |
| **Rust**       | 1.77 (stable)   | Rustup must be present. |
| **wasm-bindgen-cli** | 0.2.100 | Installed with `cargo install --locked wasm-bindgen-cli`. |
| **Python**     | 3.10            | Used only for the tiny `embed_wasm.py` helper. |
| **Cargo**      | Comes with Rust | `cargo fmt`, `cargo clippy`, etc. |

The WASM target must be installed:  
`rustup target add wasm32-unknown-unknown`

---

## Tasks
```yaml
tasks:
  setup:
    description: Install toolchain prerequisites (idempotent).
    steps:
      - run: rustup target add wasm32-unknown-unknown
      - run: cargo install --locked wasm-bindgen-cli || true

  build:
    description: Build Rust → WASM, generate JS glue, embed WASM as base64.
    steps:
      - run: cargo build --target wasm32-unknown-unknown --release
      - run: |
          wasm-bindgen target/wasm32-unknown-unknown/release/incremental_rust_game.wasm \
            --out-dir pkg --web
      - run: python scripts/embed_wasm.py

  test:
    description: Compile and run all unit + integration tests (native & wasm-bindgen-test).
    steps:
      - run: cargo test --all --locked --verbose

  lint:
    description: Static analysis (format & clippy).
    steps:
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings

  serve:
    description: Start a local dev server at http://localhost:8000/ for manual play-testing.
    steps:
      - run: python -m http.server 8000
    env:
      PORT: 8000

  clean:
    description: Remove build artefacts.
    steps:
      - run: cargo clean
      - run: rm -rf pkg/ wasm32-unknown-unknown
```
<!-- End YAML block -->

### Quick tips for agents

* **Entry point:** open `index.html` (auto-redirects to `ui/`).  
  Front-end glue lives in `src/ui/app.js`, `src/ui/components.js`.
* **Cross-platform:**  
  – Windows users can still double-click `BUILD.bat`.  
  – Agents & CI on Linux/macOS should follow the **build** task above.
* **Offline builds:** if the environment disallows Internet,
  run `cargo vendor --locked` once on a connected machine and push the `vendor/`
  folder. Then agents can call `cargo build --offline`.

> Generated on 2025-06-19  
> Maintainer: *you* – update tasks as the project evolves.