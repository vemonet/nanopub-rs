use crate::constants::{BOLD, END, NP_PREF_NS, NP_TEMP_URI, TEST_SERVER};
use crate::error::NpError;
use crate::extract::extract_np_info;
use crate::profile::{get_keys, get_pubkey_str, NpProfile};
use crate::publish::{fetch_np, publish_np};
use crate::sign::{make_trusty, normalize_dataset, replace_bnodes, replace_ns_in_quads};
use crate::utils::{ns, parse_rdf, serialize_rdf};

use base64;
use base64::{engine, Engine as _};
use chrono::Utc;
use rsa::pkcs8::DecodePublicKey;
use rsa::{sha2::Digest, sha2::Sha256, Pkcs1v15Sign, RsaPublicKey};
use serde::Serialize;
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::ns::{rdf, xsd, Namespace};
use sophia::api::term::matcher::Any;
use sophia::api::term::{SimpleTerm, Term};
// use sophia::api::;
use sophia::inmem::dataset::LightDataset;
use sophia::iri::{AsIriRef, Iri};
use std::{fmt, str};

/// Trait to provide the nanopub RDF as string or sophia dataset
pub trait RdfSource {
    fn get_dataset(&self) -> Result<LightDataset, NpError>;
}
impl RdfSource for LightDataset {
    fn get_dataset(&self) -> Result<LightDataset, NpError> {
        Ok(self.to_owned())
    }
}
impl RdfSource for &str {
    fn get_dataset(&self) -> Result<LightDataset, NpError> {
        parse_rdf(self)
    }
}
impl RdfSource for &String {
    fn get_dataset(&self) -> Result<LightDataset, NpError> {
        parse_rdf(self)
    }
}

/// A nanopublication object
#[derive(Clone, Serialize)]
pub struct Nanopub {
    pub uri: String,
    pub ns: String,
    pub rdf: String,
    pub trusty_hash: String,
    pub signature_hash: String,
    pub public_key: String,
    pub orcid: String,
    pub published: bool,
    // pub info: NpInfo,
    // dataset: LightDataset,
}

impl fmt::Display for Nanopub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}", self.rdf)?;
        writeln!(f, "URI: {}", self.uri)?;
        writeln!(f, "Trusty hash: {}", self.trusty_hash)?;
        writeln!(f, "Signature hash: {}", self.signature_hash)?;
        writeln!(f, "Public key: {}", self.public_key)?;
        writeln!(f, "Published: {}", self.published)?;
        Ok(())
    }
}

