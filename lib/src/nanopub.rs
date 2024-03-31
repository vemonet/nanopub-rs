use crate::constants::{BOLD, END, NP_TEMP_URI, TEST_SERVER};
use crate::error::NpError;
use crate::extract::{extract_np_info, NpInfo};
use crate::network::{fetch_np, publish_np};
use crate::profile::NpProfile;
use crate::sign::{make_trusty, normalize_dataset, replace_bnodes, replace_ns_in_quads};
use crate::utils::{ns, parse_rdf, serialize_rdf};

use base64;
use base64::{engine, Engine as _};
use chrono::Utc;
use rsa::pkcs8::DecodePublicKey;
use rsa::{sha2::Digest, sha2::Sha256, Pkcs1v15Sign, RsaPublicKey};
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::ns::{rdf, xsd, Namespace};
use sophia::api::term::{matcher::Any, Term};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::{AsIriRef, Iri};
use std::collections::HashSet;
use std::{fmt, str};

/// Trait to provide the nanopub RDF as string or sophia dataset
pub trait RdfSource {
    fn get_dataset(self) -> Result<LightDataset, NpError>;
}
impl RdfSource for LightDataset {
    fn get_dataset(self) -> Result<LightDataset, NpError> {
        Ok(self)
    }
}
impl RdfSource for &str {
    fn get_dataset(self) -> Result<LightDataset, NpError> {
        parse_rdf(self)
    }
}
impl RdfSource for &String {
    fn get_dataset(self) -> Result<LightDataset, NpError> {
        parse_rdf(self)
    }
}

/// A Nanopublication, contains the nanopub info (graphs URIs, signature, etc), and the RDF dataset.
#[derive(Clone, Debug)]
pub struct Nanopub {
    pub info: NpInfo,
    pub dataset: LightDataset,
}

impl fmt::Display for Nanopub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{:?}", self.rdf())?;
        writeln!(f, "URI: {}", self.info.uri)?;
        writeln!(f, "Trusty hash: {}", self.info.trusty_hash)?;
        writeln!(f, "Signature hash: {}", self.info.signature)?;
        writeln!(f, "Public key: {}", self.info.public_key)?;
        if let Some(published) = &self.info.published {
            writeln!(f, "Published: {:?}", published)?;
        }
        Ok(())
    }
}

