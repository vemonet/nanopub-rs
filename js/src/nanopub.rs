use js_sys::Promise;
use nanopub::{constants::TEST_SERVER, get_np_server as get_server, Nanopub, NpProfile};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
// use js_sys::{Promise, JsValue};

#[wasm_bindgen(js_name = Nanopub)]
#[derive(Serialize)]
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
        Nanopub::check(rdf)
            .map(|np| Self { np })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn sign(rdf: &str, profile: NpProfileJs) -> Result<NanopubJs, JsValue> {
        Nanopub::sign(rdf, &profile.profile)
            .map(|np| Self { np })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn publish(rdf: &str, profile: NpProfileJs, server_url: &str) -> Promise {
        let rdf = rdf.to_string();
        let profile = profile.profile.clone();
        let server_url = if server_url.is_empty() {
            TEST_SERVER
        } else {
            server_url
        }
        .to_string();
        // console_log!("{}", server_url);
        future_to_promise(async move {
            match Nanopub::publish(&rdf, &profile, Some(&server_url)).await {
                Ok(np) => Ok(JsValue::from(NanopubJs { np })),
                Err(e) => Err(JsValue::from_str(&format!(
                    "Error publishing the Nanopub: {e}"
                ))),
            }
        })
    }

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

// impl Into<JsValue> for NanopubJs {
//     fn into(self) -> JsValue {
//         JsValue::from_serde(&self).unwrap()
//     }
// }

#[wasm_bindgen(js_name = NpProfile)]
#[derive(Serialize)]
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
        NpProfile::new(orcid_id, name, private_key, Some(introduction_nanopub_uri))
            .map(|profile: NpProfile| Self { profile })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.profile.to_string()
    }
}

/// Return a random server
#[wasm_bindgen]
pub fn get_np_server(random: Option<bool>) -> String {
    get_server(random.unwrap_or(true)).to_string()
}
