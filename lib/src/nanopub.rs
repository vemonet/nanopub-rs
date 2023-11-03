use crate::constants::{NORMALIZED_NS, NORMALIZED_URI, TEMP_NP_NS, TEMP_NP_URI, TEST_SERVER};
use crate::namespaces::{get_prefixes, NPX};

use base64;
use std::error::Error;
use std::{fmt, str};
use log;
use rsa::{Pkcs1v15Sign, RsaPublicKey, RsaPrivateKey, pkcs8::DecodePrivateKey, pkcs8::EncodePublicKey, sha2::Sha256, sha2::Digest};
use sophia::dataset::{inmem::LightDataset, *};
use sophia::ns::Namespace;
use sophia::parser::{nq, trig};
use sophia::quad::stream::QuadSource;
use sophia::quad::Quad;
use sophia::serializer::nq::NqSerializer;
use sophia::serializer::trig::{TrigConfig, TrigSerializer};
use sophia::serializer::*;
use sophia::term::{StaticTerm, TTerm, TermKind};
use sophia::term::literal::convert::AsLiteral;
// use sophia::serializer::turtle::TrigSerializer;
// use sophia::term::matcher::TermMatcher;
// use sophia::iri::AsIri;
// use sophia::term::iri::convert::AsLiteral;
// use sophia::term::*;
// use sophia::graph::{inmem::FastGraph, *};
// use sophia::triple::stream::TripleSource;
// use sophia::serializer::nt::NtSerializer;
// use sophia::parser::turtle;

/// A nanopublication object
#[derive(Default)]
pub struct Nanopub {
    rdf: String,
    // dataset: LightDataset,
    public_key: String,
    private_key: String,
    orcid: String,
    server_url: String,
    publish: bool, // false
}
// https://docs.rs/sophia/0.5.3/sophia/dataset/inmem/index.html

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
    /// use nanopub_rs::nanopub::Nanopub;
    /// let private_key = r#"-----BEGIN PRIVATE KEY-----
    /// MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=
    /// -----END PRIVATE KEY-----"#;
    /// let np = Nanopub::new(
    ///     "<http://s> <http://p> <http://o> <http://g> .",
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
        log::trace!("Starting nanopub creation ....");
        openssl_probe::init_ssl_cert_env_vars();
      
        let tmp_ns = Namespace::new(TEMP_NP_NS)?;
        let npx: Namespace<&str> = Namespace::new(NPX)?;

        let mut dataset: LightDataset = trig::parse_str(rdf).collect_quads()
            .ok()
            .expect("Failed to parse RDF");

        let norm_quads = normalize_dataset(&dataset)
            .ok()
            .expect("Failed to normalise RDF");
        // println!("      NORMED QUADS");
        // println!("{}", norm_quads);

        // DEPRECATED: Get the keypair with OpenSSL
        // let keypair = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
        // let keypair = PKey::from_rsa(keypair).unwrap();

        // openssl_probe::init_ssl_cert_env_vars();
        // println!("GETTING READY");
        // let priv_key = RsaPrivateKey::from_pkcs1_pem(private_key)?;

        // let priv_key_bytes = base64::decode(private_key)?;
        let priv_key_bytes = base64::decode(private_key).expect("Failed to decode base64 private key");
        let priv_key = RsaPrivateKey::from_pkcs8_der(&priv_key_bytes).expect("Failed to parse RSA private key");
        // let priv_key = RsaPrivateKey::from_pkcs1_der(private_key).expect("Failed to parse RSA private key");

        log::info!("private_key GOOD");

        let public_key = RsaPublicKey::from(&priv_key);
        // let pub_key_str = normalize_key(&ToRsaPublicKey::to_pkcs1_pem(&public_key).unwrap()).unwrap();
        // let pub_key_str = normalize_key(&ToRsaPublicKey::to_pkcs1_pem(&public_key).unwrap()).unwrap();
        let pub_key_str = normalize_key(
            &RsaPublicKey::to_public_key_pem(&public_key, rsa::pkcs8::LineEnding::LF).unwrap(),
        )
        .unwrap();

        log::info!("Public key: {:?}", pub_key_str);

        // println!("Public Key:\n{}", public_key_string);
        // let pub_key_vec: Vec<u8> = keypair.public_key_to_pem().unwrap();
        // let public_key = normalize_key(str::from_utf8(pub_key_vec.as_slice()).unwrap()).unwrap();
        // println!("PUBLIC KEY: {}", public_key);

        // Add triples about the signature in the pubinfo
        dataset.insert(
            &tmp_ns.get("sig")?,
            &npx.get("hasPublicKey")?,
            &pub_key_str.as_literal(),
            Some(&tmp_ns.get("pubinfo")?),
        )?;
        dataset.insert(
            &tmp_ns.get("sig")?,
            &npx.get("hasAlgorithm")?,
            &"RSA".as_literal(),
            Some(&tmp_ns.get("pubinfo")?),
        )?;
        dataset.insert(
            &tmp_ns.get("sig")?,
            &npx.get("hasSignatureTarget")?,
            &StaticTerm::new_iri(TEMP_NP_URI)?,
            Some(&tmp_ns.get("pubinfo")?),
        )?;

        // Sign the data
        // let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        // signer.update(norm_quads.as_bytes()).unwrap();
        // let signature = signer.sign_to_vec().unwrap();
        // let signature_base64 = base64::encode(signature);
        // println!("\nSignature:\n{}\n", signature_base64);

        // let signature = priv_key
        //     .sign(PaddingScheme::PKCS1v15Sign { hash: () }, norm_quads.as_bytes())
        //     .expect("Failed to sign nanopub");

        let signature_vec = priv_key
            .sign(
                Pkcs1v15Sign::new::<Sha256>(),
                &Sha256::digest(norm_quads.as_bytes().to_vec()),
            )
            .expect("Failed to sign nanopub");
        let signature = base64::encode(signature_vec);
        log::info!("Signature: {:?}", signature);

        // Add signature to the dataset
        // let signature_lit = StaticTerm::new_literal_dt(signature_str, xsd::string);
        // dataset.insert(
        //     &tmp_ns.get("sig")?,
        //     &npx.get("hasSignature")?,
        //     &signature_lit?,
        //     // &signature_base64.to_literal()?,
        //     Some(&tmp_ns.get("pubinfo")?),
        // )?;

        // Generate TrustyURI
        // TODO: add the signature to pubinfo graph, and re-sign
        // https://github.com/fair-workflows/nanopub/blob/main/nanopub/trustyuri/rdf/RdfHasher.py
        // In python for trusty URI: return re.sub(r'=', '', base64.b64encode(s, b'-_').decode('utf-8'))
        // In java: String publicKeyString = DatatypeConverter.printBase64Binary(c.getKey().getPublic().getEncoded()).replaceAll("\\s", "");
        let sha256_str = base64::encode_config(Sha256::digest(norm_quads.as_bytes().to_vec()), base64::URL_SAFE);
        let trusty_str = format!("RA{}", sha256_str);
        println!("Trusty URI artefact:\n{}\n", trusty_str);

        // for quad in
        // dataset.quads().for_each_quad(|q| {
        //     println!("{}", q);
        // })?;
        //  {
        //     for elem in quad.iter() {
        //         println!("{}", elem);
        //     }
        // }

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
                .serialize_dataset(&mut dataset)
                .ok()
                .expect("Unable to serialize dataset to trig")
                .to_string(),
            // rdf: nq_stringifier.serialize_dataset(&mut dataset)?.to_string(),
            // dataset: dataset,
            public_key: pub_key_str,
            private_key: private_key.to_string(),
            orcid: orcid.to_string(),
            server_url: if let Some(server_url) = server_url {
                server_url.to_string()
            } else {
                TEST_SERVER.to_string()
            },
            publish: if let Some(publish) = publish {
                publish.clone()
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
        // for t in self {
        //     info!(f, "{}", t)?;
        // }
        writeln!(f, "RDF to publish: \n{}", self.rdf)?;
        writeln!(f, "ORCID: {}", self.orcid)?;
        writeln!(f, "Public key: {}", self.public_key)?;
        writeln!(f, "Private key: {}", self.private_key)?;
        writeln!(f, "Publish: {}", self.publish)?;
        writeln!(f, "Server URL: {}", self.server_url)?;
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
    Ok(normed_key.replace("\n", ""))
}

