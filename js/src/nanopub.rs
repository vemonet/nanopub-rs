use nanopub_rs::nanopub::Nanopub;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Nanopub)]
pub struct JsNanopub {
    np: Nanopub,
}

// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen(js_class = Nanopub)]
impl JsNanopub {
    // pub fn new(rdf: Option<&str>) -> Result<JsNanopub, JsValue> {

    #[wasm_bindgen(constructor)]
    pub fn new(rdf: &str) -> Result<JsNanopub, JsValue> {
        Ok(Self {
            np: Nanopub::new(&rdf).expect_throw("Error parsing the RDF"),
        })
    }

    // #[wasm_bindgen]
    pub fn get_rdf(&self) -> Result<String, JsValue> {
        Ok(self.np.get_rdf())
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.np.to_string()
    }

    // pub fn update(&self, update: &str) -> Result<(), JsValue> {
    //     self.store.update(update).map_err(to_err)
    // }
}



// let store = Self {
//     store: Store::new().map_err(to_err)?,
// };
// Ok(Self {
//     np: if let Some(rdf) = rdf {
//         Nanopub::new(rdf.unwrap_or("default in js"))
//     } else {
//         Nanopub::new()
//     }
//     .map_err(map_storage_error)?,
// })