// #![extern crate sophia];
// #extern crate sophia;

use crate::constants::{NORMALIZED_NS, NORMALIZED_URI, TEMP_NP_NS, TEMP_NP_URI, TEST_SERVER};
use crate::namespaces::NPX;

use base64;
use std::error::Error;
use std::fmt;

// use openssl::sign::{Signer, Verifier};
use openssl::rsa::Rsa;
use openssl::sign::Signer;
// use openssl::rsa::{RsaPrivateKeyBuilder};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use sophia::dataset::{inmem::LightDataset, *};
use sophia::iri::Iri;
use sophia::ns::{xsd, Namespace};
use sophia::parser::{nq, trig};
use sophia::quad::stream::QuadSource;
use sophia::quad::Quad;
use sophia::serializer::nq::NqSerializer;
use sophia::serializer::trig::{TrigConfig, TrigSerializer};
// use sophia::parser::TripleParser;
use sophia::prefix::Prefix;
// use sophia::serializer::turtle::TrigSerializer;
use sophia::serializer::*;
use sophia::term::{StaticTerm, TTerm, TermKind};
// use sophia::term::matcher::TermMatcher;

// use sophia::iri::AsIri;
// use sophia::term::iri::convert::AsLiteral;
use sophia::term::literal::convert::AsLiteral;
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
    /// let np = Nanopub::new(
    ///     "<http://s> <http://p> <http://o> .",
    ///     "PUBKEY",
    ///     "PRIVATE_KEY",
    ///     "https://orcid.org/0000-0000-0000-0000",
    ///     None,
    ///     None,
    /// );
    /// ```
    pub fn new(
        rdf: &str,
        public_key: &str,
        private_key: &str,
        orcid: &str,
        server_url: Option<&str>,
        publish: Option<&bool>,
    ) -> Result<Self, Box<dyn Error>> {
        // Self::default()

        let tmp_ns = Namespace::new(TEMP_NP_NS)?;
        // let tmp_uri = Namespace::new(TEMP_NP_URI)?;
        let npx: Namespace<&str> = Namespace::new(NPX)?;

        let mut dataset: LightDataset = trig::parse_str(rdf).collect_quads()?;

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

        // Add triples about the signature in the pubinfo
        // let base64_pubkey = base64::encode(public_key);
        // sub:sig npx:hasAlgorithm "RSA";
        //             npx:hasPublicKey "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCR9fz0fKCdWOWC+pxhkQhEM/ppbdIYe5TLSdj+lJzSlv9mYBaPgrzVezSwwbmhlHBPDZa4/vHycU315BdmUGq+pXllp9+rWFfrb+kBJwhZjpG6BeyyXBsRFz4jmQVxl/ZYHilQTh/XalYzKkEAyTiEMPee4Kz61PaWOKH24CsnOQIDAQAB";
        //             npx:hasSignatureTarget this: .
        dataset.insert(
            &tmp_ns.get("sig")?,
            &npx.get("hasPublicKey")?,
            &public_key.as_literal(),
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

        // START NORMALIZE DATASET
        let mut nq_stringifier = NqSerializer::new_stringifier();

        // TODO: add all statements required for nanopubs (signature, algo, etc)

        // Serialize the RDF as nquad string, generate a list from the lines, and sort alphabetically
        let nquads_str = nq_stringifier.serialize_dataset(&mut dataset)?.to_string();
        let split = nquads_str.split("\n");
        let mut quads_sorted: Vec<&str> = split.collect();
        quads_sorted.sort_by_key(|name| name.to_lowercase());
        let mut norm_quads: String = "".to_owned();

        // Normalize the quads like done for the trusty URI
        // // https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15
        for quad in quads_sorted {
            println!("{}", quad);
            let quad_dataset: LightDataset = nq::parse_str(quad).collect_quads()?;

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
                            let datatype_tag =
                                ["^".to_string(), datatype.value().to_string()].join("");
                            o = [datatype_tag, o].join(" ");
                        } else {
                            o = ["^http://www.w3.org/2001/XMLSchema#string".to_string(), o]
                                .join(" ");
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
        println!("      NORMED QUADS");
        println!("{}", norm_quads);
        // END NORMALIZE DATASET

        // Get the keypair
        let keypair = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
        let keypair = PKey::from_rsa(keypair).unwrap();

        // In python for trusty URI: return re.sub(r'=', '', base64.b64encode(s, b'-_').decode('utf-8'))
        // In java: String publicKeyString = DatatypeConverter.printBase64Binary(c.getKey().getPublic().getEncoded()).replaceAll("\\s", "");
        // Sign the data
        let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        signer.update(norm_quads.as_bytes()).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        let signature_base64 = base64::encode(signature);
        println!("\nSignature:\n{}\n", signature_base64);

        // let signature_str = signature_base64.as_str();

        // let signature_lit = StaticTerm::new_literal_dt(signature_str, xsd::string);

        // dataset.insert(
        //     &tmp_ns.get("sig")?,
        //     &npx.get("hasSignature")?,
        //     &signature_lit?,
        //     // &signature_base64.to_literal()?,
        //     Some(&tmp_ns.get("pubinfo")?),
        // )?;

        // TODO: add the signature and re sign
        let trusty_str = base64::encode_config(norm_quads, base64::URL_SAFE);
        println!("Trusty URI artefact:\n{}\n", trusty_str);

        // https://stackoverflow.com/questions/73716046/how-to-display-an-openssl-signature
        // println!("{}", signature.to_);

        // TODO: Sign the file with the private_key
        // Add signature to the graph
        // Generate trusty-uri

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
        let prefixes = [
            (
                Prefix::new_unchecked("rdf"),
                Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
            ),
            (
                Prefix::new_unchecked("rdfs"),
                Iri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#"),
            ),
            (
                Prefix::new_unchecked("xsd"),
                Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#"),
            ),
            (
                Prefix::new_unchecked("schema"),
                Iri::new_unchecked("http://schema.org/"),
            ),
            (
                Prefix::new_unchecked("foaf"),
                Iri::new_unchecked("http://xmlns.com/foaf/0.1/"),
            ),
            (
                Prefix::new_unchecked("biolink"),
                Iri::new_unchecked("https://w3id.org/biolink/vocab/"),
            ),
            (
                Prefix::new_unchecked("np"),
                Iri::new_unchecked("http://www.nanopub.org/nschema#"),
            ),
            (
                Prefix::new_unchecked("prov"),
                Iri::new_unchecked("http://www.w3.org/ns/prov#"),
            ),
            (
                Prefix::new_unchecked("npx"),
                Iri::new_unchecked("http://purl.org/nanopub/x/"),
            ),
            (
                Prefix::new_unchecked("nptemp"),
                Iri::new_unchecked("http://purl.org/nanopub/temp/mynanopub#"),
            ),
        ];
        // TrigConfig
        let trig_config = TrigConfig::new()
            .with_pretty(true)
            .with_prefix_map(&prefixes[..]);
        let mut trig_stringifier = TrigSerializer::new_stringifier_with_config(trig_config);
        // TODO: replace all }GRAPH by }\n ? Or fix pretty code

        Ok(Self {
            rdf: trig_stringifier
                .serialize_dataset(&mut dataset)?
                .to_string(),
            // rdf: nq_stringifier.serialize_dataset(&mut dataset)?.to_string(),
            // dataset: dataset,
            public_key: public_key.to_string(),
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
        //     writeln!(f, "{}", t)?;
        // }
        writeln!(f, "RDF: {}", self.rdf)?;
        writeln!(f, "ORCID: {}", self.orcid)?;
        writeln!(f, "Public key: {}", self.public_key)?;
        writeln!(f, "Private key: {}", self.private_key)?;
        writeln!(f, "Publish: {}", self.publish)?;
        writeln!(f, "Server URL: {}", self.server_url)?;
        Ok(())
    }
}
