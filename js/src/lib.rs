// #![allow(clippy::unused_unit)]
use wasm_bindgen::prelude::*;

mod nanopub;
mod utils;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}



// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
