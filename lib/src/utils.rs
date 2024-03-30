// use rand::{thread_rng, Rng as _};
use getrandom::getrandom;
use sophia::api::serializer::{QuadSerializer as _, Stringifier as _};
use sophia::api::source::QuadSource as _;
use sophia::api::{ns::Namespace, prefix::Prefix};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use sophia::jsonld;
use sophia::turtle::parser::trig;
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

/// Parse RDF from various format to a `LightDataset` (trig, nquads, JSON-LD)
pub fn parse_rdf(rdf: &str) -> Result<LightDataset, NpError> {
    let rdf = rdf.to_string();
    // NOTE: an efficient way to differentiate between JSON-LD and TriG is to check if the string starts with '{' or '['
    let dataset = if rdf.trim().starts_with('{') || rdf.trim().starts_with('[') {
        parse_jsonld(&rdf)?
    } else {
        // TODO: extract prefixes https://github.com/pchampin/sophia_rs/issues/45
        // The TriG parser handles nquads
        trig::parse_str(&rdf)
            .collect_quads()
            .map_err(|e| NpError(format!("Error parsing TriG: {e}")))?
        // NOTE: we can access the trig parser prefixes, but we always get an empty map, because it's not parsed yet
        // let parser = trig::parse_str(&rdf);
        // let prefixes = parser.0.prefixes();
        // println!("PREFIXES: {:?}", prefixes);
        // let dataset = parser.collect_quads()
        //     .map_err(|e| NpError(format!("Error parsing TriG: {e}")))?;
        // dataset
    };
    Ok(dataset)
}

/// The JSON-LD parser uses futures::block_on which creates conflict
/// when running in tokio runtime, so we need to spawn a separate thread
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_jsonld(rdf: &str) -> Result<LightDataset, NpError> {
    let rdf = rdf.to_string();
    let handle = std::thread::spawn(move || {
        futures::executor::block_on(async { jsonld::parse_str(&rdf).collect_quads() })
    });
    let dataset = handle
        .join()
        .map_err(|_| NpError("Error retrieving JSON-LD from thread".to_string()))?
        .map_err(|e| NpError(format!("Error parsing JSON-LD: {e}")))?;
    Ok(dataset)
}

/// Parse JSON-LD, in wasm we don't need to do the futures trick because we don't use tokio async runtime
#[cfg(target_arch = "wasm32")]
pub fn parse_jsonld(rdf: &str) -> Result<LightDataset, NpError> {
    Ok(jsonld::parse_str(rdf)
        .collect_quads()
        .map_err(|e| NpError(format!("Error parsing JSON-LD: {e}")))?)
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
    let mut buf = [0u8; 4];
    getrandom(&mut buf).expect("Failed to generate random number");
    let num = u32::from_ne_bytes(buf);
    let index = num as usize % LIST_SERVERS.len();
    LIST_SERVERS[index]
}

/// Get a namespace commonly used in nanopub manipulation
pub fn ns(ns: &str) -> Namespace<String> {
    match ns {
        "npx" => Namespace::new("http://purl.org/nanopub/x/".to_string()).unwrap(),
        "np" => Namespace::new("http://www.nanopub.org/nschema#".to_string()).unwrap(),
        "dct" => Namespace::new("http://purl.org/dc/terms/".to_string()).unwrap(),
        "prov" => Namespace::new("http://www.w3.org/ns/prov#".to_string()).unwrap(),
        "pav" => Namespace::new("http://purl.org/pav/".to_string()).unwrap(),
        "foaf" => Namespace::new("http://xmlns.com/foaf/0.1/".to_string()).unwrap(),
        _ => panic!("Unknown namespace"), // or return an error
    }
}

// TODO: improve to extract prefixes from the input RDF
/// Get the prefixes of a Nanopub
pub fn get_prefixes(np_uri: &str, np_ns: &str) -> [(Prefix<String>, Iri<String>); 18] {
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
            Prefix::new_unchecked("owl".to_string()),
            Iri::new_unchecked("http://www.w3.org/2002/07/owl#".to_string()),
        ),
        (
            Prefix::new_unchecked("skos".to_string()),
            Iri::new_unchecked("http://www.w3.org/2004/02/skos/core#".to_string()),
        ),
        (
            Prefix::new_unchecked("np".to_string()),
            Iri::new_unchecked(ns("np").to_string()),
        ),
        (
            Prefix::new_unchecked("npx".to_string()),
            Iri::new_unchecked(ns("npx").to_string()),
        ),
        (
            Prefix::new_unchecked("dc".to_string()),
            Iri::new_unchecked("http://purl.org/dc/elements/1.1/".to_string()),
        ),
        (
            Prefix::new_unchecked("dcterms".to_string()),
            Iri::new_unchecked("http://purl.org/dc/terms/".to_string()),
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
            Prefix::new_unchecked("schema".to_string()),
            Iri::new_unchecked("https://schema.org/".to_string()),
        ),
        (
            Prefix::new_unchecked("foaf".to_string()),
            Iri::new_unchecked("http://xmlns.com/foaf/0.1/".to_string()),
        ),
        (
            Prefix::new_unchecked("orcid".to_string()),
            Iri::new_unchecked("https://orcid.org/".to_string()),
        ),
        (
            Prefix::new_unchecked("biolink".to_string()),
            Iri::new_unchecked("https://w3id.org/biolink/vocab/".to_string()),
        ),
        (
            Prefix::new_unchecked("infores".to_string()),
            Iri::new_unchecked("https://w3id.org/biolink/infores/".to_string()),
        ),
    ]
}
