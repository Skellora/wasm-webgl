let m = null;
function loop() {
  if (m !== null) {
    m.update();
    m.draw();
  }
  window.requestAnimationFrame(loop);
}

function runApp() {
  wasm_bindgen.greet('World');
  m = wasm_bindgen.init();
  loop();
}

wasm_bindgen('./render_bg.wasm').then(runApp);


