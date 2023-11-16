use base64::{engine, Engine as _};
use rsa::{pkcs8::DecodePrivateKey, pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::io::Read;
use std::{env, fs};
use std::{error::Error, fmt};

use crate::constants::DEFAULT_NP_PROFILE;
use crate::constants::{BOLD, END};

#[derive(Debug, Deserialize, Serialize)]
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
    ) -> Result<Self, Box<dyn Error>> {
        let (_priv_key, pubkey) = get_keys(private_key);
        Ok(Self {
            orcid_id: orcid_id.to_string(),
            name: name.to_string(),
            public_key: get_pubkey_str(&pubkey),
            private_key: private_key.to_string(),
            introduction_nanopub_uri: Some(introduction_nanopub_uri.unwrap_or("").to_string()),
        })
    }

    /// Extract profile from YAML file
    pub fn from_file(filepath: &str) -> Result<Self, Box<dyn Error>> {
        let filepath = if filepath.is_empty() {
            DEFAULT_NP_PROFILE
        } else {
            filepath
        };
        let mut file = fs::File::open(filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut profile: NpProfile =
            serde_yaml::from_str(&contents).expect("Error parsing profile.yml");
        // Read private and public keys from file
        profile.private_key = normalize_key(&fs::read_to_string(&profile.private_key)?)?;
        profile.public_key = normalize_key(&fs::read_to_string(&profile.public_key)?)?;
        Ok(profile)
    }
}

impl fmt::Display for NpProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}Nanopub Profile:{}", BOLD, END)?;
        writeln!(f, "{}ORCID:{} {}", BOLD, END, self.orcid_id)?;
        writeln!(f, "{}Name:{} {}", BOLD, END, self.name)?;
        writeln!(f, "{}Public key:{} {}", BOLD, END, self.public_key)?;
        writeln!(f, "{}Private key:{} {}", BOLD, END, self.private_key)?;
        if self.introduction_nanopub_uri.is_some() {
            writeln!(
                f,
                "{}Introduction URI:{} {}",
                BOLD,
                END,
                self.introduction_nanopub_uri.clone().unwrap()
            )?;
        }
        Ok(())
    }
}

/// Get `RsaPrivateKey` and `RsaPublicKey` given a private key string
pub fn get_keys(private_key: &str) -> (RsaPrivateKey, RsaPublicKey) {
    let priv_key_bytes = engine::general_purpose::STANDARD
        .decode(private_key)
        .expect("Error decoding private key");
    let priv_key =
        RsaPrivateKey::from_pkcs8_der(&priv_key_bytes).expect("Failed to parse RSA private key");

    let public_key = RsaPublicKey::from(&priv_key);
    (priv_key, public_key)
}

/// Get a public key string for a `RsaPublicKey`
pub fn get_pubkey_str(public_key: &RsaPublicKey) -> String {
    normalize_key(
        &RsaPublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::LF).unwrap(),
    )
    .unwrap()
}

/// Normalize private/public keys (no prefix, no suffix, no newline)
pub fn normalize_key(key: &str) -> Result<String, Box<dyn Error>> {
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

pub fn get_default_profile_path() -> String {
    // "/home/vemonet/.nanopub/profile.yml"
    format!(
        "{}/.nanopub/profile.yml",
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or("".to_string())
    )
}
// pub const DEFAULT_NP_PROFILES: &str = &format!("{}/.nanopub/profile.yml", env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or("".to_string()));
