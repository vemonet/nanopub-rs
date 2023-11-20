use nanopub::{Nanopub, NpProfile};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Nanopub)]
pub struct NanopubJs {
    np: Nanopub,
}
// pub struct NanopubJs(Nanopub);

// Optional arguments: https://docs.rs/wasm-bindgen-derive/latest/wasm_bindgen_derive/#optional-arguments
// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[wasm_bindgen(js_class = Nanopub)]
impl NanopubJs {
    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn check(rdf: &str) -> Result<NanopubJs, JsValue> {
        console_error_panic_hook::set_once();
        let np = Nanopub::check(rdf).expect_throw("Error publishing the Nanopub");
        Ok(Self { np })
    }

    // #[wasm_bindgen(constructor)]
    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn publish(
        rdf: &str,
        profile: NpProfileJs,
        server_url: &str,
    ) -> Result<NanopubJs, JsValue> {
        console_error_panic_hook::set_once();
        let np = Nanopub::publish(rdf, &profile.profile, Some(server_url))
            .expect_throw("Error publishing the Nanopub");
        // Nanopub::sign(rdf, &profile).expect_throw("Error signing the Nanopub")
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

#[wasm_bindgen(js_name = NpProfile)]
pub struct NpProfileJs {
    profile: NpProfile,
}

#[wasm_bindgen(js_class = NpProfile)]
impl NpProfileJs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        orcid_id: &str,
        name: &str,
        private_key: &str,
        introduction_nanopub_uri: &str,
    ) -> Result<NpProfileJs, JsValue> {
        console_error_panic_hook::set_once();
        let profile =
            NpProfile::new(orcid_id, name, private_key, Some(introduction_nanopub_uri)).unwrap();
        Ok(Self { profile })
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.profile.to_string()
    }
}