impl Nanopub {
    /// Fetch a Nanopub given its URI.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the nanopub to fetch
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use nanopub::Nanopub;
    /// use tokio::runtime;
    /// let url = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
    /// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
    ///
    /// let np = rt.block_on(async {
    ///   Nanopub::fetch(&url).await
    /// }).unwrap();
    /// ```
    pub async fn fetch(url: &str) -> Result<Self, NpError> {
        let np_rdf = fetch_np(url).await?;
        let dataset: LightDataset = parse_rdf(&np_rdf)?;
        let np_info = extract_np_info(&dataset, true)?;
        // TODO: do a Nanopub::check()?
        Ok(Self {
            uri: np_info.uri.to_string(),
            ns: np_info.ns.to_string(),
            rdf: np_rdf,
            trusty_hash: np_info.trusty_hash,
            signature_hash: np_info.signature,
            public_key: np_info.public_key,
            orcid: np_info.orcid,
            published: true,
        })
    }
    /// Check a given Nanopub RDF is valid (check trusty hash and signature).
    ///
    /// A failed check will throw an error
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
    pub fn check<T: RdfSource>(rdf: T) -> Result<Self, NpError> {
        let mut dataset = rdf.get_dataset()?;
        let np_info = extract_np_info(&dataset, true)?;

        let mut msg: String = "".to_string();
        if np_info.trusty_hash.is_empty() {
            msg = format!("{}1 valid (not trusty)", msg);
        } else {
            // Check Trusty hash if found
            let expected_hash = make_trusty(
                &dataset,
                &np_info.ns,
                &np_info.normalized_ns,
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
                ns("npx").get("hasSignature")?,
                np_info.signature.as_str(),
                Some(&np_info.pubinfo),
            )?;
            // Normalize nanopub nquads to a string
            let norm_quads = normalize_dataset(
                &dataset,
                &np_info.ns,
                &np_info.normalized_ns,
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
        let rdf = serialize_rdf(&dataset, &np_info.uri.as_ref(), &np_info.ns.as_ref())?;
        // TODO: check if the np has been published with Nanopub::fetch
        Ok(Self {
            uri: np_info.uri.to_string(),
            ns: np_info.ns.to_string(),
            rdf,
            trusty_hash: np_info.trusty_hash,
            signature_hash: np_info.signature,
            public_key: np_info.public_key,
            orcid: np_info.orcid,
            published: false,
        })
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
    pub fn sign<T: RdfSource>(rdf: T, profile: &NpProfile) -> Result<Self, NpError> {
        openssl_probe::init_ssl_cert_env_vars();
        let mut dataset = rdf.get_dataset()?;

        let (priv_key, pubkey) = get_keys(&profile.private_key)?;
        let pubkey_str = get_pubkey_str(&pubkey)?;

        // Extract graph URLs from the nanopub (fails if np not valid)
        let np_info = extract_np_info(&dataset, false)?;

        dataset = replace_bnodes(&dataset, &np_info.ns, &np_info.uri)?;

        let np_info = extract_np_info(&dataset, false)?;

        // Add triples about the signature in the pubinfo
        dataset.insert(
            np_info.ns.get("sig")?,
            ns("npx").get("hasPublicKey")?,
            &*pubkey_str,
            Some(&np_info.pubinfo),
        )?;
        dataset.insert(
            np_info.ns.get("sig")?,
            ns("npx").get("hasAlgorithm")?,
            "RSA",
            Some(&np_info.pubinfo),
        )?;
        dataset.insert(
            np_info.ns.get("sig")?,
            ns("npx").get("hasSignatureTarget")?,
            np_info.ns.get("")?,
            Some(&np_info.pubinfo),
        )?;

        // TODO: if not already set, automatically add the current date to pubinfo created
        // But there is an error when trying to cast the string to xsd::dateTime
        // np_uri dct:created "2023-11-17T14:13:52.560Z"^^xsd:dateTime ;
        if dataset
            .quads_matching(
                [
                    &np_info.uri,
                    &Iri::new_unchecked(np_info.ns.get("")?.to_string()),
                ],
                [ns("dct").get("created")?],
                Any,
                [Some(&np_info.pubinfo)],
            )
            .next()
            .is_none()
        {
            let now = Utc::now();
            let datetime_str = now.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            // TODO: error when trying to convert to datetime
            // let lit_date = "2019" * xsd::dateTime;
            // let lit_date = datetime_str.as_str() * xsd::dateTime;
            let lit_date = SimpleTerm::LiteralDatatype(datetime_str.into(), xsd::dateTime.iriref());
            dataset.insert(
                np_info.ns.as_iri_ref(),
                ns("dct").get("created")?,
                lit_date,
                // &*datetime_str * xsd::dateTime.iriref(),
                Some(&np_info.pubinfo),
            )?;
        }

        // If ORCID provided and not already provided, add to pubinfo graph
        if !profile.orcid_id.is_empty()
            && dataset
                .quads_matching(
                    [
                        &np_info.uri,
                        &Iri::new_unchecked(np_info.ns.get("")?.to_string()),
                    ],
                    [
                        ns("dct").get("creator")?,
                        ns("prov").get("wasAttributedTo")?,
                        ns("pav").get("createdBy")?,
                    ],
                    Any,
                    [Some(&np_info.pubinfo)],
                )
                .next()
                .is_none()
        {
            dataset.insert(
                np_info.ns.as_iri_ref(),
                ns("dct").get("creator")?,
                Iri::new_unchecked(profile.orcid_id.clone()),
                Some(&np_info.pubinfo),
            )?;
        }

        // Normalize nanopub nquads to a string
        let norm_quads = normalize_dataset(
            &dataset,
            np_info.ns.as_str(),
            &np_info.normalized_ns,
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
            ns("npx").get("hasSignature")?,
            &*signature_hash,
            Some(&np_info.pubinfo),
        )?;

        // Generate Trusty URI, and replace the old URI with the trusty URI in the dataset
        let trusty_hash = make_trusty(
            &dataset,
            &np_info.ns,
            &np_info.normalized_ns,
            &np_info.separator_after_trusty,
        )?;
        let trusty_uri = format!("{}{trusty_hash}", np_info.normalized_ns);
        let trusty_ns = format!("{trusty_uri}#");
        dataset =
            replace_ns_in_quads(&dataset, &np_info.ns, &np_info.uri, &trusty_ns, &trusty_uri)?;

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
        })
    }

    /// Async function to sign and publish RDF to a nanopub server
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
    /// use tokio::runtime;
    ///
    /// let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    /// let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
    /// let orcid = "https://orcid.org/0000-0000-0000-0000";
    /// let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();
    /// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
    ///
    /// let np = rt.block_on(async {
    ///   Nanopub::publish(&np_rdf, &profile, None).await
    /// }).unwrap();
    /// ```
    pub async fn publish<T: RdfSource>(
        rdf: T,
        profile: &NpProfile,
        server_url: Option<&str>,
    ) -> Result<Self, NpError> {
        openssl_probe::init_ssl_cert_env_vars();
        let dataset = rdf.get_dataset()?;
        let np_info = extract_np_info(&dataset, false)?;

        let mut np = if np_info.signature.is_empty() {
            println!("Nanopub not signed, signing it before publishing");
            Nanopub::sign(rdf, profile)?
        } else {
            // If the nanopub is already signed we verify it, then publish it
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
        let published = publish_np(&server_url, &np.get_rdf()).await?;
        if published {
            println!(
                "\n🎉 Nanopublication published at {}{}{}",
                BOLD, np.uri, END
            );
        } else {
            println!("\n❌ Issue publishing the Nanopublication \n{}", np);
            // TODO: when publish fails, should we return a Nanopub struct with published=false, or throw an error?
            return Err(NpError(format!(
                "Issue publishing the Nanopublication \n{}",
                np
            )));
        }
        np.set_published(published);
        Ok(np)
    }

    /// Async function to sign and publish a Nanopub intro for a Profile
    ///
    /// # Arguments
    ///
    /// * `profile` - The NpProfile with private key and ORCID
    /// * `server_url` - The URL of the server to publish to
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// use nanopub::{Nanopub, NpProfile};
    /// use tokio::runtime;
    ///
    /// let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    /// let orcid = "https://orcid.org/0000-0000-0000-0000";
    /// let profile = NpProfile::new(orcid, "User Name", &private_key, None).unwrap();
    /// let rt = runtime::Runtime::new().expect("Failed to create Tokio runtime");
    ///
    /// let np = rt.block_on(async {
    ///   Nanopub::publish_intro(&profile, None).await
    /// }).unwrap();
    /// ```
    pub async fn publish_intro(
        profile: &NpProfile,
        server_url: Option<&str>,
    ) -> Result<Self, NpError> {
        let mut ds = create_base_dataset()?;
        let np_ns = Namespace::new_unchecked(NP_TEMP_URI);
        let assertion_graph = np_ns.get("assertion")?;
        let prov_graph = np_ns.get("provenance")?;

        // Assertion graph triples, add key declaration
        ds.insert(
            np_ns.get("keyDeclaration")?,
            ns("npx").get("declaredBy")?,
            Iri::new_unchecked(profile.orcid_id.as_str()),
            Some(&assertion_graph),
        )?;
        ds.insert(
            np_ns.get("keyDeclaration")?,
            ns("npx").get("hasAlgorithm")?,
            "RSA",
            Some(&assertion_graph),
        )?;
        ds.insert(
            np_ns.get("keyDeclaration")?,
            ns("npx").get("hasPublicKey")?,
            profile.public_key.as_str(),
            Some(&assertion_graph),
        )?;
        ds.insert(
            Iri::new_unchecked(profile.orcid_id.as_str()),
            ns("foaf").get("name")?,
            profile.name.as_str(),
            Some(&assertion_graph),
        )?;
        // Provenance graph triples
        ds.insert(
            assertion_graph,
            ns("prov").get("wasAttributedTo")?,
            assertion_graph,
            Some(&prov_graph),
        )?;
        Nanopub::publish(ds, profile, server_url).await
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

/// Bootstrap a base nanopub dataset that can be edited later
pub fn create_base_dataset() -> Result<LightDataset, NpError> {
    let mut dataset = LightDataset::new();
    let np_iri = Iri::new_unchecked(NP_TEMP_URI);
    let np_ns = Namespace::new_unchecked(NP_TEMP_URI);
    let head_graph = np_ns.get("Head")?;
    // Add Head graph triples
    dataset.insert(
        np_iri,
        ns("np").get("hasAssertion")?,
        np_ns.get("assertion")?,
        Some(head_graph),
    )?;
    dataset.insert(
        np_iri,
        ns("np").get("hasProvenance")?,
        np_ns.get("provenance")?,
        Some(head_graph),
    )?;
    dataset.insert(
        np_iri,
        ns("np").get("hasPublicationInfo")?,
        np_ns.get("pubinfo")?,
        Some(&head_graph),
    )?;
    dataset.insert(
        np_iri,
        rdf::type_,
        ns("np").get("Nanopublication")?,
        Some(&head_graph),
    )?;
    Ok(dataset)
}
