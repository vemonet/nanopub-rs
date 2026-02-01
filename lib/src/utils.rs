// use rand::{thread_rng, Rng as _};
use getrandom::getrandom;
use oxiri::Iri;
use sophia::api::serializer::{QuadSerializer as _, Stringifier as _};
use sophia::api::source::QuadSource as _;
use sophia::api::{prelude::Term, term::SimpleTerm};
use sophia::inmem::dataset::LightDataset as Dataset;
use sophia::jsonld;
use sophia::turtle::parser::trig;
use sophia::turtle::serializer::trig::{TrigConfig, TrigSerializer};

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

/// Parse RDF from various format to a `Dataset` (trig, nquads, JSON-LD)
pub fn parse_rdf(rdf: &str) -> Result<Dataset, NpError> {
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
pub fn parse_jsonld(rdf: &str) -> Result<Dataset, NpError> {
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
pub fn parse_jsonld(rdf: &str) -> Result<Dataset, NpError> {
    Ok(jsonld::parse_str(rdf)
        .collect_quads()
        .map_err(|e| NpError(format!("Error parsing JSON-LD: {e}")))?)
}

/// Serialize RDF dataset to Trig
pub fn serialize_rdf(dataset: &Dataset, uri: &str, ns: &str) -> Result<String, NpError> {
    let prefixes = get_prefixes(uri, ns)?;
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

// TODO: improve to extract prefixes from the input RDF
/// Get the prefixes of a Nanopub
pub fn get_prefixes(
    np_uri: &str,
    np_ns: &str,
) -> Result<[(String, Iri<String>); 18], NpError> {
    Ok([
        (
            "this".to_string(),
            Iri::parse_unchecked(np_uri.to_string()),
        ),
        (
            "sub".to_string(),
            Iri::parse_unchecked(np_ns.to_string()),
        ),
        (
            "rdf".to_string(),
            Iri::parse_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()),
        ),
        (
            "rdfs".to_string(),
            Iri::parse_unchecked("http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ),
        (
            "xsd".to_string(),
            Iri::parse_unchecked("http://www.w3.org/2001/XMLSchema#".to_string()),
        ),
        (
            "owl".to_string(),
            Iri::parse_unchecked("http://www.w3.org/2002/07/owl#".to_string()),
        ),
        (
            "skos".to_string(),
            Iri::parse_unchecked("http://www.w3.org/2004/02/skos/core#".to_string()),
        ),
        (
            "np".to_string(),
            Iri::parse_unchecked("http://www.nanopub.org/nschema#".to_string()),
        ),
        (
            "npx".to_string(),
            Iri::parse_unchecked("http://purl.org/nanopub/x/".to_string()),
        ),
        (
            "dc".to_string(),
            Iri::parse_unchecked("http://purl.org/dc/elements/1.1/".to_string()),
        ),
        (
            "dcterms".to_string(),
            Iri::parse_unchecked("http://purl.org/dc/terms/".to_string()),
        ),
        (
            "prov".to_string(),
            Iri::parse_unchecked("http://www.w3.org/ns/prov#".to_string()),
        ),
        (
            "pav".to_string(),
            Iri::parse_unchecked("http://purl.org/pav/".to_string()),
        ),
        (
            "schema".to_string(),
            Iri::parse_unchecked("https://schema.org/".to_string()),
        ),
        (
            "foaf".to_string(),
            Iri::parse_unchecked("http://xmlns.com/foaf/0.1/".to_string()),
        ),
        (
            "orcid".to_string(),
            Iri::parse_unchecked("https://orcid.org/".to_string()),
        ),
        (
            "biolink".to_string(),
            Iri::parse_unchecked("https://w3id.org/biolink/vocab/".to_string()),
        ),
        (
            "infores".to_string(),
            Iri::parse_unchecked("https://w3id.org/biolink/infores/".to_string()),
        ),
    ])
}

/// Extract IRI as `String` from subject term
pub fn subject_iri_to_string(node: &SimpleTerm) -> Result<String, NpError> {
    match node {
        SimpleTerm::Iri(iri) => Ok(iri.as_ref().to_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract IRI from subject: Got {}", variant_name)))
        },
    }
}

/// Extract blank node ID as `&str` from subject term
pub fn subject_blank_to_str<'a>(node: &'a SimpleTerm<'a>) -> Result<&'a str, NpError> {
    match node {
        SimpleTerm::BlankNode(n) => Ok(n.as_str()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract blank node ID from subject: Got {}", variant_name)))
        },
    }
}

/// Extract IRI as `String` from predicate term
pub fn predicate_iri_to_string(node: &SimpleTerm) -> Result<String, NpError> {
    match node {
        SimpleTerm::Iri(iri) => Ok(iri.as_ref().to_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract IRI from predicate: Got {}", variant_name)))
        },
    }
}

/// Extract IRI as `String` from object term
pub fn object_iri_to_string(node: &SimpleTerm) -> Result<String, NpError> {
    match node {
        SimpleTerm::Iri(iri) => Ok(iri.as_ref().to_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract IRI from object: Got {}", variant_name)))
        },
    }
}

/// Extract blank node ID as `&str` from object term
pub fn object_blank_to_str<'a>(node: &'a SimpleTerm<'a>) -> Result<&'a str, NpError> {
    match node {
        SimpleTerm::BlankNode(n) => Ok(n.as_str()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract blank node ID from object: Got {}", variant_name)))
        },
    }
}

/// Extract literal as `String` tuple from object term
pub fn object_literal_to_strings(node: &SimpleTerm) -> Result<(String, String, String), NpError> {
    match node {
        SimpleTerm::LiteralDatatype(value, datatype) => {
            if node.is_literal() {
                Ok((value.to_string(), datatype.to_string(), "".to_string()))
            } else {
                Err(NpError("foo".to_string()))
            }
        },
        SimpleTerm::LiteralLanguage(value, tag) => {
            if node.is_literal() {
                Ok((value.to_string(), "".to_string(), tag.to_string()))
            } else {
                Err(NpError("bar".to_string()))
            }
        },
        SimpleTerm::Iri(iri) => {
            Ok((iri.as_ref().to_string(), "".to_string(), "".to_string()))
        },
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract literal from object: Got {}", variant_name)))
        },
    }
}

/// Extract IRI as `String` from graph name
pub fn graph_iri_to_string(node: Option<&SimpleTerm>) -> Result<String, NpError> {
    match node {
        Some(SimpleTerm::Iri(iri)) => Ok(iri.as_ref().to_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!("Failed to extract graph name IRI: Got {}", variant_name)))
        },
    }
}
