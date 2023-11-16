use crate::constants::{BOLD, END, NP_PREF_NS, TEMP_NP_URI, TEST_SERVER};
use crate::namespaces::{get_prefixes, get_ns};

use base64;
use base64::{alphabet, engine, Engine as _};
use regex::Regex;
use reqwest::header;
use rsa::pkcs8::DecodePublicKey;
use rsa::{
    pkcs8::DecodePrivateKey, pkcs8::EncodePublicKey, sha2::Digest, sha2::Sha256, Pkcs1v15Sign,
    RsaPrivateKey, RsaPublicKey,
};
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
use std::collections::HashMap;
use std::error::Error;
use std::{cmp::Ordering, fmt, str};

pub struct NpMetadata {
    pub extracted_url: Iri<String>,
    pub extracted_ns: Namespace<String>,
    pub head: Iri<String>,
    pub assertion: Iri<String>,
    pub prov: Iri<String>,
    pub pubinfo: Iri<String>,
    pub base_uri: String,
    pub separator_char: String,
    pub trusty_hash: String,
    pub signature: String,
    pub public_key: String,
}

impl fmt::Display for NpMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}Nanopub URL:{} {}", BOLD, END, self.extracted_url)?;
        writeln!(f, "{}Namespace:{} {}", BOLD, END, *self.extracted_ns)?;
        writeln!(f, "{}Base URI:{} {}", BOLD, END, self.base_uri)?;
        writeln!(f, "{}Trusty Hash:{} {}", BOLD, END, self.trusty_hash)?;
        writeln!(f, "{}Assertion Graph:{} {}", BOLD, END, self.assertion)?;
        Ok(())
    }
}

/// A nanopublication object
pub struct Nanopub {
    pub uri: String,
    pub rdf: String,
    pub trusty_hash: String,
    pub signature_hash: String,
    pub public_key: String,
    pub orcid: String,
    pub published: bool,
    pub metadata: NpMetadata,
    // private_key: String,
    // dataset: LightDataset,
}

impl Nanopub {
    /// Creates a new nanopub
    ///
    /// # Arguments
    ///
    /// * `rdf` - A string slice that holds the RDF of the nanopub
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use nanopub::Nanopub;
    /// let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    /// let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
    /// let np = Nanopub::sign(
    ///     np_rdf.as_str(),
    ///     private_key,
    ///     "https://orcid.org/0000-0000-0000-0000",
    /// );
    /// ```

    // TODO: fn verify
    // TODO: Pass Profile instead of private_key and ORCID separately

    // def verify_signature(g: ConjunctiveGraph, source_namespace: Namespace) -> bool:
    // """Verify RSA signature in a nanopub Graph"""
    // # Get signature and public key from the triples
    // np_sig = extract_np_metadata(g)
    // if not np_sig.signature:
    //     raise MalformedNanopubError("No Signature found in the nanopublication RDF")

    // # Normalize RDF
    // quads = RdfUtils.get_quads(g)
    // normed_rdf = RdfHasher.normalize_quads(
    //     quads,
    //     baseuri=str(source_namespace),
    //     hashstr=" "
    // )

    // # Verify signature using the normalized RDF
    // key = RSA.import_key(decodebytes(str(np_sig.public_key).encode()))
    // hash_value = SHA256.new(normed_rdf.encode())
    // verifier = PKCS1_v1_5.new(key)
    // try:
    //     verifier.verify(hash_value, decodebytes(np_sig.signature.encode()))
    //     return True
    // except Exception as e:
    //     raise MalformedNanopubError(e)

