function runApp() {
  wasm_bindgen.greet('World');
  wasm_bindgen.run();
}

wasm_bindgen('./testy_bg.wasm').then(runApp);


