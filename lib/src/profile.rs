use base64::{engine, Engine as _};
use rsa::{pkcs8::DecodePrivateKey, pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt;
use std::io::Read;
use std::{env, fs};

use crate::constants::DEFAULT_NP_PROFILE;
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
    pub fn new(
        orcid_id: &str,
        name: &str,
        private_key: &str,
        introduction_nanopub_uri: Option<&str>,
    ) -> Result<Self, NpError> {
        let (_priv_key, pubkey) = get_keys(private_key)?;
        Ok(Self {
            orcid_id: orcid_id.to_string(),
            name: name.to_string(),
            public_key: get_pubkey_str(&pubkey)?,
            private_key: private_key.to_string(),
            introduction_nanopub_uri: Some(introduction_nanopub_uri.unwrap_or("").to_string()),
        })
    }

    /// Extract profile from YAML file
    pub fn from_file(filepath: &str) -> Result<Self, NpError> {
        let filepath = if filepath.is_empty() {
            DEFAULT_NP_PROFILE
        } else {
            filepath
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

/// Get `RsaPrivateKey` and `RsaPublicKey` given a private key string
pub fn get_keys(private_key: &str) -> Result<(RsaPrivateKey, RsaPublicKey), NpError> {
    let priv_key_bytes = engine::general_purpose::STANDARD.decode(private_key)?;
    let priv_key = RsaPrivateKey::from_pkcs8_der(&priv_key_bytes)?;
    let public_key = RsaPublicKey::from(&priv_key);
    Ok((priv_key, public_key))
}

/// Get a public key string for a `RsaPublicKey`
pub fn get_pubkey_str(public_key: &RsaPublicKey) -> Result<String, NpError> {
    normalize_key(&RsaPublicKey::to_public_key_pem(
        public_key,
        rsa::pkcs8::LineEnding::LF,
    )?)
}

/// Normalize private/public keys (no prefix, no suffix, no newline)
pub fn normalize_key(key: &str) -> Result<String, NpError> {
    let mut normed_key = key.trim();
    let rm_prefix = "-----BEGIN PUBLIC KEY-----";
    if normed_key.starts_with(rm_prefix) {
        normed_key = &normed_key[rm_prefix.len()..].trim();
    }
    let rm_suffix = "-----END PUBLIC KEY-----";
    if normed_key.ends_with(rm_suffix) {
        normed_key = &normed_key[..normed_key.len() - rm_suffix.len() - 1].trim();
    }
    Ok(normed_key.trim().replace('\n', ""))
}

/// Try to get default profile path from users home folder
pub fn get_default_profile_path() -> String {
    format!(
        "{}/.nanopub/profile.yml",
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or("~".to_string())
    )
}
