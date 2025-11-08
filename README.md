# synth8z_wasm

A Rust + WebAssembly implementation of the synth8z landing page hero.
All dynamic rendering (typing three lines, fades, and final brand reveal)
is performed by the Wasm module. CSS controls the visuals.

## Build

Requirements:
- Rust toolchain (https://rustup.rs)
- wasm-pack: `cargo install wasm-pack`

```bash
# from the project root (this directory)
wasm-pack build --target web --out-dir ./static/pkg
cd static
python3 -m http.server 8080
# Open http://localhost:8080
```
