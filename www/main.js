function runApp() {
  wasm_bindgen.greet('World');
  let m = wasm_bindgen.init();
  m.update();
}

wasm_bindgen('./render_bg.wasm').then(runApp);


