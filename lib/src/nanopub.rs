use crate::constants::{BOLD, END, NP_PREF_NS, TEMP_NP_URI, TEST_SERVER};
use crate::profile::{get_keys, get_pubkey_str, NpProfile};
use crate::publish::publish_np;
use crate::sign::{make_trusty, normalize_dataset, replace_bnodes, replace_ns_in_quads};
use crate::utils::{get_ns, get_prefixes, NpError};

use base64;
use base64::{engine, Engine as _};
use regex::Regex;
use rsa::pkcs8::DecodePublicKey;
use rsa::{sha2::Digest, sha2::Sha256, Pkcs1v15Sign, RsaPublicKey};
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::ns::{rdf, Namespace};
use sophia::api::quad::Quad;
use sophia::api::serializer::{QuadSerializer, Stringifier};
use sophia::api::source::QuadSource;
use sophia::api::term::{matcher::Any, Term};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia::turtle::parser::trig;
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};
use std::error::Error;
use std::{fmt, str};

/// Infos extracted from a nanopublication: graphs URLs, signature, trusty hash...
pub struct NpInfo {
    pub uri: Iri<String>,
    pub ns: Namespace<String>,
    pub head: Iri<String>,
    pub assertion: Iri<String>,
    pub prov: Iri<String>,
    pub pubinfo: Iri<String>,
    pub base_uri: String,
    pub separator_char: String,
    pub trusty_hash: String,
    pub signature: String,
    pub algo: String,
    pub public_key: String,
}

impl fmt::Display for NpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}Nanopub URL:{} {}", BOLD, END, self.uri)?;
        writeln!(f, "{}Namespace:{} {}", BOLD, END, *self.ns)?;
        writeln!(f, "{}Base URI:{} {}", BOLD, END, self.base_uri)?;
        writeln!(f, "{}Trusty Hash:{} {}", BOLD, END, self.trusty_hash)?;
        writeln!(f, "{}Assertion Graph:{} {}", BOLD, END, self.assertion)?;
        Ok(())
    }
}

/// A nanopublication object
pub struct Nanopub {
    pub uri: String,
    pub ns: String,
    pub rdf: String,
    pub trusty_hash: String,
    pub signature_hash: String,
    pub public_key: String,
    pub orcid: String,
    pub published: bool,
    pub info: NpInfo,
    // dataset: LightDataset,
}

impl Nanopub {
    /// Check a given Nanopub RDF is valid (check trusty hash and signature)
    ///
    /// # Arguments
    ///
    /// * `rdf` - A string slice that holds the RDF of the nanopub
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use nanopub::{Nanopub, NpProfile};
    /// let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    /// let np_rdf = fs::read_to_string("./tests/resources/signed.simple1-rsa.trig").unwrap();
    /// let orcid = "https://orcid.org/0000-0000-0000-0000";
    /// let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();
    /// let np = Nanopub::check(&np_rdf).unwrap();
    /// ```
    ///
    pub fn check(rdf: &str) -> Result<Self, Box<dyn Error>> {
        let mut dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");
        let np_info = extract_np_info(&dataset).expect("The provided Nanopublication is not valid");

        let norm_ns = if !np_info.trusty_hash.is_empty() {
            format!("{}{}", np_info.base_uri, np_info.separator_char)
        } else {
            NP_PREF_NS.to_string()
        };

        // Check Trusty hash
        let expected_hash = make_trusty(&dataset, &np_info.ns, &norm_ns).unwrap();
        assert_eq!(expected_hash, np_info.trusty_hash);

        // Remove the signature from the graph before re-generating it
        dataset.remove(
            np_info.ns.get("sig")?,
            get_ns("npx").get("hasSignature")?,
            np_info.signature.as_str(),
            Some(&np_info.pubinfo),
        )?;
        // Normalize nanopub nquads to a string
        let norm_quads = normalize_dataset(&dataset, &np_info.ns, &norm_ns)
            .expect("Failed to normalise RDF before adding signature");
        // println!("NORMED QUADS\n{}", norm_quads);

        // Load public key
        let pubkey_bytes = engine::general_purpose::STANDARD
            .decode(&np_info.public_key)
            .expect("Error decoding public key");
        let pubkey = RsaPublicKey::from_public_key_der(&pubkey_bytes)
            .expect("Failed to parse RSA public key");

        // Regenerate and check the signature hash
        pubkey
            .verify(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes()),
                &engine::general_purpose::STANDARD
                    .decode(np_info.signature.as_bytes())
                    .unwrap(),
            )
            .expect("Failed to verify the Nanopub signature hash");

