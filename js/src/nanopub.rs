use js_sys::{Promise, JSON};
use nanopub::{
    constants::TEST_SERVER, get_np_server as get_server, profile::gen_keys, Nanopub as RsNanopub,
    NpProfile as RsNpProfile, ProfileBuilder,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_derive::TryFromJsValue;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Nanopub {
    np: RsNanopub,
}

// Maybe try https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html
#[allow(unused_variables, clippy::inherent_to_string)]
#[wasm_bindgen]
impl Nanopub {
    #[wasm_bindgen(constructor)]
    pub fn new(rdf: JsValue) -> Result<Nanopub, JsValue> {
        let rdf_str = if rdf.is_string() {
            rdf.as_string()
                .ok_or_else(|| JsValue::from_str("RDF input must be a string"))?
        } else {
            JSON::stringify(&rdf)
                .map_err(|e| JsValue::from_str("Failed to stringify JSON-LD object"))?
                .as_string()
                .ok_or_else(|| JsValue::from_str("Failed to convert JSON-LD object to string"))?
        };
        RsNanopub::new(&rdf_str)
            .map(|np| Self { np })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn check(self) -> Result<Nanopub, JsValue> {
        self.np
            .check()
            .map(|np| Self { np })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn sign(self, profile: &NpProfile) -> Result<Nanopub, JsValue> {
        self.np
            .sign(&profile.profile)
            .map(|np| Self { np })
            .map_err(|e| JsValue::from_str(&e.to_string()))
        // Alternative:
        // match self.np.sign(&profile.profile) {
        //     Ok(np) => {
        //         self.np = np;
        //         Ok(self)
        //     }
        //     Err(e) => Err(JsValue::from_str(&e.to_string()))
        // }
    }

    // NOTE: optional args docs https://docs.rs/wasm-bindgen-derive/latest/wasm_bindgen_derive/#optional-arguments
    #[wasm_bindgen]
    pub fn publish(self, profile: &OptionNpProfile, server_url: Option<String>) -> Promise {
        // Handle null/undefined profile
        let js_value: &JsValue = profile.as_ref();
        let option_profile: Option<NpProfile> = if js_value.is_undefined() {
            None
        } else {
            match NpProfile::try_from(js_value) {
                Ok(profile) => Some(profile),
                Err(e) => return Promise::reject(&JsValue::from_str(&e.to_string())),
            }
        };
        let profile = if let Some(option_profile) = option_profile {
            Some(option_profile.profile.clone())
        } else {
            None
        };
        future_to_promise(async move {
            match self
                .np
                .publish(profile.as_ref(), server_url.as_deref())
                .await
            {
                Ok(np) => Ok(JsValue::from(Nanopub { np })),
                Err(e) => Err(JsValue::from_str(&format!(
                    "Error publishing the Nanopub: {e}"
                ))),
            }
        })
    }

    #[wasm_bindgen(static_method_of = Nanopub)]
    pub fn fetch(uri: String) -> Promise {
        future_to_promise(async move {
            match RsNanopub::fetch(&uri).await {
                Ok(np) => Ok(JsValue::from(Nanopub { np })),
                Err(e) => Err(JsValue::from_str(&format!(
                    "Error fetching the Nanopub: {e}"
                ))),
            }
        })
    }

    #[wasm_bindgen(static_method_of = Nanopub)]
    pub fn publish_intro(profile: &NpProfile, server_url: String) -> Promise {
        let profile = profile.profile.clone();
        let server_url = if server_url.is_empty() {
            TEST_SERVER
        } else {
            &server_url
        }
        .to_string();
        future_to_promise(async move {
            let np = match RsNanopub::new_intro(&profile) {
                Ok(np) => np,
                Err(e) => {
                    return Err(JsValue::from_str(&format!(
                        "Error creating Nanopub Introduction: {e}"
                    )))
                }
            };
            match np.publish(Some(&profile), Some(&server_url)).await {
                Ok(np) => Ok(JsValue::from(Nanopub { np })),
                Err(e) => Err(JsValue::from_str(&format!(
                    "Error publishing Nanopub Introduction: {e}"
                ))),
            }
        })
    }

    #[wasm_bindgen(js_name = rdf)]
    pub fn rdf(&self) -> Result<String, JsValue> {
        self.np.rdf().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn info(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.np.info).map_err(|e| e.into())
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.np.to_string()
    }
}

/// Nanopub profile in JavaScript
#[derive(TryFromJsValue)]
#[wasm_bindgen]
#[derive(Clone, Serialize)]
pub struct NpProfile {
    profile: RsNpProfile,
}

// Optional arguments: https://docs.rs/wasm-bindgen-derive/latest/wasm_bindgen_derive/#optional-arguments
// https://github.com/rustwasm/wasm-bindgen/issues/2370
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "NpProfile | undefined")]
    pub type OptionNpProfile;
}

#[allow(clippy::inherent_to_string)]
#[wasm_bindgen]
impl NpProfile {
    #[wasm_bindgen(constructor)]
    pub fn new(
        private_key: String,
        orcid_id: Option<String>,
        name: Option<String>,
        introduction_nanopub_uri: Option<String>,
    ) -> Result<NpProfile, JsValue> {
        let mut profile = ProfileBuilder::new(private_key);
        if let Some(orcid_id) = orcid_id {
            profile = profile.with_orcid(orcid_id);
        };
        if let Some(name) = name {
            profile = profile.with_name(name);
        };
        if let Some(intro_np_uri) = introduction_nanopub_uri {
            profile = profile.with_intro_nanopub(intro_np_uri);
        };
        Ok(Self {
            profile: profile
                .build()
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }
    // TODO: create from profile.yml file?

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.profile.to_string()
    }

    #[wasm_bindgen(js_name = toJs)]
    pub fn to_js(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.profile).map_err(|e| e.into())
    }
}

/// Return a random server or the main nanopub server. Default to random server
#[wasm_bindgen(js_name = getNpServer)]
pub fn get_np_server(random: Option<bool>) -> String {
    get_server(random.unwrap_or(true)).to_string()
}

#[wasm_bindgen(js_name = KeyPair)]
#[derive(Serialize)]
pub struct KeyPair {
    public: String,
    private: String,
}

/// Generate a private/public RSA key pair
#[wasm_bindgen(js_class = KeyPair)]
impl KeyPair {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<KeyPair, JsValue> {
        gen_keys()
            .map(|(private, public)| Self { private, public })
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toJs)]
    pub fn to_js(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self).map_err(|e| e.into())
    }
}

// impl Into<JsValue> for Nanopub {
//     fn into(self) -> JsValue {
//         // JsValue::from_serde(&self).unwrap()
//         self.to_js()
//     }
// }