    pub fn check(rdf: &str) -> Result<Self, Box<dyn Error>> {
        let mut dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");
        let np_meta =
            extract_np_metadata(&dataset).expect("The provided Nanopublication is not valid");

        // Check Trusty hash
        let expected_hash = make_trusty(&dataset, &np_meta.extracted_ns).unwrap();
        assert_eq!(expected_hash, np_meta.trusty_hash);

        // TODO: Check Signature
        dataset.remove(
            np_meta.extracted_ns.get("sig")?,
            get_ns("npx").get("hasSignature")?,
            np_meta.signature.as_str(),
            Some(&np_meta.pubinfo),
        )?;
        // Normalize nanopub nquads to a string
        let norm_quads = normalize_dataset(&dataset, &np_meta.extracted_ns, "")
            .expect("Failed to normalise RDF before adding signature");
        println!("NORMED QUADS\n{}", norm_quads);

        // Load public key
        let pubkey_bytes = engine::general_purpose::STANDARD
            .decode(&np_meta.public_key)
            .expect("Error decoding public key");
        let pubkey = RsaPublicKey::from_public_key_der(&pubkey_bytes)
            .expect("Failed to parse RSA public key");

        // Regenerate and check the signature hash
        pubkey
            .verify(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes()),
                &engine::general_purpose::STANDARD
                    .decode(np_meta.signature.as_bytes())
                    .unwrap(),
            )
            .expect("Failed to verify the Nanopub signature hash");
        println!(
            "✅ Nanopub {}{}{} is valid",
            BOLD, np_meta.extracted_url, END
        );

        // TODO: check if the np has been published
        Ok(Self {
            uri: np_meta.extracted_url.to_string(),
            rdf: rdf.to_string(),
            trusty_hash: np_meta.trusty_hash.to_string(),
            signature_hash: np_meta.signature.to_string(),
            public_key: np_meta.public_key.to_string(),
            orcid: np_meta.public_key.to_string(),
            published: false,
            metadata: np_meta,
            // dataset: dataset,
        })
    }

    /// Sign and publish RDF to a nanopub server
    pub fn publish(
        rdf: &str,
        private_key: &str,
        orcid: &str,
        server_url: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        // If already signed we verify it, then publish it
        let dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");
        let np_meta =
            extract_np_metadata(&dataset).expect("The provided Nanopublication is not valid");

        let mut np = if np_meta.signature.is_empty() {
            println!("Nanopub not signed, signing it before publishing");
            Nanopub::sign(rdf, private_key, orcid).unwrap()
        } else {
            Nanopub::check(rdf).unwrap()
        };

        let server_url = if let Some(server_url) = server_url {
            server_url.to_string()
        } else {
            TEST_SERVER.to_string()
        };
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(server_url)
            .body(np.get_rdf())
            .header(header::CONTENT_TYPE, "application/trig")
            // .header(header::ACCEPT, "application/json")
            .send()?;
        println!("{:#?}", res);
        println!("{:#?}", res.text());
        np.set_published(true);
        Ok(np)
    }

    /// Sign a nanopub RDF
    pub fn sign(rdf: &str, private_key: &str, orcid: &str) -> Result<Self, Box<dyn Error>> {
        openssl_probe::init_ssl_cert_env_vars();

        let priv_key_bytes = engine::general_purpose::STANDARD
            .decode(private_key)
            .expect("Error decoding private key");
        let priv_key = RsaPrivateKey::from_pkcs8_der(&priv_key_bytes)
            .expect("Failed to parse RSA private key");

        let public_key = RsaPublicKey::from(&priv_key);
        let pub_key_str = normalize_key(
            &RsaPublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::LF).unwrap(),
        )
        .unwrap();

        // Parse the provided RDF
        let mut dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");

        // Extract graph URLs from the nanopub (fails if np not valid)
        let np_meta =
            extract_np_metadata(&dataset).expect("The provided Nanopublication is not valid");
        println!("{}", np_meta);

        dataset = replace_bnodes(&dataset, &np_meta.extracted_ns).unwrap();

        // Add triples about the signature in the pubinfo
        dataset.insert(
            np_meta.extracted_ns.get("sig")?,
            get_ns("npx").get("hasPublicKey")?,
            &*pub_key_str,
            Some(&np_meta.pubinfo),
        )?;
        dataset.insert(
            np_meta.extracted_ns.get("sig")?,
            get_ns("npx").get("hasAlgorithm")?,
            "RSA",
            Some(&np_meta.pubinfo),
        )?;
        dataset.insert(
            np_meta.extracted_ns.get("sig")?,
            get_ns("npx").get("hasSignatureTarget")?,
            Iri::new_unchecked(TEMP_NP_URI),
            Some(&np_meta.pubinfo),
        )?;

        // Normalize nanopub nquads to a string
        let norm_quads = normalize_dataset(&dataset, np_meta.extracted_ns.as_str(), "")
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
            np_meta.extracted_ns.get("sig")?,
            get_ns("npx").get("hasSignature")?,
            &*signature_hash,
            Some(&np_meta.pubinfo),
        )?;

        // Generate TrustyURI
        let trusty_hash = make_trusty(&dataset, np_meta.extracted_ns.as_str()).unwrap();
        let trusty_uri = format!("{}{}", NP_PREF_NS, trusty_hash);
        let trusty_ns = format!("{}{}", trusty_uri, np_meta.separator_char);
        dataset =
            replace_ns_in_quads(&dataset, &np_meta.extracted_ns, &trusty_ns, &trusty_uri).unwrap();

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

        // Return the Nanopub object
        Ok(Self {
            uri: trusty_uri,
            rdf: rdf_str,
            trusty_hash,
            signature_hash,
            public_key: pub_key_str,
            orcid: orcid.to_string(),
            published: false,
            metadata: np_meta,
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

    pub fn get_ns(&self, ns: &str) -> String {
        self.rdf.clone()
    }

    // /// Returns all the quads contained by the nanopub.
    // pub fn iter(&self) -> Iter<'_> {
    //     let iter = self.spog.iter();
    //     Iter {
    //         dataset: self,
    //         inner: iter,
    //     }
    // }
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
        // for t in self {
        //     info!(f, "{}", t)?;
        // }
        Ok(())
    }
}

