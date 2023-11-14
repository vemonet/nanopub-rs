use crate::constants::{NORMALIZED_NS, NORMALIZED_URI, TEMP_NP_NS, TEMP_NP_URI, TEST_SERVER, BOLD, END};
use crate::namespaces::{get_prefixes, NPX, NP_NS};

use base64;
use base64::{alphabet, engine, Engine as _};
use rsa::{
    pkcs8::DecodePrivateKey, pkcs8::EncodePublicKey, sha2::Digest, sha2::Sha256, Pkcs1v15Sign,
    RsaPrivateKey, RsaPublicKey,
};
use regex::Regex;
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::ns::{Namespace, rdf};
use sophia::api::quad::Quad;
use sophia::api::serializer::{QuadSerializer, Stringifier};
use sophia::api::source::QuadSource;
use sophia::api::term::{Term, TermKind, matcher::Any, matcher::GraphNameMatcher};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia::turtle::parser::{nq, trig};
use sophia::turtle::serializer::nq::NqSerializer;
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};
use std::error::Error;
use std::{fmt, str};

pub struct NpMetadata {
    pub np_url: Iri<String>,
    pub np_ns: Namespace<String>,
    pub head: Iri<String>,
    pub assertion: Iri<String>,
    pub prov: Iri<String>,
    pub pubinfo: Iri<String>,
    pub base_uri: String,
    pub separator_char: String,
    pub trusty_hash: String,
}

impl fmt::Display for NpMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}Nanopub URL:{} {}", BOLD, END, self.np_url)?;
        writeln!(f, "{}Nanopub Namespace:{} {}", BOLD, END, self.np_ns.to_string())?;
        writeln!(f, "{}Base URI:{} {}", BOLD, END, self.base_uri)?;
        writeln!(f, "{}Trusty Hash:{} {}", BOLD, END, self.trusty_hash)?;
        writeln!(f, "{}Assertion Graph:{} {}", BOLD, END, self.assertion)?;
        Ok(())
    }
}


/// A nanopublication object
#[derive(Default)]
pub struct Nanopub {
    rdf: String,
    // pub metadata: NpMetadata,
    pub trusty_hash: String,
    pub signature_hash: String,
    pub public_key: String,
    private_key: String,
    pub orcid: String,
    pub server_url: String,
    pub publish: bool, // false
    // dataset: LightDataset,
}
// https://docs.rs/sophia/0.5.3/sophia/dataset/inmem/index.html

#[derive(Debug)]
struct NanopubError(String);

impl Error for NanopubError {}

impl fmt::Display for NanopubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