/// Returns all the quads contained by the nanopub.
fn normalize_dataset(dataset: &LightDataset) -> Result<String, Box<dyn Error>> {
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
    let nquads_str = nq_stringifier.serialize_dataset(dataset)
        .ok()
        .expect("Unable to serialize provided RDF").to_string();
    let split = nquads_str.split("\n");
    let mut quads_sorted: Vec<&str> = split.collect();
    quads_sorted.sort_by_key(|name| name.to_lowercase());
    let mut norm_quads: String = "".to_owned();

    // Normalize the quads like done for the trusty URI
    // // https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15
    for quad in quads_sorted {
        // println!("{}", quad);
        let quad_dataset: LightDataset = nq::parse_str(quad).collect_quads().ok().expect("Unable to parse quad");

        for q in quad_dataset.quads() {
            let q = q?;
            let mut s = q.s().value().to_string();
            let p = q.p().value().to_string();
            let mut o: String = q.o().value().to_string();

            if q.s().kind() == TermKind::Iri {
                // Replace temp np URIs with normalized URI in subject URIs
                s = s.replace(TEMP_NP_NS, NORMALIZED_NS);
                s = s.replace(temp_np_uri, NORMALIZED_URI);
            }

            if q.o().kind() == TermKind::Iri {
                // Replace temp np URIs with normalized URI in object URIs
                o = o.replace(TEMP_NP_NS, NORMALIZED_NS);
                o = o.replace(temp_np_uri, NORMALIZED_URI);
            } else {
                // If lang tag, add @en or @fr in front of the object
                let lang = q.o().language();
                if let Some(lang) = lang {
                    let lang_tag = ['@'.to_string(), lang.to_string()].join("");
                    o = [lang_tag, o].join(" ");
                } else {
                    // If no lang type, we add the datatype
                    let datatype = q.o().datatype();
                    if let Some(datatype) = datatype {
                        let datatype_tag = ["^".to_string(), datatype.value().to_string()].join("");
                        o = [datatype_tag, o].join(" ");
                    } else {
                        o = ["^http://www.w3.org/2001/XMLSchema#string".to_string(), o].join(" ");
                    }
                }
            }

            let g_term = q.g();
            if let Some(g_term) = g_term {
                let mut g = g_term.value().to_string();
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
