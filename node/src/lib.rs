use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen()]
pub fn run() {
  hello();
}

fn hello() {
  log("Hello from Rust!");
}