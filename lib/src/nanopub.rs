
use std::fmt;

// # extern crate sophia;
use sophia::graph::{inmem::FastGraph, *};
use sophia::ns::Namespace;
use sophia::parser::turtle;
use sophia::serializer::nt::NtSerializer;
// use sophia::serializer::nq::QuadSerializer;
use sophia::serializer::*;
use sophia::triple::stream::TripleSource;


/// A nanopublication object
#[derive(Debug, Default)]
pub struct Nanopub {
    pub rdf: String,
    // pubkey
    // privkey
    // orcid
    // server_url
    // publish: bool, // false
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
    /// use nanopub_rs::nanopub::Nanopub;
    /// let np = Nanopub::new("<http://s> <http://p> <http://o> .");
    /// ```
    pub fn new(rdf: &str) -> Self {
        // Self::default()

        // let example = r#"
        //         @prefix : <http://example.org/>.
        //         @prefix foaf: <http://xmlns.com/foaf/0.1/>.

        //         :alice foaf:name "Alice";
        //             foaf:mbox <mailto:alice@work.example> .

        //         :bob foaf:name "Bob".
        //         "#;
        // let mut graph: FastGraph = turtle::parse_str(example).collect_triples();

        // let ex = Namespace::new("http://example.org/");
        // let foaf = Namespace::new("http://xmlns.com/foaf/0.1/");
        // graph.insert(&ex.get("bob"), &foaf.get("knows"), &ex.get("alice"));

        // let mut nt_stringifier = NtSerializer::new_stringifier();
        // let example2 = nt_stringifier.serialize_graph(&mut graph).as_str();
        // println!("The resulting graph\n{}", example2);
        // Ok(())

        Self {
            rdf: rdf.to_string()
            // rdf: if let Some(rdf) = rdf {
            //     rdf.to_string()
            // } else {
            //     "Default toast".to_string()
            // }
        }
    }
    // pub fn new(rdf: Option<&str>) -> Self {
    //     // Self::default()
    //     Self {
    //         rdf: if let Some(rdf) = rdf {
    //             rdf.to_string()
    //         } else {
    //             "Default toast".to_string()
    //         }
    //     }
    // }

    /// Returns the RDF of the nanopub
    pub fn get_rdf(&self) -> String {
        self.parse_rdf();
        self.rdf.clone()
    }


    fn parse_rdf(&self) -> Result<(), Box<dyn std::error::Error>> {
        // let example = r#"
        //         @prefix this: <http://purl.org/np/RA5IWUwPmx_chibRuDOMfby6Sz8I0n76xnB3BiAm6ZP74> .
        //         @prefix sub: <http://purl.org/np/RA5IWUwPmx_chibRuDOMfby6Sz8I0n76xnB3BiAm6ZP74#> .
        //         @prefix drugbank: <http://identifiers.org/drugbank/> .
        //         @prefix np: <http://www.nanopub.org/nschema#> .
        //         @prefix pav: <http://purl.org/pav/> .
        //         @prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
        //         @prefix infores: <https://w3id.org/biolink/infores/> .
        //         @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
        //         @prefix dcterms: <http://purl.org/dc/terms/> .
        //         @prefix orcid: <https://orcid.org/> .
        //         @prefix biolink: <https://w3id.org/biolink/vocab/> .
        //         @prefix pmid: <http://www.ncbi.nlm.nih.gov/pubmed/> .
        //         @prefix prov: <http://www.w3.org/ns/prov#> .
        //         @prefix npx: <http://purl.org/nanopub/x/> .

        //         sub:Head {
        //         this: np:hasAssertion sub:assertion;
        //             np:hasProvenance sub:provenance;
        //             np:hasPublicationInfo sub:pubInfo;
        //             a np:Nanopublication .
        //         }

        //         sub:assertion {
        //         drugbank:DB10771 a biolink:Drug;
        //             biolink:category biolink:Drug .

        //         <http://purl.obolibrary.org/obo/OMIM_130000> a biolink:Disease;
        //             biolink:category biolink:Disease .

        //         sub:association rdf:object <http://purl.obolibrary.org/obo/OMIM_130000>;
        //             rdf:predicate biolink:treats;
        //             rdf:subject drugbank:DB10771;
        //             a biolink:ChemicalToDiseaseOrPhenotypicFeatureAssociation;
        //             biolink:aggregator_knowledge_source infores:knowledge-collaboratory;
        //             biolink:category biolink:ChemicalToDiseaseOrPhenotypicFeatureAssociation;
        //             biolink:publications pmid:PMC3159979;
        //             biolink:relation <http://purl.obolibrary.org/obo/RO_0002606> .
        //         }

        //         sub:provenance {
        //         sub:assertion dcterms:created "2020-09-21T00:00:00"^^xsd:dateTime;
        //             prov:wasAttributedTo orcid:0000-0001-7769-4272 .
        //         }

        //         sub:pubInfo {
        //         sub:sig npx:hasAlgorithm "RSA";
        //             npx:hasPublicKey "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCR9fz0fKCdWOWC+pxhkQhEM/ppbdIYe5TLSdj+lJzSlv9mYBaPgrzVezSwwbmhlHBPDZa4/vHycU315BdmUGq+pXllp9+rWFfrb+kBJwhZjpG6BeyyXBsRFz4jmQVxl/ZYHilQTh/XalYzKkEAyTiEMPee4Kz61PaWOKH24CsnOQIDAQAB";
        //             npx:hasSignatureTarget this: .

        //         this: prov:generatedAtTime "2022-09-16T18:18:46.871040"^^xsd:dateTime;
        //             prov:wasAttributedTo orcid:0000-0002-1501-1082 .
        //         }

        //         "#;
        let example = r#"
            @prefix : <http://example.org/>.
            @prefix foaf: <http://xmlns.com/foaf/0.1/>.

            :alice foaf:name "Alice";
                foaf:mbox <mailto:alice@work.example> .

            :bob foaf:name "Bob".
            "#;
        let mut graph: FastGraph = turtle::parse_str(example).collect_triples()?;

        let ex = Namespace::new("http://example.org/")?;
        let foaf = Namespace::new("http://xmlns.com/foaf/0.1/")?;
        graph.insert(&ex.get("bob")?, &foaf.get("knows")?, &ex.get("alice")?)?;

        let mut nt_stringifier = NtSerializer::new_stringifier();
        // let mut nt_stringifier = QuadSerializer::new_stringifier();
        let example2 = nt_stringifier.serialize_graph(&mut graph)?.as_str();
        println!("The resulting graph\n{}", example2);

        Ok(())
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