/// Generate TrustyURI using base64 encoding
fn make_trusty(dataset: &LightDataset, base_ns: &str) -> Result<String, NanopubError> {
    let norm_quads = normalize_dataset(&dataset, base_ns, "")
        .expect("Failed to normalise RDF after adding signature");
    // println!("NORMED QUADS AFTER SIGNING\n{}", norm_quads);

    let base64_engine = engine::GeneralPurpose::new(
        &alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        )
        .unwrap(),
        engine::GeneralPurposeConfig::new().with_encode_padding(false),
    );
    let trusty_hash = format!(
        "RA{}",
        base64_engine.encode(Sha256::digest(norm_quads.as_bytes()))
    );
    Ok(trusty_hash)
}

/// Extract graphs URLs from a nanopub: nanopub URL, head, assertion, prov, pubinfo
fn extract_np_metadata(dataset: &LightDataset) -> Result<NpMetadata, NanopubError> {
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
            return Err(NanopubError("The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string()));
        } else {
            np_url = Some(q.unwrap().s().iri().unwrap().to_string());
            head = Some(q.unwrap().g().unwrap().iri().unwrap().to_string());
        }
    }
    if np_url.is_none() {
        return Err(NanopubError(
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

    // TODO: Extract signature, algo, public key here too?
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

    let mut pubkey: Option<String> = None;
    for q in dataset.quads_matching(
        [np_ns.get("sig").unwrap()],
        [get_ns("npx").get("hasPublicKey").unwrap()],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        pubkey = Some(q.unwrap().o().lexical_form().unwrap().to_string());
    }

    Ok(NpMetadata {
        extracted_url: np_iri,
        extracted_ns: np_ns,
        head: head_iri,
        assertion: Iri::new_unchecked(assertion.unwrap()),
        prov: Iri::new_unchecked(prov.unwrap()),
        pubinfo: pubinfo_iri,
        base_uri: base_uri.unwrap(),
        separator_char: separator_char.unwrap(),
        trusty_hash: trusty_hash.unwrap(),
        signature: signature.unwrap_or("".to_string()),
        public_key: pubkey.unwrap_or("".to_string()),
    })
}

/// Replace bnodes by URI ending with `_1` in the RDF dataset
fn replace_bnodes(dataset: &LightDataset, base_ns: &str) -> Result<LightDataset, NanopubError> {
    let mut new_dataset = LightDataset::new();
    let mut bnode_map: HashMap<String, usize> = HashMap::new();
    let mut bnode_counter = 1;
    let re_underscore_uri = Regex::new(&format!(r"{}(_+\d+)$", base_ns)).unwrap();

    for quad in dataset.quads() {
        let quad = quad.unwrap();

        // Replace bnode in subjects
        let subject = if quad.s().is_blank_node() {
            let bnode_id = quad.s().bnode_id().unwrap().to_string();
            bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                let counter = bnode_counter;
                bnode_counter += 1;
                counter
            });
            format!("{}_{}", base_ns, bnode_map[&bnode_id])
        } else if let Some(mat) = re_underscore_uri.find(&quad.s().iri().unwrap().as_ref()) {
            let mut subject_iri = quad.s().iri().unwrap().to_string();
            let new_ending = mat.as_str().replacen('_', "__", 1);
            subject_iri.truncate(subject_iri.len() - mat.as_str().len()); // Remove the original ending
            subject_iri.push_str(&new_ending);
            subject_iri
        } else {
            quad.s().iri().unwrap().to_string()
        };

        // Replace bnode in objects
        if quad.o().is_blank_node() {
            let bnode_id = quad.o().bnode_id().unwrap().to_string();
            bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                let counter = bnode_counter;
                bnode_counter += 1;
                counter
            });
            let object = format!("{}_{}", base_ns, bnode_map[&bnode_id]);
            new_dataset
                .insert(
                    &Iri::new_unchecked(subject),
                    quad.p(),
                    &Iri::new_unchecked(object),
                    quad.g(),
                )
                .unwrap();
        } else if quad.o().is_iri() {
            // Handle URI ending with #_1 to double _
            if let Some(mat) = re_underscore_uri.find(&quad.o().iri().unwrap().as_ref()) {
                let mut object_iri = quad.s().iri().unwrap().to_string();
                let new_ending = mat.as_str().replacen('_', "__", 1);
                object_iri.truncate(object_iri.len() - mat.as_str().len()); // Remove the original ending
                object_iri.push_str(&new_ending);
                new_dataset
                    .insert(
                        &Iri::new_unchecked(subject),
                        quad.p(),
                        &Iri::new_unchecked(object_iri),
                        quad.g(),
                    )
                    .unwrap();
            } else {
                new_dataset
                    .insert(&Iri::new_unchecked(subject), quad.p(), quad.o(), quad.g())
                    .unwrap();
            }
        } else {
            new_dataset
                .insert(&Iri::new_unchecked(subject), quad.p(), quad.o(), quad.g())
                .unwrap();
        };
    }
    Ok(new_dataset)
}

