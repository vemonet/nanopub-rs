extern crate alloc;

use js_sys::Error;
// #![allow(clippy::unused_unit)]
use wasm_bindgen::prelude::*;

mod nanopub;

#[wasm_bindgen(start)]
pub fn startup() {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Improve error messages
#[macro_export]
macro_rules! format_err {
    ($msg:literal $(,)?) => {
        ::wasm_bindgen::JsValue::from(::js_sys::Error::new($msg))
    };
    ($fmt:literal, $($arg:tt)*) => {
        ::wasm_bindgen::JsValue::from(::js_sys::Error::new(&format!($fmt, $($arg)*)))
    };
}

pub fn to_err(e: impl ToString) -> JsValue {
    JsValue::from(Error::new(&e.to_string()))
}

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     pub fn log(s: &str);
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

// #[macro_export]
// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
// }
