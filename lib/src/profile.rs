use base64::{engine, Engine as _};
use getrandom::getrandom;
use rand_core::{impls, CryptoRng, RngCore};
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{BufRead as _, BufReader};
use std::{env, fs};

use crate::error::NpError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpProfile {
    pub private_key: String,
    pub public_key: String,
    pub orcid_id: Option<String>,
    pub name: Option<String>,
    pub introduction_nanopub_uri: Option<String>,
}

pub struct ProfileBuilder {
    pub private_key: String,
    pub public_key: Option<String>,
    pub orcid_id: Option<String>,
    pub name: Option<String>,
    pub introduction_nanopub_uri: Option<String>,
}

impl ProfileBuilder {
    pub fn new(private_key: String) -> Self {
        ProfileBuilder {
            private_key,
            public_key: None,
            orcid_id: None,
            name: None,
            introduction_nanopub_uri: None,
        }
    }

    pub fn with_orcid(mut self, orcid: String) -> Self {
        self.orcid_id = Some(orcid);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_intro_nanopub(mut self, introduction_nanopub_uri: String) -> Self {
        self.introduction_nanopub_uri = Some(introduction_nanopub_uri);
        self
    }

    pub fn with_public_key(mut self, public_key: String) -> Self {
        self.public_key = Some(public_key);
        self
    }

    /// Build a `NpProfile` struct
    pub fn build(self) -> Result<NpProfile, NpError> {
        // Normalize the private key to ensure it's in the correct format
        let normalized_privkey = normalize_key(&self.private_key)?;

        let pubkey = if let Some(pubkey) = self.public_key {
            pubkey
        } else {
            // Generate public key from private key
            let privkey = RsaPrivateKey::from_pkcs8_der(
                &engine::general_purpose::STANDARD.decode(&normalized_privkey)?,
            )?;
            get_pubkey_str(&RsaPublicKey::from(&privkey))?
        };
        // Check ORCID is valid
        if let Some(orcid) = &self.orcid_id {
            if !orcid.starts_with("https://orcid.org/") {
                return Err(NpError(
                    "The ORCID should start with https://orcid.org/".to_string(),
                ));
            }
        };
        Ok(NpProfile {
            private_key: normalized_privkey,
            public_key: pubkey,
            orcid_id: self.orcid_id,
            name: self.name,
            introduction_nanopub_uri: self.introduction_nanopub_uri,
        })
    }

    /// Create a `NpProfile` from a YAML file, Default to home dir if empty string provided
    pub fn from_file(filepath: &str) -> Result<NpProfile, NpError> {
        let filepath = if filepath.is_empty() {
            // Default to home dir if nothing provided
            format!(
                "{}/.nanopub/profile.yml",
                env::var("HOME")
                    .or_else(|_| env::var("USERPROFILE"))
                    .unwrap_or("~".to_string())
            )
        } else {
            filepath.to_string()
        };
        let file = fs::File::open(filepath)?;
        let reader = BufReader::new(file);
        let mut privkey_path = None;
        // let mut pubkey_path = None;
        let mut orcid = None;
        let mut name = None;
        let mut intro_np_uri = None;
        for line in reader.lines() {
            let line =
                line.map_err(|_| NpError("Failed to read line in profile.yml".to_string()))?;
            if let Some((key, value)) = line.split_once(": ") {
                match key.trim() {
                    "private_key" => privkey_path = Some(remove_quotes(value)),
                    // "public_key" => pubkey_path = Some(remove_quotes(value)),
                    "orcid_id" => orcid = Some(remove_quotes(value)).filter(|s| !s.is_empty()),
                    "name" => name = Some(remove_quotes(value)).filter(|s| !s.is_empty()),
                    "introduction_nanopub_uri" => {
                        intro_np_uri = Some(remove_quotes(value)).filter(|s| !s.is_empty())
                    }
                    _ => {}
                }
            }
        }
        let privkey = normalize_key(&fs::read_to_string(privkey_path.as_ref().ok_or_else(
            || NpError("Invalid Profile: private key file is empty.".to_string()),
        )?)?)?;
        let mut profile = ProfileBuilder::new(privkey);
        // NOTE: we dont get the public key anymore when loading from profile, to avoid issues with keys in OpenSSH format
        // The public key is always generated from the private key now
        // if let Some(pubkey_path) = pubkey_path {
        //     profile = profile.with_public_key(normalize_key(&fs::read_to_string(pubkey_path)?)?);
        // }
        if let Some(orcid) = orcid {
            profile = profile.with_orcid(orcid);
        }
        if let Some(name) = name {
            profile = profile.with_name(name);
        }
        if let Some(intro_nanopub_uri) = intro_np_uri {
            profile = profile.with_intro_nanopub(intro_nanopub_uri);
        }
        profile.build()
    }
}

impl NpProfile {
    /// Get the private key as `RsaPrivateKey` struct
    pub fn get_private_key(&self) -> Result<RsaPrivateKey, NpError> {
        Ok(RsaPrivateKey::from_pkcs8_der(
            &engine::general_purpose::STANDARD.decode(&self.private_key)?,
        )?)
    }