/// Replace the dummy nanopub URI by the new one in the RDF dataset
fn replace_ns_in_quads(
    dataset: &LightDataset,
    old_ns: &str,
    new_ns: &str,
    new_uri: &str,
) -> Result<LightDataset, NanopubError> {
    let mut new = LightDataset::new();
    for quad in dataset.quads() {
        let quad = quad.unwrap();
        // Replace URI in subjects
        let subject = if quad.s().iri().unwrap().to_string() == old_ns {
            Iri::new_unchecked(new_uri.to_string())
        } else {
            Iri::new_unchecked(quad.s().iri().unwrap().to_string().replace(old_ns, new_ns))
        };
        // Replace URI in graphs
        let graph = Some(Iri::new_unchecked(
            quad.g()
                .unwrap()
                .iri()
                .unwrap()
                .to_string()
                .replace(old_ns, new_ns),
        ));

        // Replace URI in objects
        if quad.o().is_iri() {
            if quad.o().iri().unwrap().to_string() == old_ns {
                new.insert(
                    &subject,
                    quad.p(),
                    &Iri::new_unchecked(new_uri.to_string()),
                    graph,
                )
                .unwrap();
            } else {
                let object = quad.o().iri().unwrap().to_string().replace(old_ns, new_ns);
                new.insert(&subject, quad.p(), &Iri::new_unchecked(object), graph)
                    .unwrap();
            }
        } else {
            new.insert(&subject, quad.p(), quad.o(), graph).unwrap();
        };
    }
    Ok(new)
}

