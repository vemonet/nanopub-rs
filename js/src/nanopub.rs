use nanopub::{constants::TEST_SERVER, Nanopub, NpProfile};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Nanopub)]
pub struct NanopubJs {
    np: Nanopub,
}

// Optional arguments: https://docs.rs/wasm-bindgen-derive/latest/wasm_bindgen_derive/#optional-arguments
// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[allow(unused_variables, clippy::inherent_to_string)]
#[wasm_bindgen(js_class = Nanopub)]
impl NanopubJs {
    // #[wasm_bindgen(constructor)]
    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn check(rdf: &str) -> Result<NanopubJs, JsValue> {
        Ok(Self {
            np: Nanopub::check(rdf).expect_throw("Error checking the Nanopub"),
        })
    }

    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn sign(rdf: &str, profile: NpProfileJs) -> Result<NanopubJs, JsValue> {
        Ok(Self {
            np: Nanopub::sign(rdf, &profile.profile).expect_throw("Error signing the Nanopub"),
        })
    }

    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn publish(
        rdf: &str,
        profile: NpProfileJs,
        server_url: &str,
    ) -> Result<NanopubJs, JsValue> {
        let server_url = if server_url.is_empty() {
            TEST_SERVER
        } else {
            server_url
        };
        Ok(Self {
            np: Nanopub::publish(rdf, &profile.profile, Some(server_url))
                .expect_throw("Error publishing the Nanopub"),
        })
    }

    // #[wasm_bindgen]
    pub fn get_rdf(&self) -> Result<String, JsValue> {
        Ok(self.np.get_rdf())
    }

    pub fn published(&self) -> Result<bool, JsValue> {
        Ok(self.np.published)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.np.to_string()
    }
}

#[wasm_bindgen(js_name = NpProfile)]
pub struct NpProfileJs {
    profile: NpProfile,
}
// pub struct NpProfileJs(NpProfile);

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
