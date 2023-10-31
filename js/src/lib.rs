// #![allow(clippy::unused_unit)]
use wasm_bindgen::prelude::*;

mod nanopub;
mod utils;


#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