    /// Get the public key as `RsaPublicKey` struct
    pub fn get_public_key(&self) -> Result<RsaPublicKey, NpError> {
        Ok(RsaPublicKey::from(&self.get_private_key()?))
    }
}

impl fmt::Display for NpProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nNanopub Profile:")?;
        writeln!(f, "Public key: {}", self.public_key)?;
        writeln!(f, "Private key: {}", self.private_key)?;
        if let Some(orcid) = &self.orcid_id {
            writeln!(f, "ORCID: {orcid}")?;
        }
        if let Some(name) = &self.name {
            writeln!(f, "Name: {name}")?;
        }
        if let Some(intro_np_uri) = &self.introduction_nanopub_uri {
            writeln!(f, "Introduction URI: {intro_np_uri}")?;
        }
        Ok(())
    }
}

/// Normalize a private or public key string - remove headers/footers and newlines
pub fn normalize_key(key: &str) -> Result<String, NpError> {
    let key_trimmed = key.trim();
    // Check for OpenSSH format keys (not supported)
    if key_trimmed.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----") {
        return Err(NpError(
            "Keys in OpenSSH format are not supported. Please convert to PKCS8 format, or generate a new one with `ssh-keygen -t rsa -m PKCS8 -b 4096 -f ~/.nanopub/id_rsa -C 'your@email.com'`".to_string(),
        ));
    }

    // If it has PEM headers, parse and extract the base64 content
    if key_trimmed.starts_with("-----BEGIN") {
        // Try parsing as PKCS8 PEM private key
        if let Ok(private_key) = RsaPrivateKey::from_pkcs8_pem(key_trimmed) {
            let der = private_key.to_pkcs8_der()?;
            return Ok(engine::general_purpose::STANDARD.encode(der.as_bytes()));
        }
        // Try parsing as PKCS1 PEM private key
        if let Ok(private_key) = RsaPrivateKey::from_pkcs1_pem(key_trimmed) {
            let der = private_key.to_pkcs8_der()?;
            return Ok(engine::general_purpose::STANDARD.encode(der.as_bytes()));
        }
        // Try parsing as PKCS8 PEM public key
        if let Ok(public_key) = RsaPublicKey::from_public_key_pem(key_trimmed) {
            let der = public_key.to_public_key_der()?;
            return Ok(engine::general_purpose::STANDARD.encode(der.as_bytes()));
        }
        // Try parsing as PKCS1 PEM public key
        if let Ok(public_key) = RsaPublicKey::from_pkcs1_pem(key_trimmed) {
            let der = public_key.to_public_key_der()?;
            return Ok(engine::general_purpose::STANDARD.encode(der.as_bytes()));
        }
        return Err(NpError("Failed to parse PEM key".to_string()));
    }
    // // Alternative: if it has PEM headers, just strip them and extract base64
    // if key_trimmed.starts_with("-----BEGIN") {
    //     let mut result = String::new();
    //     for line in key_trimmed.lines() {
    //         if !line.starts_with("-----") {
    //             result.push_str(line.trim());
    //         }
    //     }
    //     return Ok(result);
    // }

    // No headers - assume it's already base64 without headers/newlines
    // Just remove any whitespace and return as-is
    Ok(key_trimmed.replace(['\n', '\r', ' ', '\t'], ""))
}

/// Get a public key string for a `RsaPublicKey`
pub fn get_pubkey_str(pubkey: &RsaPublicKey) -> Result<String, NpError> {
    normalize_key(&pubkey.to_public_key_pem(rsa::pkcs8::LineEnding::LF)?)
}

/// Generate private/public key pair
pub fn gen_keys() -> Result<(String, String), NpError> {
    let mut rng = WasmRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok((
        normalize_key(&priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)?)?,
        get_pubkey_str(&pub_key)?,
    ))
}

/// Removes leading and trailing quotes from a string slice, e.g. YAML value
fn remove_quotes(value: &str) -> String {
    value
        .trim()
        .trim_matches(|c| c == '"' || c == '\'')
        .trim()
        .to_string()
}

// Because of wasm we can't use the rand crate
struct WasmRng;
impl RngCore for WasmRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        getrandom(dest).expect("Error generating random bytes");
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        getrandom(dest).map_err(rand_core::Error::new)
    }
    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }
}
impl CryptoRng for WasmRng {}