fn extract_np_metadata(dataset: &LightDataset) -> Result<NpMetadata, NanopubError> {
    // Extract graphs URLs from a nanopub: nanopub URL, head, assertion, prov, pubinfo
    let np_ns: Namespace<&str> = Namespace::new(NP_NS).unwrap();
    let mut np_url: Option<String> = None;
    let mut head: Option<String> = None;
    let mut assertion: Option<String> = None;
    let mut prov: Option<String> = None;
    let mut pubinfo: Option<String> = None;

    // Extract nanopub URL and head graph
    for q in dataset.quads_matching(Any, [&rdf::type_], [np_ns.get("Nanopublication").unwrap()], Any) {
        if np_url.is_some() {
            return Err(NanopubError("The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string()));
        } else {
            np_url = Some(q.unwrap().s().iri().unwrap().to_string());
            head = Some(q.unwrap().g().unwrap().iri().unwrap().to_string());
        }
    }
    if !np_url.is_some() {
        return Err(NanopubError("The provided RDF does not contain a Nanopublication.".to_string()));
    }
    let np_iri: Iri<String> = Iri::new_unchecked(np_url.unwrap());
    let head_iri: Iri<String> = Iri::new_unchecked(head.unwrap());

    // Extract assertion, prov, pubinfo, and head graphs URLs
    for q in dataset.quads_matching([&np_iri], [np_ns.get("hasAssertion").unwrap()], Any, [Some(&head_iri)]) {
        assertion = Some(q.unwrap().o().iri().unwrap().to_string());
    }
    for q in dataset.quads_matching([&np_iri], [np_ns.get("hasProvenance").unwrap()], Any, [Some(&head_iri)]) {
        prov = Some(q.unwrap().o().iri().unwrap().to_string());
    }
    for q in dataset.quads_matching([&np_iri], [np_ns.get("hasPublicationInfo").unwrap()], Any, [Some(&head_iri)]) {
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
        separator_char = Some(caps.get(2).map_or("", |m| m.as_str()).to_string());
        // The third group captures everything after 'RA', if present.
        trusty_hash = Some(caps.get(3).map_or("", |m| m.as_str()).to_string());
    }

    // Get np namespace from the np URL (add # if not ending with / or #)
    let mut namespace: String = np_iri.to_string();
    if !namespace.ends_with('#') && !namespace.ends_with('/') {
        namespace.push('#');
    }
    // TODO: extract signature, algo, public key here too?

    Ok(NpMetadata {
        np_url: np_iri,
        np_ns: Namespace::new(namespace).unwrap(),
        head: head_iri,
        assertion: Iri::new_unchecked(assertion.unwrap()),
        prov: Iri::new_unchecked(prov.unwrap()),
        pubinfo: Iri::new_unchecked(pubinfo.unwrap()),
        base_uri: base_uri.unwrap(),
        separator_char: separator_char.unwrap(),
        trusty_hash: trusty_hash.unwrap(),
    })
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
    /// let np = Nanopub::new(
    ///     np_rdf.as_str(),
    ///     private_key,
    ///     "https://orcid.org/0000-0000-0000-0000",
    ///     None,
    ///     None,
    /// );
    /// ```

    pub fn new(
        rdf: &str,
        private_key: &str,
        orcid: &str,
        server_url: Option<&str>,
        publish: Option<&bool>,
    ) -> Result<Self, Box<dyn Error>> {
        // Self::default()
        openssl_probe::init_ssl_cert_env_vars();

        let tmp_ns = Namespace::new(TEMP_NP_NS)?;
        let npx: Namespace<&str> = Namespace::new(NPX)?;

        let mut dataset: LightDataset = trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse RDF");

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

        // Extract graph URLs from the nanopub (fails if np not valid)
        let np_meta = extract_np_metadata(&dataset).expect("The provided Nanopublication is not valid");
        println!("{}", np_meta);

        // TODO: check the np is valid and extract required metadata (baseuri/trusty_hash if there)
        // cf. utils.py extract_np_metadata(): baseuri, hash_fragment
        // 1. We should be able to detect if it is an unsigned np, and extract the dummy URI used
        // 2. If the np is incomplete we add the missing triples
        // We always replace the hashstr to " " when normalizing

        // Add triples about the signature in the pubinfo
        dataset.insert(
            np_meta.np_ns.get("sig")?,
            npx.get("hasPublicKey")?,
            &*pub_key_str,
            Some(&np_meta.pubinfo),
        )?;
        dataset.insert(
            np_meta.np_ns.get("sig")?,
            npx.get("hasAlgorithm")?,
            "RSA",
            Some(&np_meta.pubinfo),
        )?;
        dataset.insert(
            np_meta.np_ns.get("sig")?,
            npx.get("hasSignatureTarget")?,
            Iri::new_unchecked(TEMP_NP_URI),
            Some(&np_meta.pubinfo),
        )?;

        // Normalized nanopub nquads to a string
        let norm_quads = normalize_dataset(&dataset, Some(""), Some(""))
            .expect("Failed to normalise RDF before adding signature");
        // println!("      NORMED QUADS\n{}", norm_quads);

        // Generate signature using the private and normalized RDF
        let signature_vec = priv_key
            .sign(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes()),
            )
            .expect("Failed to sign nanopub");
        let signature_hash = engine::general_purpose::STANDARD.encode(signature_vec);

        // Add the signature to the pubinfo graph
        dataset.insert(
            np_meta.np_ns.get("sig")?,
            npx.get("hasSignature")?,
            &*signature_hash,
            Some(&np_meta.pubinfo),
        )?;

        // Generate TrustyURI
        let norm_quads_signed = normalize_dataset(&dataset, Some(""), Some(""))
            .expect("Failed to normalise RDF after adding signature");
        println!("NORMED QUADS AFTER SIGNING\n{}", norm_quads);

        let base64_engine = engine::GeneralPurpose::new(
            &alphabet::Alphabet::new(
                "-_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            )?,
            engine::GeneralPurposeConfig::new().with_encode_padding(false),
        );
        let trusty_hash = format!(
            "RA{}",
            base64_engine.encode(Sha256::digest(norm_quads_signed.as_bytes()))
        );

        // Now we serialize the Nanopub to RDF Trig format
        // Prepare the trig serializer
        let prefixes = get_prefixes();
        let trig_config = TrigConfig::new()
            .with_pretty(true)
            .with_prefix_map(&prefixes[..]);
        let mut trig_stringifier = TrigSerializer::new_stringifier_with_config(trig_config);
        // TODO: replace all }GRAPH by }\n ? Or fix pretty code

        // Return the Nanopub object
        Ok(Self {
            rdf: trig_stringifier
                .serialize_dataset(&dataset)
                .expect("Unable to serialize dataset to trig")
                .to_string(),
            // rdf: nq_stringifier.serialize_dataset(&mut dataset)?.to_string(),
            // dataset: dataset,
            trusty_hash,
            signature_hash,
            public_key: pub_key_str,
            private_key: private_key.to_string(),
            orcid: orcid.to_string(),
            server_url: if let Some(server_url) = server_url {
                server_url.to_string()
            } else {
                TEST_SERVER.to_string()
            },
            publish: if let Some(publish) = publish {
                *publish
            } else {
                false
            },
        })
    }

    // - preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear;
    // this includes the signature part, except the triple that is stating the actual signature
    // - preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
    // - Signature is calculated on this normalized representation
    // - Signature triple is added
    // - Trusty URI code is calculated on normalized representation that includes signature
    // - Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub

    /// Returns the RDF of the nanopub
    pub fn get_rdf(&self) -> String {
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
        writeln!(f, "\n{}Nanopublication RDF:{} \n{}", BOLD, END, self.rdf)?;
        writeln!(f, "{}ORCID:{} {}", BOLD, END, self.orcid)?;
        writeln!(f, "{}Public key:{} {}", BOLD, END, self.public_key)?;
        writeln!(f, "{}Private key:{} {}", BOLD, END, self.private_key)?;
        writeln!(f, "{}Trusty hash:{} {}", BOLD, END, self.trusty_hash)?;
        writeln!(f, "{}Signature hash:{} {}", BOLD, END, self.signature_hash)?;
        writeln!(f, "{}Publish:{} {}", BOLD, END, self.publish)?;
        writeln!(f, "{}Server URL:{} {}", BOLD, END, self.server_url)?;
        // for t in self {
        //     info!(f, "{}", t)?;
        // }
        Ok(())
    }
}

fn normalize_key(key: &str) -> Result<String, Box<dyn Error>> {
    let mut normed_key = key.trim();
    //println!("Normalize");
    let rm_prefix = "-----BEGIN PUBLIC KEY-----";
    if normed_key.starts_with(rm_prefix) {
        normed_key = &normed_key[rm_prefix.len()..].trim();
    }
    // let rm_suffix = format!("-----BEGIN {} KEY-----", key_type);
    let rm_suffix = "-----END PUBLIC KEY-----";
    if normed_key.ends_with(rm_suffix) {
        normed_key = &normed_key[..normed_key.len() - rm_suffix.len() - 1].trim();
    }
    // normed_key = str::replace(normed_key, "\n", "");
    // TODO: split on \n and join? We might need to move to String type anyway
    // normed_key = normed_key.split("\n").collect().join("");
    // normed_key = normed_key.replace("\n", "");
    // key = key.trim();
    Ok(normed_key.replace('\n', ""))
}

/// Returns all the quads contained by the nanopub.
fn normalize_dataset(
    dataset: &LightDataset,
    _baseuri: Option<&str>,
    _hashstr: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    // let baseuri = baseuri.unwrap_or("http://purl.org/np/");
    // let hashstr = hashstr.unwrap_or(" ");

    // baseuri=str(dummy_namespace),
    // hashstr=" "
    // Generate the temp np URI based on the temp NS (remove trailing # and /)
    let mut temp_np_uri = TEMP_NP_NS;
    let escape_last_chars = ['#', '/'];
    for escape_char in escape_last_chars {
        let last_char = temp_np_uri.chars().last().unwrap();
        if last_char == escape_char {
            temp_np_uri = &temp_np_uri[..temp_np_uri.len() - 1];
            // temp_np_uri = temp_np_uri.substring(0, temp_np_uri.len() - 1);
        }
    }

    let mut nq_stringifier = NqSerializer::new_stringifier();

    // Serialize the RDF as nquad string, generate a list from the lines, and sort alphabetically
    // TODO: better ordering with comparator https://stackoverflow.com/questions/46512227/sort-a-vector-with-a-comparator-which-changes-its-behavior-dynamically
    let nquads_str = nq_stringifier
        .serialize_dataset(dataset)
        .expect("Unable to serialize provided RDF")
        .to_string();
    let split = nquads_str.split('\n');
    let mut quads_sorted: Vec<&str> = split.collect();
    quads_sorted.sort_by_key(|name| name.to_lowercase());
    let mut norm_quads: String = "".to_owned();

    // Normalize the quads like done for the trusty URI
    // https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15
    for quad in quads_sorted {
        // println!("{}", quad);
        let quad_dataset: LightDataset = nq::parse_str(quad)
            .collect_quads()
            .expect("Unable to parse quad");

        for q in quad_dataset.quads() {
            let q = q?;
            let mut s = q.s().iri().unwrap().to_string();
            let p = q.p().iri().unwrap().to_string();

            if q.s().kind() == TermKind::Iri {
                // Replace temp np URIs with normalized URI in subject URIs
                s = s.replace(TEMP_NP_NS, NORMALIZED_NS);
                s = s.replace(temp_np_uri, NORMALIZED_URI);
            }

            // let mut o: String = q.o().iri().unwrap().to_string();
            let mut o: String;
            if q.o().kind() == TermKind::Iri {
                // Replace temp np URIs with normalized URI in object URIs
                o = q.o().iri().unwrap().to_string();
                o = o.replace(TEMP_NP_NS, NORMALIZED_NS);
                o = o.replace(temp_np_uri, NORMALIZED_URI);
            } else {
                // If lang tag, add @en or @fr in front of the object
                o = q.o().lexical_form().unwrap().to_string();
                let lang = q.o().language_tag();
                if let Some(lang) = lang {
                    let lang_tag = ['@'.to_string(), lang.to_string()].join("");
                    o = [lang_tag, o].join(" ");
                } else {
                    // If no lang type, we add the datatype
                    let datatype = q.o().datatype();
                    if let Some(datatype) = datatype {
                        let datatype_tag =
                            ["^".to_string(), datatype.iri().unwrap().to_string()].join("");
                        o = [datatype_tag, o].join(" ");
                    } else {
                        o = ["^http://www.w3.org/2001/XMLSchema#string".to_string(), o].join(" ");
                    }
                }
            }

            let g_term = q.g();
            if let Some(g_term) = g_term {
                let mut g = g_term.iri().unwrap().to_string();
                if g_term.kind() == TermKind::Iri {
                    // Replace temp np URIs with normalized URI in URIs
                    g = g.replace(TEMP_NP_NS, NORMALIZED_NS);
                    g = g.replace(temp_np_uri, NORMALIZED_URI);
                }
                norm_quads = [norm_quads, s, p, o, g].join("\n");
                // println!("{}", g);
            }
        }
    }
    //println!("      NORMED QUADS in normalize");
    //println!("{}", norm_quads);
    Ok(norm_quads)
    // let iter = self.spog.iter();
    // Iter {
    //     dataset: self,
    //     inner: iter,
    // }
}
