function runApp() {
  wasm_bindgen.greet('World');
  wasm_bindgen.run();
}

wasm_bindgen('./render_bg.wasm').then(runApp);