        println!("\n✅ The Nanopub {}{}{} is valid", BOLD, np_info.uri, END);
        // TODO: check if the np has been published
        Ok(Self {
            uri: np_info.uri.to_string(),
            ns: np_info.ns.to_string(),
            rdf: rdf.to_string(),
            trusty_hash: np_info.trusty_hash.to_string(),
            signature_hash: np_info.signature.to_string(),
            public_key: np_info.public_key.to_string(),
            orcid: np_info.public_key.to_string(),
            published: false,
            info: np_info,
            // dataset: dataset,
        })
    }

    /// Sign and publish RDF to a nanopub server
    ///
    /// # Arguments
    ///
    /// * `rdf` - A string slice that holds the RDF of the nanopub
    /// * `profile` - The NpProfile with private key and ORCID
    /// * `server_url` - The URL of the server to publish to
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use nanopub::{Nanopub, NpProfile};
    /// let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    /// let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
    /// let orcid = "https://orcid.org/0000-0000-0000-0000";
    /// let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();
    /// let np = Nanopub::publish(&np_rdf, &profile, None).unwrap();
    /// ```
    pub fn publish(
        rdf: &str,
        profile: &NpProfile,
        server_url: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        // If the nanopub is already signed we verify it, then publish it
        let dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");
        let np_info = extract_np_info(&dataset).expect("The provided Nanopublication is not valid");

        let mut np = if np_info.signature.is_empty() {
            println!("Nanopub not signed, signing it before publishing");
            Nanopub::sign(rdf, profile).unwrap()
        } else {
            Nanopub::check(rdf).unwrap()
        };

        let server_url = if let Some(server_url) = server_url {
            server_url.to_string()
        } else {
            // Use test server if None provided
            println!(
                "No server URL provided, using the test server {}",
                TEST_SERVER
            );
            TEST_SERVER.to_string()
        };
        let published = publish_np(&server_url, &np.get_rdf());
        if published {
            println!(
                "\n🎉 Nanopublication published at {}{}{}",
                BOLD, np.uri, END
            );
        } else {
            println!(
                "\n❌ Issue publishing the Nanopublication {}{}{}",
                BOLD, np.uri, END
            );
        }
        np.set_published(published);
        Ok(np)
    }

    /// Sign an unsigned nanopub RDF and add trusty URI
    ///
    /// # Arguments
    ///
    /// * `rdf` - A string slice that holds the RDF of the nanopub
    /// * `profile` - The NpProfile with private key and ORCID
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use nanopub::{Nanopub, NpProfile};
    /// let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    /// let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
    /// let orcid = "https://orcid.org/0000-0000-0000-0000";
    /// let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();
    /// let np = Nanopub::sign(&np_rdf, &profile).unwrap();
    /// ```
    pub fn sign(rdf: &str, profile: &NpProfile) -> Result<Self, Box<dyn Error>> {
        openssl_probe::init_ssl_cert_env_vars();

        let (priv_key, pubkey) = get_keys(&profile.private_key);
        let pubkey_str = get_pubkey_str(&pubkey);

        // Parse the provided RDF
        let mut dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");

        // Extract graph URLs from the nanopub (fails if np not valid)
        let np_info = extract_np_info(&dataset).expect("The provided Nanopublication is not valid");
        // println!("{}", np_info);

        dataset = replace_bnodes(&dataset, &np_info.ns).unwrap();

        // Add triples about the signature in the pubinfo
        dataset.insert(
            np_info.ns.get("sig")?,
            get_ns("npx").get("hasPublicKey")?,
            &*pubkey_str,
            Some(&np_info.pubinfo),
        )?;
        dataset.insert(
            np_info.ns.get("sig")?,
            get_ns("npx").get("hasAlgorithm")?,
            "RSA",
            Some(&np_info.pubinfo),
        )?;
        dataset.insert(
            np_info.ns.get("sig")?,
            get_ns("npx").get("hasSignatureTarget")?,
            Iri::new_unchecked(TEMP_NP_URI),
            Some(&np_info.pubinfo),
        )?;

        // Normalize nanopub nquads to a string
        let norm_ns = NP_PREF_NS;
        let norm_quads = normalize_dataset(&dataset, np_info.ns.as_str(), norm_ns)
            .expect("Failed to normalise RDF before adding signature");
        // println!("NORMED QUADS\n{}", norm_quads);

        // Generate signature using the private key and normalized RDF
        let signature_vec = priv_key
            .sign(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes()),
            )
            .expect("Failed to sign nanopub");
        let signature_hash = engine::general_purpose::STANDARD.encode(signature_vec);
        // Add the signature to the pubinfo graph
        dataset.insert(
            np_info.ns.get("sig")?,
            get_ns("npx").get("hasSignature")?,
            &*signature_hash,
            Some(&np_info.pubinfo),
        )?;

        // Generate Trusty URI, and replace the old URI with the trusty URI in the dataset
        let trusty_hash = make_trusty(&dataset, np_info.ns.as_str(), norm_ns).unwrap();
        let trusty_uri = format!("{}{}", NP_PREF_NS, trusty_hash);
        let trusty_ns = format!("{}{}", trusty_uri, np_info.separator_char);
        dataset = replace_ns_in_quads(&dataset, &np_info.ns, &np_info.uri, &trusty_ns, &trusty_uri)
            .unwrap();

        // Prepare the trig serializer
        let prefixes = get_prefixes(&trusty_uri, &trusty_ns);
        let trig_config = TrigConfig::new()
            .with_pretty(true)
            .with_prefix_map(&prefixes[..]);
        let mut trig_stringifier = TrigSerializer::new_stringifier_with_config(trig_config);

        let rdf_str = trig_stringifier
            .serialize_dataset(&dataset)
            .expect("Unable to serialize dataset to trig")
            .to_string();

        // Return the signed Nanopub object
        Ok(Self {
            uri: trusty_uri,
            ns: trusty_ns,
            rdf: rdf_str,
            trusty_hash,
            signature_hash,
            public_key: pubkey_str,
            orcid: profile.orcid_id.to_string(),
            published: false,
            info: np_info,
            // dataset: dataset,
        })
    }

    /// Returns the RDF of the nanopub
    pub fn get_rdf(&self) -> String {
        self.rdf.clone()
    }

    /// Sets if the nanopub has been published to the network
    pub fn set_published(&mut self, value: bool) {
        self.published = value;
    }
}

