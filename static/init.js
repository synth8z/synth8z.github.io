import init from "./pkg/synth8z_wasm.js";
init().then(() => {
  console.info("synth8z wasm module initialized");
}).catch((e) => console.error("WASM init failed", e));
