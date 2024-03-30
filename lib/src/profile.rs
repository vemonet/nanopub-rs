use base64::{engine, Engine as _};
use getrandom::getrandom;
use rand_core::{impls, CryptoRng, RngCore};
use rsa::{RsaPrivateKey, RsaPublicKey};
// use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey};
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt;
use std::io::Read;
use std::{env, fs};

use crate::error::NpError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpProfile {
    pub orcid_id: String,
    pub name: String,
    pub private_key: String,
    pub public_key: String,
    pub introduction_nanopub_uri: Option<String>,
}

impl NpProfile {
    /// Create a new Nanopub profile
    pub fn new(
        private_key: &str,
        orcid_id: &str,
        name: &str,
        introduction_nanopub_uri: Option<String>,
    ) -> Result<Self, NpError> {
        let privkey =
            RsaPrivateKey::from_pkcs8_der(&engine::general_purpose::STANDARD.decode(private_key)?)?;
        let pubkey = RsaPublicKey::from(&privkey);
        Ok(Self {
            orcid_id: orcid_id.to_string(),
            name: name.to_string(),
            public_key: get_pubkey_str(&pubkey)?,
            private_key: private_key.to_string(),
            introduction_nanopub_uri,
        })
    }

    /// Create a Nanopub profile from a YAML file
    pub fn from_file(filepath: &str) -> Result<Self, NpError> {
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
        let mut file = fs::File::open(filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut profile: NpProfile = serde_yaml::from_str(&contents)?;
        // Read private and public keys from file
        profile.private_key = normalize_key(&fs::read_to_string(&profile.private_key)?)?;
        profile.public_key = normalize_key(&fs::read_to_string(&profile.public_key)?)?;
        Ok(profile)
    }

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
        writeln!(f, "ORCID:{}", self.orcid_id)?;
        writeln!(f, "Name:{}", self.name)?;
        writeln!(f, "Public key: {}", self.public_key)?;
        writeln!(f, "Private key: {}", self.private_key)?;
        if self.introduction_nanopub_uri.is_some() {
            writeln!(
                f,
                "Introduction URI: {}",
                self.introduction_nanopub_uri
                    .clone()
                    .unwrap_or("".to_string())
            )?;
        }
        Ok(())
    }
}

/// Normalize a private or public key string (remove prefix, suffix, newlines)
pub fn normalize_key(key: &str) -> Result<String, NpError> {
    let mut normed_key = key.trim().to_string();
    let start_patterns = [
        "-----BEGIN PUBLIC KEY-----",
        "-----BEGIN PRIVATE KEY-----",
        "-----BEGIN RSA PRIVATE KEY-----",
    ];
    for pattern in start_patterns.iter() {
        if normed_key.starts_with(pattern) {
            normed_key = normed_key[pattern.len()..].to_string();
            break;
        }
    }
    let end_patterns = [
        "-----END PUBLIC KEY-----",
        "-----END PRIVATE KEY-----",
        "-----END RSA PRIVATE KEY-----",
    ];
    for pattern in end_patterns.iter() {
        if normed_key.ends_with(pattern) {
            normed_key = normed_key[..normed_key.len() - pattern.len()].to_string();
            break;
        }
    }
    Ok(normed_key.trim().replace('\n', ""))
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
