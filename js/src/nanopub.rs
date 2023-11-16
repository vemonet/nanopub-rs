use nanopub::{Nanopub, NpProfile};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Nanopub)]
pub struct NanopubJs {
    np: Nanopub,
}

// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen(js_class = Nanopub)]
impl NanopubJs {
    // pub fn new(rdf: Option<&str>) -> Result<JsNanopub, JsValue> {

    #[wasm_bindgen(constructor)]
    pub fn new(
        rdf: &str,
        private_key: &str,
        orcid: &str,
        server_url: &str,
        publish: bool,
    ) -> Result<NanopubJs, JsValue> {
        console_error_panic_hook::set_once();
        let profile = NpProfile::new(orcid, "", private_key, None).unwrap();
        let np = if publish {
            Nanopub::publish(
                // &rdf.unwrap_or("default in py").to_string(),
                rdf,
                profile,
                Some(server_url),
            )
            .expect_throw("Error publishing the Nanopub")
        } else {
            Nanopub::sign(rdf, profile).expect_throw("Error signing the Nanopub")
        };
        Ok(Self { np })
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
