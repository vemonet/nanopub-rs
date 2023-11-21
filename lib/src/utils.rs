// use rand::{thread_rng, Rng as _};
use getrandom::getrandom;
use sophia::api::serializer::{QuadSerializer as _, Stringifier as _};
use sophia::api::source::QuadSource as _;
use sophia::api::{ns::Namespace, prefix::Prefix};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia::jsonld;
use sophia::turtle::parser::{nq, trig};
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};
// use rand::prelude::*;

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

/// Parse RDF from various format to a `LightDataset` (trig, nquads, JSON-LD)
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_rdf(rdf: &str) -> Result<LightDataset, NpError> {
    let rdf = rdf.to_string();
    // The JSON-LD parser uses futures::block_on which creates conflict when running in tokio runtime
    // So we need to spawn a separate thread
    let handle = std::thread::spawn(move || {
        futures::executor::block_on(async {
            return if rdf.trim().starts_with('{') || rdf.trim().starts_with('[') {
                jsonld::parse_str(&rdf)
                    .collect_quads()
                    .expect("Failed to parse JSON-LD RDF")
            } else if rdf.lines().all(|line| line.split_whitespace().count() == 4) {
                nq::parse_str(&rdf)
                    .collect_quads()
                    .expect("Failed to parse Nquads RDF")
            } else {
                trig::parse_str(&rdf)
                    .collect_quads()
                    .expect("Failed to parse Trig RDF")
            };
        })
    });
    // Retrieve the result from the isolated operation
    let dataset = handle.join().expect("Error parsing RDF");
    Ok(dataset)
    // NOTE: XML does not support graph apart from the uncommon trix format
    // } else if rdf.starts_with("<?xml") {
    //     let graph = xml::parser::parse_str(rdf)
    //         .collect_triples()
    //         .expect("Failed to parse XML RDF");
}

/// Parse RDF, in wasm we don't need to do the futures trick because we don't use tokio async runtime
#[cfg(target_arch = "wasm32")]
pub fn parse_rdf(rdf: &str) -> Result<LightDataset, NpError> {
    let rdf = rdf.to_string();
    let dataset = if rdf.trim().starts_with('{') || rdf.trim().starts_with('[') {
        jsonld::parse_str(&rdf)
            .collect_quads()
            .expect("Failed to parse JSON-LD RDF")
    } else if rdf.lines().all(|line| line.split_whitespace().count() == 4) {
        nq::parse_str(&rdf)
            .collect_quads()
            .expect("Failed to parse Nquads RDF")
    } else {
        trig::parse_str(&rdf)
            .collect_quads()
            .expect("Failed to parse Trig RDF")
    };
    Ok(dataset)
}

/// Serialize RDF dataset to Trig
pub fn serialize_rdf(dataset: &LightDataset, uri: &str, ns: &str) -> Result<String, NpError> {
    let prefixes = get_prefixes(uri, ns);
    let trig_config = TrigConfig::new()
        .with_pretty(true)
        .with_prefix_map(&prefixes[..]);
    let mut trig_stringifier = TrigSerializer::new_stringifier_with_config(trig_config);

    Ok(trig_stringifier.serialize_dataset(&dataset)?.to_string())
}

/// Return a Nanopub server, the main one or one picked randomly from the list of available servers
pub fn get_np_server(random: bool) -> &'static str {
    if !random {
        return LIST_SERVERS[0];
    }
    // Generate a random number
    let mut buf = [0u8; 4]; // Buffer to store 4 bytes (u32)
    getrandom(&mut buf).expect("Failed to generate random number");
    let num = u32::from_ne_bytes(buf); // Convert bytes to u32
                                       // Use the random number to generate an index
    let index = num as usize % LIST_SERVERS.len();
    LIST_SERVERS[index]
}

/// Get a namespace commonly used in nanopub manipulation
pub fn get_ns(ns: &str) -> Namespace<String> {
    match ns {
        "npx" => Namespace::new("http://purl.org/nanopub/x/".to_string()).unwrap(),
        "np" => Namespace::new("http://www.nanopub.org/nschema#".to_string()).unwrap(),
        "dct" => Namespace::new("http://purl.org/dc/terms/".to_string()).unwrap(),
        _ => panic!("Unknown namespace"), // or return an error
    }
}

/// Get the prefixes of a Nanopub
pub fn get_prefixes(np_uri: &str, np_ns: &str) -> [(Prefix<String>, Iri<String>); 14] {
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
            Prefix::new_unchecked("pav".to_string()),
            Iri::new_unchecked("http://purl.org/pav/".to_string()),
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
