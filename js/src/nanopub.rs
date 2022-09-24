use wasm_bindgen::prelude::*;
use nanopub_rs::nanopub::Nanopub;


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
        // let store = Self {
        //     store: Store::new().map_err(to_err)?,
        // };
        Ok(Self {
            np: Nanopub::new(rdf)
            // np: Nanopub::new(rdf.unwrap_or("default in js"))

            // .map_err(map_storage_error)?,
            // np: if let Some(rdf) = rdf {
            //     Nanopub::new(rdf.unwrap_or("default in js"))
            // } else {
            //     Nanopub::new()
            // }
            // .map_err(map_storage_error)?,
        })
    }

    // - preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear;
    // this includes the signature part, except the triple that is stating the actual signature
    // - preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
    // - Signature is calculated on this normalized representation
    // - Signature triple is added
    // - Trusty URI code is calculated on normalized representation that includes signature
    // - Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub


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