impl fmt::Display for Nanopub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\n{}Nanopublication: {}{} \n{}",
            BOLD, self.uri, END, self.rdf
        )?;
        writeln!(f, "{}ORCID:{} {}", BOLD, END, self.orcid)?;
        writeln!(f, "{}Public key:{} {}", BOLD, END, self.public_key)?;
        writeln!(f, "{}Trusty hash:{} {}", BOLD, END, self.trusty_hash)?;
        writeln!(f, "{}Signature hash:{} {}", BOLD, END, self.signature_hash)?;
        writeln!(f, "{}Published:{} {}", BOLD, END, self.published)?;
        Ok(())
    }
}

/// Extract graphs URLs from a nanopub: nanopub URL, head, assertion, prov, pubinfo
fn extract_np_info(dataset: &LightDataset) -> Result<NpInfo, NpError> {
    let mut np_url: Option<String> = None;
    let mut head: Option<String> = None;
    let mut assertion: Option<String> = None;
    let mut prov: Option<String> = None;
    let mut pubinfo: Option<String> = None;

    // Extract nanopub URL and head graph
    for q in dataset.quads_matching(
        Any,
        [&rdf::type_],
        [get_ns("np").get("Nanopublication").unwrap()],
        Any,
    ) {
        if np_url.is_some() {
            return Err(NpError("The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string()));
        } else {
            np_url = Some(q.unwrap().s().iri().unwrap().to_string());
            head = Some(q.unwrap().g().unwrap().iri().unwrap().to_string());
        }
    }
    if np_url.is_none() {
        return Err(NpError(
            "The provided RDF does not contain a Nanopublication.".to_string(),
        ));
    }

    let np_iri: Iri<String> = Iri::new_unchecked(np_url.unwrap());
    let head_iri: Iri<String> = Iri::new_unchecked(head.unwrap());

    // Extract assertion, prov, pubinfo, and head graphs URLs
    for q in dataset.quads_matching(
        [&np_iri],
        [get_ns("np").get("hasAssertion").unwrap()],
        Any,
        [Some(&head_iri)],
    ) {
        assertion = Some(q.unwrap().o().iri().unwrap().to_string());
    }
    for q in dataset.quads_matching(
        [&np_iri],
        [get_ns("np").get("hasProvenance").unwrap()],
        Any,
        [Some(&head_iri)],
    ) {
        prov = Some(q.unwrap().o().iri().unwrap().to_string());
    }
    for q in dataset.quads_matching(
        [&np_iri],
        [get_ns("np").get("hasPublicationInfo").unwrap()],
        Any,
        [Some(&head_iri)],
    ) {
        pubinfo = Some(q.unwrap().o().iri().unwrap().to_string());
    }

    // Extract base URI, separator character (# or /), and trusty hash (if present) from the np URL
    // Default to empty strings when nothing found
    let mut base_uri: Option<String> = None;
    let mut separator_char: Option<String> = None;
    let mut trusty_hash: Option<String> = None;
    let re = Regex::new(r"^(.*?)(/|#)?(RA.*)?$").unwrap();
    if let Some(caps) = re.captures(&np_iri) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        base_uri = Some(caps.get(1).map_or("", |m| m.as_str()).to_string());
        // The second group captures '/' or '#' if present.
        separator_char = Some(caps.get(2).map_or("#", |m| m.as_str()).to_string());
        // The third group captures everything after 'RA', if present.
        trusty_hash = Some(caps.get(3).map_or("", |m| m.as_str()).to_string());
    }

    // Get np namespace from the np URL (add # if not ending with / or #)
    let mut namespace: String = np_iri.to_string();
    if !namespace.ends_with('#') && !namespace.ends_with('/') {
        namespace.push('#');
    }
    let np_ns = Namespace::new(namespace).unwrap();

    // Remove last char if it is # or / to get the URI
    let np_iri: Iri<String> = if np_iri.ends_with('#') || np_iri.ends_with('/') {
        match np_iri.chars().last() {
            Some(_) => Iri::new_unchecked(np_iri.to_string()[..np_iri.len() - 1].to_string()),
            None => np_iri,
        }
    } else {
        np_iri
    };

    // Extract signature
    let pubinfo_iri: Iri<String> = Iri::new_unchecked(pubinfo.unwrap());
    let mut signature: Option<String> = None;
    for q in dataset.quads_matching(
        [np_ns.get("sig").unwrap()],
        [get_ns("npx").get("hasSignature").unwrap()],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        signature = Some(q.unwrap().o().lexical_form().unwrap().to_string());
    }

    // Extract public key
    let mut pubkey: Option<String> = None;
    for q in dataset.quads_matching(
        [np_ns.get("sig").unwrap()],
        [get_ns("npx").get("hasPublicKey").unwrap()],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        pubkey = Some(q.unwrap().o().lexical_form().unwrap().to_string());
    }

    // Extract algo
    let mut algo: Option<String> = None;
    for q in dataset.quads_matching(
        [np_ns.get("sig").unwrap()],
        [get_ns("npx").get("hasAlgorithm").unwrap()],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        algo = Some(q.unwrap().o().lexical_form().unwrap().to_string());
    }

    Ok(NpInfo {
        uri: np_iri,
        ns: np_ns,
        head: head_iri,
        assertion: Iri::new_unchecked(assertion.unwrap()),
        prov: Iri::new_unchecked(prov.unwrap()),
        pubinfo: pubinfo_iri,
        base_uri: base_uri.unwrap(),
        separator_char: separator_char.unwrap(),
        trusty_hash: trusty_hash.unwrap(),
        signature: signature.unwrap_or("".to_string()),
        public_key: pubkey.unwrap_or("".to_string()),
        algo: algo.unwrap_or("".to_string()),
    })
}
