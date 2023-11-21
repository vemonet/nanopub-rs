use js_sys::{Promise, JSON};
use nanopub::{constants::TEST_SERVER, get_np_server as get_server, Nanopub, NpProfile};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

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
    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn check(rdf: &str) -> Result<NanopubJs, JsValue> {
        Nanopub::check(rdf)
            .map(|np| Self { np })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(static_method_of = NanopubJs)]
    pub fn sign(rdf: JsValue, profile: NpProfileJs) -> Result<NanopubJs, JsValue> {
        let rdf_str = if rdf.is_string() {
            rdf.as_string()
                .ok_or_else(|| JsValue::from_str("RDF input must be a string"))?
        } else {
            JSON::stringify(&rdf)
                .map_err(|e| JsValue::from_str("Failed to stringify JSON-LD object"))?
                .as_string()
                .ok_or_else(|| JsValue::from_str("Failed to convert JSON-LD object to string"))?
        };
        Nanopub::sign(&rdf_str, &profile.profile)
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

    #[wasm_bindgen(js_name = toJs)]
    pub fn to_js(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.np).map_err(|e| e.into())
    }
}

/// Nanopub profile in JavaScript
#[wasm_bindgen(js_name = NpProfile)]
#[derive(Serialize)]
pub struct NpProfileJs {
    profile: NpProfile,
}
// pub struct NpProfileJs(NpProfile);

#[allow(clippy::inherent_to_string)]
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
    // TODO: create from profile.yml file

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.profile.to_string()
    }

    #[wasm_bindgen(js_name = toJs)]
    pub fn to_js(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.profile).map_err(|e| e.into())
    }
}

/// Return a random server
#[wasm_bindgen]
pub fn get_np_server(random: Option<bool>) -> String {
    get_server(random.unwrap_or(true)).to_string()
}

// impl Into<JsValue> for NanopubJs {
//     fn into(self) -> JsValue {
//         // JsValue::from_serde(&self).unwrap()
//         self.to_js()
//     }
// }
