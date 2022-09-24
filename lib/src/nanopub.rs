// #![extern crate sophia];
// extern crate sophia;

// use crate::nanopub::co::{Decoder, EncodedQuad, EncodedTerm};

use crate::constants::{TEST_SERVER};

use std::fmt;
use std::error::Error;

use sophia::dataset::{inmem::FastDataset, *};
use sophia::ns::Namespace;
use sophia::parser::trig;
use sophia::quad::stream::QuadSource;
use sophia::serializer::nq::NqSerializer;
use sophia::serializer::*;
// use sophia::graph::{inmem::FastGraph, *};
// use sophia::triple::stream::TripleSource;
// use sophia::serializer::nt::NtSerializer;
// use sophia::parser::turtle;

/// A nanopublication object
#[derive(Default)]
pub struct Nanopub {
    rdf: String,
    // dataset: FastDataset,
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
    /// let np = Nanopub::new("<http://s> <http://p> <http://o> .");
    /// ```
    pub fn new(
        rdf: &str, public_key: &str, private_key: &str, orcid: &str,
        server_url: Option<&str>, publish: Option<&bool>
    ) -> Result<Self, Box<dyn Error>> {
        // Self::default()

        let ex = Namespace::new("http://example.org/")?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;

        let mut dataset: FastDataset = trig::parse_str(rdf).collect_quads()?;

        dataset.insert(
            &ex.get("bob")?,
            &foaf.get("knows")?,
            &ex.get("alice")?,
            Some(&ex.get("bob")?),
        )?;

        let mut nq_stringifier = NqSerializer::new_stringifier();

        // println!("The resulting graph\n{}", example2);

        Ok( Self {
            rdf: nq_stringifier.serialize_dataset(&mut dataset)?.to_string(),
            // dataset: dataset,
            public_key: public_key.to_string(),
            private_key: private_key.to_string(),
            orcid: orcid.to_string(),
            server_url: if let Some(server_url) = server_url {
                server_url.to_string()
            } else{
                TEST_SERVER.to_string()
            },
            publish: if let Some(publish) = publish {
                publish.clone()
            } else {
                false
            }
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
