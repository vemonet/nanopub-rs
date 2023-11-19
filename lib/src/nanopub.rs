use crate::constants::{BOLD, END, NP_PREF_NS, NP_TEMP_URI, TEST_SERVER};
use crate::error::{NpError, TermError};
use crate::profile::{get_keys, get_pubkey_str, NpProfile};
use crate::publish::publish_np;
use crate::sign::{make_trusty, normalize_dataset, replace_bnodes, replace_ns_in_quads};
use crate::utils::{get_ns, parse_rdf, serialize_rdf};

use base64;
use base64::{engine, Engine as _};
use regex::Regex;
use rsa::pkcs8::DecodePublicKey;
use rsa::{sha2::Digest, sha2::Sha256, Pkcs1v15Sign, RsaPublicKey};
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::ns::{rdf, Namespace};
use sophia::api::quad::Quad;
use sophia::api::term::{matcher::Any, Term};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use std::collections::HashSet;
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
    pub separator_before_trusty: String,
    pub separator_after_trusty: String,
    pub trusty_hash: String,
    pub signature: String,
    pub signature_iri: Iri<String>,
    pub algo: String,
    pub public_key: String,
}

impl fmt::Display for NpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}Nanopub URL:{} {}", BOLD, END, self.uri)?;
        writeln!(f, "{}Namespace:{} {}", BOLD, END, *self.ns)?;
        writeln!(f, "{}Base URI:{} {}", BOLD, END, self.base_uri)?;
        writeln!(f, "{}Trusty Hash:{} {}", BOLD, END, self.trusty_hash)?;
        writeln!(f, "{}Head Graph:{} {}", BOLD, END, self.head)?;
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
    /// Check a given Nanopub RDF is valid (check trusty hash and signature).
    ///
    /// A failed check will throw an error (panic)
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
    pub fn check(rdf: &str) -> Result<Self, NpError> {
        let mut dataset: LightDataset = parse_rdf(rdf)?;
        let np_info = extract_np_info(&dataset)?;

        let norm_ns = if !np_info.trusty_hash.is_empty() {
            format!("{}{}", np_info.base_uri, np_info.separator_before_trusty)
        } else {
            NP_PREF_NS.to_string()
        };

        let mut msg: String = "".to_string();
        if np_info.trusty_hash.is_empty() {
            msg = format!("{}1 valid (not trusty)", msg);
        } else {
            // Check Trusty hash if found
            let expected_hash = make_trusty(
                &dataset,
                &np_info.ns,
                &norm_ns,
                &np_info.separator_after_trusty,
            )?;
            if expected_hash != np_info.trusty_hash {
                return Err(NpError(format!("Invalid Nanopub: the hash of the nanopublication is different than the expected hash \n{}\n{}", np_info.trusty_hash, expected_hash).to_string()));
            }
            msg = format!("{}1 trusty", msg);
        }

        // Check signature if found
        if !np_info.signature.is_empty() {
            // Remove the signature from the graph before re-generating it
            dataset.remove(
                &np_info.signature_iri,
                get_ns("npx").get("hasSignature")?,
                np_info.signature.as_str(),
                Some(&np_info.pubinfo),
            )?;
            // Normalize nanopub nquads to a string
            let norm_quads = normalize_dataset(
                &dataset,
                &np_info.ns,
                &norm_ns,
                &np_info.separator_after_trusty,
            )?;
            // println!("NORMED QUADS CHECK\n{}", norm_quads);

            // Load public key
            let pubkey_bytes = engine::general_purpose::STANDARD.decode(&np_info.public_key)?;
            let pubkey = RsaPublicKey::from_public_key_der(&pubkey_bytes)?;

            // Regenerate and check the signature hash
            pubkey.verify(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes()),
                &engine::general_purpose::STANDARD.decode(np_info.signature.as_bytes())?,
            )?;
            msg = format!("{} with signature", msg);
        } else {
            msg = format!("{} without signature", msg);
        }

        println!(
            "\n✅ Nanopub {}{}{} is valid: {}",
            BOLD, np_info.uri, END, msg
        );
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
    ) -> Result<Self, NpError> {
        // If the nanopub is already signed we verify it, then publish it
        let dataset: LightDataset = parse_rdf(rdf)?;
        let np_info = extract_np_info(&dataset)?;

        let mut np = if np_info.signature.is_empty() {
            println!("Nanopub not signed, signing it before publishing");
            Nanopub::sign(rdf, profile)?
        } else {
            Nanopub::check(rdf)?
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
            println!("\n❌ Issue publishing the Nanopublication \n{}", np);
            // TODO: when publish fails, should we return a Nanopub struct with published=false, or throw an error?
            // return Err(NpError(format!("Issue publishing the Nanopublication \n{}", np)))
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
    pub fn sign(rdf: &str, profile: &NpProfile) -> Result<Self, NpError> {
        openssl_probe::init_ssl_cert_env_vars();

        let (priv_key, pubkey) = get_keys(&profile.private_key)?;
        let pubkey_str = get_pubkey_str(&pubkey);

        // Parse the provided RDF
        let mut dataset: LightDataset = parse_rdf(rdf)?;

        // Extract graph URLs from the nanopub (fails if np not valid)
        let np_info = extract_np_info(&dataset)?;

        dataset = replace_bnodes(&dataset, &np_info.ns, &np_info.uri)?;

        let np_info = extract_np_info(&dataset)?;

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
            np_info.ns.get("")?,
            Some(&np_info.pubinfo),
        )?;

        // TODO: if not already set, automatically add the current date to pubinfo created
        // np_uri dct:created "2023-11-17T14:13:52.560Z"^^xsd:dateTime ;
        // if dataset
        //     .quads_matching(
        //         [
        //             &np_info.uri,
        //             &Iri::new_unchecked(np_info.ns.get("")?.to_string()),
        //         ],
        //         [get_ns("dct").get("created")?],
        //         Any,
        //         [Some(&np_info.pubinfo)],
        //     )
        //     .next()
        //     .is_none()
        // {
        //     let now = Utc::now();
        //     let datetime_str = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        //     // TODO: error when trying to convert to datetime
        //     //  let lit_date = "2019" * xsd::dateTime;
        //     dataset.insert(
        //         &np_info.uri,
        //         get_ns("dct").get("created")?,
        //         &*datetime_str,
        //         Some(&np_info.pubinfo),
        //     )?;
        // }

        // If ORCID provided, add to pubinfo graph
        if !profile.orcid_id.is_empty()
            && dataset
                .quads_matching(
                    [
                        &np_info.uri,
                        &Iri::new_unchecked(np_info.ns.get("")?.to_string()),
                    ],
                    [get_ns("dct").get("creator")?],
                    // TODO: also skip if pav:createdBy is present?
                    Any,
                    [Some(&np_info.pubinfo)],
                )
                .next()
                .is_none()
        {
            dataset.insert(
                &np_info.uri,
                get_ns("dct").get("creator")?,
                Iri::new_unchecked(profile.orcid_id.clone()),
                Some(&np_info.pubinfo),
            )?;
        }

        let norm_ns = if np_info.ns.starts_with(NP_TEMP_URI) {
            NP_PREF_NS
        } else {
            &np_info.ns
        };

        // Normalize nanopub nquads to a string
        let norm_quads = normalize_dataset(
            &dataset,
            np_info.ns.as_str(),
            norm_ns,
            &np_info.separator_after_trusty,
        )?;
        // println!("NORMED QUADS sign before add signature\n{}", norm_quads);

        // Generate signature using the private key and normalized RDF
        let signature_vec = priv_key.sign(
            Pkcs1v15Sign::new::<Sha256>(),
            &Sha256::digest(norm_quads.as_bytes()),
        )?;
        let signature_hash = engine::general_purpose::STANDARD.encode(signature_vec);
        // Add the signature to the pubinfo graph
        dataset.insert(
            np_info.ns.get("sig")?,
            get_ns("npx").get("hasSignature")?,
            &*signature_hash,
            Some(&np_info.pubinfo),
        )?;

        // Generate Trusty URI, and replace the old URI with the trusty URI in the dataset
        let trusty_hash = make_trusty(
            &dataset,
            &np_info.ns,
            norm_ns,
            &np_info.separator_after_trusty,
        )?;
        let trusty_uri = format!("{norm_ns}{trusty_hash}");
        let trusty_ns = format!("{trusty_uri}#");
        dataset =
            replace_ns_in_quads(&dataset, &np_info.ns, &np_info.uri, &trusty_ns, &trusty_uri)?;

        // Prepare the trig serializer
        let rdf_str = serialize_rdf(&dataset, &trusty_uri, &trusty_ns)?;

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
pub fn extract_np_info(dataset: &LightDataset) -> Result<NpInfo, NpError> {
    let mut np_url: String = "".to_string();
    let mut head: String = "".to_string();
    let mut assertion: String = "".to_string();
    let mut prov: String = "".to_string();
    let mut pubinfo: String = "".to_string();

    // Extract nanopub URL and head graph
    for q in dataset.quads_matching(
        Any,
        [&rdf::type_],
        [get_ns("np").get("Nanopublication")?],
        Any,
    ) {
        if !np_url.is_empty() {
            return Err(NpError("The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string()));
        } else {
            np_url = q?.s().iri().ok_or(TermError())?.to_string();
            head = q?
                .g()
                .ok_or(TermError())?
                .iri()
                .ok_or(TermError())?
                .to_string();
        }
    }
    if np_url.is_empty() {
        return Err(NpError(
            "The provided RDF does not contain a Nanopublication.".to_string(),
        ));
    }

    let np_iri: Iri<String> = Iri::new_unchecked(np_url);
    let head_iri: Iri<String> = Iri::new_unchecked(head);

    // Extract assertion, prov, pubinfo, and head graphs URLs
    for q in dataset.quads_matching(
        [&np_iri],
        [get_ns("np").get("hasAssertion")?],
        Any,
        [Some(&head_iri)],
    ) {
        assertion = q?.o().iri().ok_or(TermError())?.to_string();
    }
    for q in dataset.quads_matching(
        [&np_iri],
        [get_ns("np").get("hasProvenance")?],
        Any,
        [Some(&head_iri)],
    ) {
        prov = q?.o().iri().ok_or(TermError())?.to_string();
    }
    for q in dataset.quads_matching(
        [&np_iri],
        [get_ns("np").get("hasPublicationInfo")?],
        Any,
        [Some(&head_iri)],
    ) {
        pubinfo = q?.o().iri().ok_or(TermError())?.to_string();
    }

    // Remove last char if it is # or / to get the URI
    let np_iri: Iri<String> =
        if np_iri.ends_with('#') || np_iri.ends_with('/') || np_iri.ends_with('.') {
            match np_iri.chars().last() {
                Some(_) => Iri::new_unchecked(np_iri.to_string()[..np_iri.len() - 1].to_string()),
                None => np_iri,
            }
        } else {
            np_iri
        };

    // Getting potential ns from head graph (removing the last frag from head)
    let np_ns_str = &head_iri[..np_iri.len() + 1];

    // Extract base URI, separator character (# or / or _), and trusty hash (if present) from the np URL
    // Default to empty strings when nothing found
    let mut base_uri: String = "".to_string();
    let mut separator_before_trusty: String = '.'.to_string();
    let mut separator_after_trusty: String = "".to_string();
    let mut trusty_hash: String = "".to_string();

    // Get just the Trusty hash from the URI
    let re_trusty = Regex::new(r"^.*?[/#\.]?(RA[a-zA-Z0-9-_]*)$")?;
    if let Some(caps) = re_trusty.captures(&np_iri.as_ref()) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        trusty_hash = caps.get(1).map_or("", |m| m.as_str()).to_string();
    }

    // Get the base URI and separators from the namespace
    let re_trusty_ns = Regex::new(r"^(.*?)(/|#|\.)?(RA[a-zA-Z0-9-_]*)?([#/\.])?$")?;
    // let re = Regex::new(r"^(.*?)(RA.*)?$")?;
    if let Some(caps) = re_trusty_ns.captures(np_ns_str) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        base_uri = caps.get(1).map_or("", |m| m.as_str()).to_string();
        // The second group captures '/' or '#' if present, defaults to .
        separator_before_trusty = caps
            .get(2)
            .map_or(separator_before_trusty, |m| m.as_str().to_string())
            .to_string();
        // The last group captures everything after 'RA', if present.
        // trusty_hash = caps.get(3).map_or("", |m| m.as_str()).to_string();
        separator_after_trusty = caps
            .get(4)
            .map_or(separator_after_trusty, |m| m.as_str().to_string())
            .to_string();
    }
    if trusty_hash.is_empty() && separator_after_trusty.is_empty() {
        separator_after_trusty = "#".to_string()
    };

    // TODO: handle diff if trusty or not (if not we use default, if trusty we only extract)
    let np_ns =
        if !np_ns_str.ends_with('#') && !np_ns_str.ends_with('/') && !np_ns_str.ends_with('.') {
            if !trusty_hash.is_empty() {
                // TODO: Change the after trusty part?
                Namespace::new_unchecked(np_ns_str.to_string())
            } else {
                Namespace::new_unchecked(format!(
                    "{}.",
                    &np_ns_str.strip_suffix('_').unwrap_or(np_ns_str)
                ))
            }
        } else {
            Namespace::new_unchecked(np_ns_str.to_string())
        };

    // Extract signature and its subject URI
    let pubinfo_iri: Iri<String> = Iri::new_unchecked(pubinfo);
    let mut signature: String = "".to_string();
    let mut signature_iri: Iri<String> = Iri::new_unchecked(np_ns.get("sig")?.to_string());
    for q in dataset.quads_matching(
        Any,
        [get_ns("npx").get("hasSignature")?],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        signature = q?.o().lexical_form().ok_or(TermError())?.to_string();
        signature_iri = Iri::new_unchecked(q?.s().iri().ok_or(TermError())?.to_string());
    }

    // Extract public key
    let mut pubkey: Option<String> = None;
    for q in dataset.quads_matching(
        [&signature_iri],
        [get_ns("npx").get("hasPublicKey")?],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        pubkey = Some(q?.o().lexical_form().ok_or(TermError())?.to_string());
    }

    // Extract algo
    let mut algo: Option<String> = None;
    for q in dataset.quads_matching(
        [&signature_iri],
        [get_ns("npx").get("hasAlgorithm")?],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        algo = Some(q?.o().lexical_form().ok_or(TermError())?.to_string());
    }

    // Check minimal required triples in assertion, prov, pubinfo graphs
    let assertion_iri = Iri::new_unchecked(assertion);
    let prov_iri = Iri::new_unchecked(prov);
    if head_iri.is_empty() {
        return Err(NpError("Invalid Nanopub: no Head graph found.".to_string()));
    }
    if assertion_iri.is_empty() {
        return Err(NpError(
            "Invalid Nanopub: no Assertion graph found.".to_string(),
        ));
    }
    if prov_iri.is_empty() {
        return Err(NpError(
            "Invalid Nanopub: no Provenance graph found.".to_string(),
        ));
    }
    if pubinfo_iri.is_empty() {
        return Err(NpError(
            "Invalid Nanopub: no PubInfo graph found.".to_string(),
        ));
    }
    if dataset
        .quads_matching(Any, Any, Any, [Some(assertion_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the assertion graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching(Any, Any, Any, [Some(prov_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the provenance graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching(Any, Any, Any, [Some(pubinfo_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the pubinfo graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching([assertion_iri.clone()], Any, Any, [Some(prov_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError("Invalid Nanopub: no triples with the assertion graph as subject in the provenance graph.".to_string()));
    }
    if dataset
        .quads_matching(
            [
                np_iri.clone(),
                Iri::new_unchecked(np_ns.get("")?.to_string()),
            ],
            Any,
            Any,
            [Some(pubinfo_iri.clone())],
        )
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples with the nanopub URI as subject in the pubinfo graph."
                .to_string(),
        ));
    }
    let mut graph_names = HashSet::new();
    for g in dataset.graph_names() {
        if let Some(graph_name) = g?.iri() {
            graph_names.insert(graph_name.to_string());
        }
    }
    if graph_names.len() != 4 {
        return Err(NpError(
            format!("Invalid Nanopub: it should have 4 graphs (head, assertion, provenance, pubinfo), but the given nanopub has {} graphs.", graph_names.len())
        ));
    }

    Ok(NpInfo {
        uri: np_iri,
        ns: np_ns,
        head: head_iri,
        assertion: assertion_iri,
        prov: prov_iri,
        pubinfo: pubinfo_iri,
        base_uri,
        separator_before_trusty,
        separator_after_trusty,
        trusty_hash,
        signature,
        signature_iri,
        public_key: pubkey.unwrap_or("".to_string()),
        algo: algo.unwrap_or("".to_string()),
    })
}
