// #![extern crate sophia];
// extern crate sophia;

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
#[derive(Debug, Default)]
pub struct Nanopub {
    pub rdf: String,
    // graph
    // pubkey
    // privkey
    // orcid
    // server_url
    // publish: bool, // false
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
    pub fn new(rdf: &str) -> Result<Self, Box<dyn Error>> {
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
            // graph: &mut graph,
        })


        // let mut graph: FastGraph = turtle::parse_str(rdf).collect_triples()?;

        // let mut nt_stringifier = NtSerializer::new_stringifier();

        // graph.insert(
        //     &ex.get("bob")?,
        //     &foaf.get("knows")?,
        //     &ex.get("alice")?,
        // )?;

        // Ok( Self {
        //     rdf: nt_stringifier.serialize_graph(&mut graph)?.to_string(),
        //     // graph: &mut graph,
        // })



        // Self {
        //     rdf: if let Some(rdf) = rdf {
        //         rdf.to_string()
        //     } else {
        //         "Default toast".to_string()
        //     }
        // }
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
        Ok(())
    }
}
