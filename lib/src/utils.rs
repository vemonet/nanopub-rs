use sophia::api::source::{QuadSource as _, TripleSource as _};
use sophia::api::{ns::Namespace, prefix::Prefix};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia::turtle::parser::{nq, trig};
use sophia::{jsonld, xml};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

/// Return a Nanopub server, the main one or one picked randomly from the list of available servers
pub fn get_np_server(random: bool) -> &'static str {
    if !random {
        return LIST_SERVERS[0];
    }
    // Use time to generate a pseudo-random number, to avoid installing the rand crate
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let millis = since_the_epoch.as_millis();
    // Use the milliseconds to generate an index
    let index = (millis as usize) % LIST_SERVERS.len();
    LIST_SERVERS[index]
}

pub fn parse_rdf(rdf: &str) -> Result<LightDataset, NpError> {
    let dataset = if rdf.starts_with("{") || rdf.starts_with("[") {
        jsonld::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse JSON-LD RDF")
    // } else if rdf.starts_with("<?xml") {
    //     xml::parser::parse_str(rdf)
    //         .collect_quads()
    //         .expect("Failed to parse XML RDF")
    } else if rdf.lines().all(|line| line.split_whitespace().count() == 4) {
        nq::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse Nquads RDF")
    } else {
        trig::parse_str(rdf)
            .collect_quads()
            .expect("Failed to parse Trig RDF")
    };
    Ok(dataset)
}

/// Get a namespace commonly used in nanopub manipulation
pub fn get_ns(ns: &str) -> Namespace<String> {
    match ns {
        "npx" => Namespace::new("http://purl.org/nanopub/x/".to_string()).unwrap(),
        "np" => Namespace::new("http://www.nanopub.org/nschema#".to_string()).unwrap(),
        _ => panic!("Unknown namespace"), // or return an error
    }
}

/// Get the prefixes of a Nanopub
pub fn get_prefixes(np_uri: &str, np_ns: &str) -> [(Prefix<String>, Iri<String>); 13] {
    [
        (
            Prefix::new_unchecked("this".to_string()),
            Iri::new_unchecked(np_uri.to_string()),
        ),
        (
            Prefix::new_unchecked("sub".to_string()),
            Iri::new_unchecked(np_ns.to_string()),
        ),
        (
            Prefix::new_unchecked("rdf".to_string()),
            Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()),
        ),
        (
            Prefix::new_unchecked("rdfs".to_string()),
            Iri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ),
        (
            Prefix::new_unchecked("xsd".to_string()),
            Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#".to_string()),
        ),
        (
            Prefix::new_unchecked("schema".to_string()),
            Iri::new_unchecked("http://schema.org/".to_string()),
        ),
        (
            Prefix::new_unchecked("foaf".to_string()),
            Iri::new_unchecked("http://xmlns.com/foaf/0.1/".to_string()),
        ),
        (
            Prefix::new_unchecked("biolink".to_string()),
            Iri::new_unchecked("https://w3id.org/biolink/vocab/".to_string()),
        ),
        (
            Prefix::new_unchecked("np".to_string()),
            Iri::new_unchecked(get_ns("np").to_string()),
        ),
        (
            Prefix::new_unchecked("prov".to_string()),
            Iri::new_unchecked("http://www.w3.org/ns/prov#".to_string()),
        ),
        (
            Prefix::new_unchecked("dcterms".to_string()),
            Iri::new_unchecked("http://purl.org/dc/terms/".to_string()),
        ),
        (
            Prefix::new_unchecked("orcid".to_string()),
            Iri::new_unchecked("https://orcid.org/".to_string()),
        ),
        (
            Prefix::new_unchecked("npx".to_string()),
            Iri::new_unchecked(get_ns("npx").to_string()),
        ),
    ]
}