impl Nanopub {
    pub fn new<T: RdfSource>(rdf: T) -> Result<Self, NpError> {
        let dataset = rdf.get_dataset()?;
        let np_info = extract_np_info(&dataset)?;
        Ok(Self {
            info: np_info,
            dataset,
        })
    }

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
    /// let rt = runtime::Runtime::new().expect("Runtime failed");
    ///
    /// let np = rt.block_on(async {
    ///   Nanopub::fetch(&url).await
    /// }).unwrap();
    /// ```
    pub async fn fetch(url: &str) -> Result<Self, NpError> {
        let np_rdf = fetch_np(url).await?;
        let dataset: LightDataset = parse_rdf(&np_rdf)?;
        let mut np_info = extract_np_info(&dataset)?;
        np_info.published = Some(url.to_string());
        Ok(Self {
            info: np_info,
            dataset,
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
    /// let profile = NpProfile::new(&private_key, orcid, "", None).unwrap();
    /// let np = Nanopub::new(&np_rdf).unwrap().check();
    /// ```
    pub fn check(self) -> Result<Self, NpError> {
        let _ = self.is_valid()?;
        let mut msg: String = "".to_string();
        if self.info.trusty_hash.is_empty() {
            msg = format!("{}1 valid (not trusty)", msg);
        } else {
            // Check Trusty hash if found
            let expected_hash = make_trusty(
                &self.dataset,
                &self.info.uri,
                &self.info.normalized_ns,
                &self.info.separator_after_trusty,
            )?;
            if expected_hash != self.info.trusty_hash {
                return Err(NpError(format!("Invalid Nanopub: the hash of the nanopublication is different than the expected hash \n{}\n{}", self.info.trusty_hash, expected_hash).to_string()));
            }
            msg = format!("{}1 trusty", msg);
        }

        // Check the signature is valid if found
        let mut unsigned_dataset = self.dataset.clone();
        if !self.info.signature.is_empty() {
            // Remove the signature from the graph before re-generating it
            unsigned_dataset.remove(
                &self.info.signature_iri,
                ns("npx").get("hasSignature")?,
                self.info.signature.as_str(),
                Some(&self.info.pubinfo),
            )?;
            // Normalize nanopub nquads to a string
            let norm_quads = normalize_dataset(
                &unsigned_dataset,
                &self.info.uri,
                &self.info.normalized_ns,
                &self.info.separator_after_trusty,
            )?;
            // println!("NORMED QUADS CHECK\n{}", norm_quads);

            // Load public key
            let pubkey = RsaPublicKey::from_public_key_der(
                &engine::general_purpose::STANDARD.decode(&self.info.public_key)?,
            )?;

            // Regenerate and check the signature hash
            pubkey.verify(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes()),
                &engine::general_purpose::STANDARD.decode(self.info.signature.as_bytes())?,
            )?;
            msg = format!("{} with signature", msg);
        } else {
            msg = format!("{} without signature", msg);
        }

        println!(
            "\n✅ Nanopub {}{}{} is valid: {}",
            BOLD, self.info.uri, END, msg
        );
        // let rdf = serialize_rdf(&self.dataset, &self.info.uri.as_ref(), &self.info.ns.as_ref())?;
        // TODO: should check return a string or a Nanopub? A string is not easy to process by machines
        // Should we check if the np has been published with Nanopub::fetch?
        Ok(self)
    }

    /// Sign a nanopub: generate and add signature and trusty URI. If the nanopub is already signed, unsign it first.
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
    /// let profile = NpProfile::new(&private_key, orcid, "", None).unwrap();
    /// let np = Nanopub::new(&np_rdf).unwrap().sign(&profile);
    /// ```
    pub fn sign(mut self, profile: &NpProfile) -> Result<Self, NpError> {
        openssl_probe::init_ssl_cert_env_vars();
        self.dataset = replace_bnodes(&self.dataset, &self.info.ns, &self.info.uri)?;
        self.info = extract_np_info(&self.dataset)?;
        if !self.info.signature.is_empty() {
            println!("Nanopub already signed, unsigning it before re-signing");
            self = self.unsign()?;
            // println!("DEBUG: Unsigned: {}", self.rdf()?);
        }

        // Add triples about the signature in the pubinfo
        self.dataset.insert(
            self.info.ns.get("sig")?,
            ns("npx").get("hasPublicKey")?,
            &*profile.public_key,
            Some(&self.info.pubinfo),
        )?;
        self.dataset.insert(
            self.info.ns.get("sig")?,
            ns("npx").get("hasAlgorithm")?,
            "RSA",
            Some(&self.info.pubinfo),
        )?;
        self.dataset.insert(
            self.info.ns.get("sig")?,
            ns("npx").get("hasSignatureTarget")?,
            self.info.ns.get("")?,
            Some(&self.info.pubinfo),
        )?;

        // If not already set, automatically add the current date to pubinfo created
        if self
            .dataset
            .quads_matching(
                [
                    &self.info.uri,
                    &Iri::new_unchecked(self.info.ns.get("")?.to_string()),
                ],
                [ns("dct").get("created")?],
                Any,
                [Some(&self.info.pubinfo)],
            )
            .next()
            .is_none()
        {
            let datetime_now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            self.dataset.insert(
                self.info.ns.as_iri_ref(),
                ns("dct").get("created")?,
                datetime_now.as_str() * xsd::dateTime,
                Some(&self.info.pubinfo),
            )?;
        }

        // If ORCID provided and not already provided, add to pubinfo graph
        if !profile.orcid_id.is_empty()
            && self
                .dataset
                .quads_matching(
                    [
                        &self.info.uri,
                        &Iri::new_unchecked(self.info.ns.get("")?.to_string()),
                    ],
                    [
                        ns("dct").get("creator")?,
                        ns("prov").get("wasAttributedTo")?,
                        ns("pav").get("createdBy")?,
                    ],
                    Any,
                    [Some(&self.info.pubinfo)],
                )
                .next()
                .is_none()
        {
            self.dataset.insert(
                self.info.ns.as_iri_ref(),
                ns("dct").get("creator")?,
                Iri::new_unchecked(profile.orcid_id.clone()),
                Some(&self.info.pubinfo),
            )?;
        }

        // Normalize nanopub nquads to a string
        let norm_quads = normalize_dataset(
            &self.dataset,
            self.info.ns.as_str(),
            &self.info.normalized_ns,
            &self.info.separator_after_trusty,
        )?;
        // println!("NORMED QUADS sign before add signature\n{}", norm_quads);

        // Generate signature using the private key and normalized RDF
        let signature_vec = profile.get_private_key()?.sign(
            Pkcs1v15Sign::new::<Sha256>(),
            &Sha256::digest(norm_quads.as_bytes()),
        )?;
        let signature_hash = engine::general_purpose::STANDARD.encode(signature_vec);
        // Add the signature to the pubinfo graph
        self.dataset.insert(
            self.info.ns.get("sig")?,
            ns("npx").get("hasSignature")?,
            &*signature_hash,
            Some(&self.info.pubinfo),
        )?;

        // Generate Trusty URI, and replace the old URI with the trusty URI in the dataset
        let trusty_hash = make_trusty(
            &self.dataset,
            &self.info.ns,
            &self.info.normalized_ns,
            &self.info.separator_after_trusty,
        )?;
        let trusty_uri = format!("{}{trusty_hash}", self.info.normalized_ns);
        let trusty_ns = format!("{trusty_uri}#");
        self.dataset = replace_ns_in_quads(
            &self.dataset,
            &self.info.ns,
            &self.info.uri,
            &trusty_ns,
            &trusty_uri,
        )?;
        // TODO: it would be more efficient to assign the self.info field directly in the code above
        self.info = extract_np_info(&self.dataset)?;
        let _ = self.is_valid()?;
        // Return the signed Nanopub object
        Ok(self)
    }

    /// Publish a nanopub to a nanopub server. If the nanopub is not signed and a profile is provided, it will be signed before publishing.
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
    /// let profile = NpProfile::new(&private_key, orcid, "", None).unwrap();
    /// let rt = runtime::Runtime::new().expect("Runtime failed");
    ///
    /// let np = rt.block_on(async {
    ///   Nanopub::new(&np_rdf).unwrap().publish(Some(&profile), None).await
    /// }).unwrap();
    /// ```
    pub async fn publish(
        mut self,
        profile: Option<&NpProfile>,
        server_url: Option<&str>,
    ) -> Result<Self, NpError> {
        self = if let Some(profile) = profile {
            // If profile provided we sign the nanopub
            self.sign(profile)?
        } else if self.info.signature.is_empty() {
            // If no profile and nanopub not signed we throw an error
            return Err(NpError(format!(
                "No profile provided and nanopub not signed, could not sign the Nanopublication \n{}",
                self
            )));
        } else {
            // If no profile provided, but the nanopub is already signed, we verify it, then publish it
            self.check()?
        };
        // Use test server if server_url not provided
        let server_url = if let Some(server_url) = server_url {
            if server_url.is_empty() {
                TEST_SERVER.to_string()
            } else {
                server_url.to_string()
            }
        } else {
            TEST_SERVER.to_string()
        };
        let published = publish_np(&server_url, &self.rdf()?).await?;
        if published {
            if TEST_SERVER == server_url {
                self.info.published = Some(format!("{}{}", server_url, self.info.trusty_hash));
            } else {
                self.info.published = Some(self.info.uri.to_string());
            }
            // println!(
            //     "\n🎉 Nanopublication published at {}{:?}{}",
            //     BOLD, self.info.published, END
            // );
        } else {
            return Err(NpError(format!(
                "Issue publishing the Nanopublication \n{}",
                self
            )));
        }
        Ok(self)
    }

    /// Unsign a signed nanopub RDF. Remove signature triples and replace trusty URI with default temp URI
    pub fn unsign(mut self) -> Result<Self, NpError> {
        self.dataset.remove(
            &self.info.signature_iri,
            ns("npx").get("hasPublicKey")?,
            &*self.info.public_key,
            Some(&self.info.pubinfo),
        )?;
        self.dataset.remove(
            &self.info.signature_iri,
            ns("npx").get("hasAlgorithm")?,
            &*self.info.algo,
            Some(&self.info.pubinfo),
        )?;
        self.dataset.remove(
            &self.info.signature_iri,
            ns("npx").get("hasSignatureTarget")?,
            &self.info.uri,
            Some(&self.info.pubinfo),
        )?;
        self.dataset.remove(
            &self.info.signature_iri,
            ns("npx").get("hasSignature")?,
            &*self.info.signature,
            Some(&self.info.pubinfo),
        )?;
        self.dataset = replace_ns_in_quads(
            &self.dataset,
            &self.info.ns,
            &self.info.uri,
            NP_TEMP_URI,
            NP_TEMP_URI,
        )?;
        self.info.uri = Iri::new_unchecked(NP_TEMP_URI.to_string());
        self.info.ns = Namespace::new_unchecked(NP_TEMP_URI.to_string());
        self.info = extract_np_info(&self.dataset)?;
        Ok(self)
    }

    /// Create a Nanopub intro for a Profile
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
    /// let profile = NpProfile::new(&private_key, orcid, "User Name", None).unwrap();
    /// let rt = runtime::Runtime::new().expect("Runtime failed");
    ///
    /// let np = rt.block_on(async {
    ///   Nanopub::new_intro(&profile).unwrap().publish(Some(&profile), None).await.unwrap()
    /// });
    /// ```
    pub fn new_intro(profile: &NpProfile) -> Result<Self, NpError> {
        if profile.orcid_id.is_empty() {
            return Err(NpError("Invalid Profile: ORCID is empty.".to_string()));
        }
        if profile.name.is_empty() {
            return Err(NpError("Invalid Profile: name is empty.".to_string()));
        }
        let mut dataset = create_base_dataset()?;
        let np_ns = Namespace::new_unchecked(NP_TEMP_URI);
        let assertion_graph = np_ns.get("assertion")?;
        let prov_graph = np_ns.get("provenance")?;

        // Assertion graph triples, add key declaration
        dataset.insert(
            np_ns.get("keyDeclaration")?,
            ns("npx").get("declaredBy")?,
            Iri::new_unchecked(profile.orcid_id.as_str()),
            Some(&assertion_graph),
        )?;
        dataset.insert(
            np_ns.get("keyDeclaration")?,
            ns("npx").get("hasAlgorithm")?,
            "RSA",
            Some(&assertion_graph),
        )?;
        dataset.insert(
            np_ns.get("keyDeclaration")?,
            ns("npx").get("hasPublicKey")?,
            profile.public_key.as_str(),
            Some(&assertion_graph),
        )?;
        dataset.insert(
            Iri::new_unchecked(profile.orcid_id.as_str()),
            ns("foaf").get("name")?,
            profile.name.as_str(),
            Some(&assertion_graph),
        )?;
        // Provenance graph triples
        dataset.insert(
            assertion_graph,
            ns("prov").get("wasAttributedTo")?,
            assertion_graph,
            Some(&prov_graph),
        )?;
        Ok(Self {
            info: extract_np_info(&dataset)?,
            dataset,
        })
    }

    /// Check if Nanopub is valid: minimal required triples in assertion, prov, pubinfo graphs
    pub fn is_valid(&self) -> Result<bool, NpError> {
        if self
            .dataset
            .quads_matching(Any, Any, Any, [Some(self.info.assertion.clone())])
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples in the assertion graph.".to_string(),
            ));
        }
        if self
            .dataset
            .quads_matching(Any, Any, Any, [Some(self.info.prov.clone())])
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples in the provenance graph.".to_string(),
            ));
        }
        if self
            .dataset
            .quads_matching(
                [self.info.assertion.clone()],
                Any,
                Any,
                [Some(self.info.prov.clone())],
            )
            .next()
            .is_none()
        {
            return Err(NpError("Invalid Nanopub: no triples with the assertion graph as subject in the provenance graph.".to_string()));
        }
        if self
            .dataset
            .quads_matching(Any, Any, Any, [Some(self.info.pubinfo.clone())])
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples in the pubinfo graph.".to_string(),
            ));
        }
        if self
            .dataset
            .quads_matching(
                [
                    self.info.uri.clone(),
                    Iri::new_unchecked(self.info.ns.get("")?.to_string()),
                ],
                Any,
                Any,
                [Some(self.info.pubinfo.clone())],
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
        for g in self.dataset.graph_names() {
            if let Some(graph_name) = g?.iri() {
                graph_names.insert(graph_name.to_string());
            }
        }
        if graph_names.len() > 4 {
            return Err(NpError(
                format!("Invalid Nanopub: it should have 4 graphs (head, assertion, provenance, pubinfo), but the given nanopub has {} graphs.", graph_names.len())
            ));
        }
        Ok(true)
    }

    /// Returns the RDF of the nanopub
    pub fn rdf(&self) -> Result<String, NpError> {
        serialize_rdf(&self.dataset, self.info.uri.as_str(), self.info.ns.as_str())
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
