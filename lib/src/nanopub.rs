// #![extern crate sophia];
// #extern crate sophia;

use crate::constants::TEST_SERVER;

use std::error::Error;
use std::fmt;

use sophia::dataset::{inmem::LightDataset, *};
use sophia::ns::Namespace;
use sophia::parser::{nq, trig};
use sophia::quad::stream::QuadSource;
use sophia::quad::Quad;
use sophia::serializer::nq::NqSerializer;
use sophia::serializer::*;
use sophia::term::TTerm;
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

        let ex = Namespace::new("http://example.org/")?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;

        let mut dataset: LightDataset = trig::parse_str(rdf).collect_quads()?;

        dataset.insert(
            &ex.get("bob")?,
            &foaf.get("knows")?,
            &ex.get("alice")?,
            Some(&ex.get("bob")?),
        )?;
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
                let s = q.s().to_string();
                let p = q.p().to_string();
                let mut o = q.o().to_string();
                // If lang tag, add @en or @fr in front of the object
                let lang = q.o().language();
                if let Some(lang) = lang {
                    let lang_tag = ['@'.to_string(), lang.to_string()].join("");
                    o = [lang_tag, o].join(" ");
                }

                let g = q.g();
                if let Some(g) = g {
                    norm_quads = [norm_quads, s, p, o, g.to_string()].join("\n");
                    // println!("{}", g);
                }
            }

            // let item = self.quads().oks().position(|q| {
            //     term_eq(q.s(), s) && term_eq(q.p(), p) && term_eq(q.o(), o) && same_graph_name(g, q.g())
            // });
        }
        println!("NORMED QUADDDDDS");
        println!("{}", norm_quads);

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

        Ok(Self {
            rdf: nq_stringifier.serialize_dataset(&mut dataset)?.to_string(),
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