/// Normalize private/public keys (no prefix, no suffix, no newline)
fn normalize_key(key: &str) -> Result<String, Box<dyn Error>> {
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

#[derive(Debug, Copy, Clone)]
enum Field {
    Graph,
    Subject,
    Predicate,
    Object,
    Datatype,
    // Lang,
}

struct NormQuad {
    graph: String,
    subject: String,
    predicate: String,
    object: String,
    datatype: String,
    lang: String,
}

/// Returns all the quads contained in the nanopub.
fn normalize_dataset(
    dataset: &LightDataset,
    base_ns: &str,
    _hash_str: &str,
) -> Result<String, Box<dyn Error>> {
    let mut quads_vec: Vec<NormQuad> = vec![];
    // let norm_base = "http://purl.org/np/ ";
    let norm_base = "https://w3id.org/np/ ";
    let base_uri = match base_ns.chars().last() {
        Some(_) => &base_ns[..base_ns.len() - 1],
        None => base_ns,
    };

    // Convert dataset to a list of NormQuad struct
    for quad in dataset.quads() {
        let quad = quad.unwrap();

        // Extract components of the quad and convert them to strings. Replace the base URI if present
        let graph = if quad.g().unwrap().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.g()
                .unwrap()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, norm_base)
        };

        let subject = if quad.s().is_blank_node() {
            quad.s().bnode_id().unwrap().to_string()
        } else if quad.s().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.s()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, norm_base)
        };

        let predicate = if quad.p().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.p()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, norm_base)
        };

        let object = if quad.o().is_iri() {
            if quad.o().iri().unwrap().to_string() == base_ns {
                norm_base.to_string()
            } else {
                quad.o()
                    .iri()
                    .unwrap()
                    .to_string()
                    .replace(base_uri, norm_base)
            }
        } else if quad.o().is_blank_node() {
            quad.o().bnode_id().unwrap().to_string()
        } else {
            quad.o().lexical_form().unwrap().to_string()
        };

        // Extract datatype and language if available
        let datatype = if quad.o().datatype().is_some() {
            quad.o().datatype().unwrap().to_string()
        } else {
            "".to_string()
        };
        let lang = if quad.o().language_tag().is_some() {
            quad.o().language_tag().unwrap().to_string()
        } else {
            "".to_string()
        };

        // Create a NormQuad struct and push it to the vector
        quads_vec.push(NormQuad {
            graph,
            subject,
            predicate,
            object,
            datatype,
            lang,
        });
    }

    // Order the list of nquads
    use Field::*;
    let orders = [Graph, Subject, Predicate, Object, Datatype];
    quads_vec.sort_by(|a, b| {
        orders.iter().fold(Ordering::Equal, |acc, &field| {
            acc.then_with(|| match field {
                Graph => a.graph.cmp(&b.graph),
                Subject => a.subject.cmp(&b.subject),
                Predicate => a.predicate.cmp(&b.predicate),
                Object => a.object.cmp(&b.object),
                Datatype => a.datatype.cmp(&b.datatype),
                Lang => a.lang.cmp(&b.lang),
            })
        })
    });
    // println!(quads_vec);

    // Format the ordered list in the normalized string that will be encrypted
    let mut normed_quads = String::new();
    for quad in quads_vec {
        normed_quads.push_str(&format!("{}\n", quad.graph));
        normed_quads.push_str(&format!("{}\n", quad.subject));
        normed_quads.push_str(&format!("{}\n", quad.predicate));

        let formatted_object = if !quad.lang.is_empty() {
            format!("@{} {}", quad.lang, quad.object)
        } else if !quad.datatype.is_empty() {
            format!("^{} {}", quad.datatype, quad.object)
        } else {
            quad.object
        };
        normed_quads.push_str(&formatted_object);
        normed_quads.push('\n');
    }
    Ok(normed_quads)
}

#[derive(Debug)]
struct NanopubError(String);

impl Error for NanopubError {}

impl fmt::Display for NanopubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
