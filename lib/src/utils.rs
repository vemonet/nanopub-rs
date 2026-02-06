// use rand::{thread_rng, Rng as _};
use getrandom::getrandom;
use oxjsonld::JsonLdProfileSet;
use oxrdf::{Dataset, GraphNameRef, NamedOrBlankNodeRef};
use oxrdfio::{RdfFormat, RdfParser, RdfSerializer};

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

// TODO: improve to collect document prefixes, for use in `serialize_rdf()`
/// Parse RDF from various format to a `Dataset` (trig, nquads, JSON-LD)
pub fn parse_rdf(rdf: &str) -> Result<Dataset, NpError> {
    let mut dataset = Dataset::new();
    // NOTE: an efficient way to differentiate between JSON-LD and TriG is to check if the string starts with '{' or '['
    let format = if rdf.trim_start().starts_with(['{', '[']) {
        RdfFormat::JsonLd {
            profile: JsonLdProfileSet::empty(),
        }
    } else {
        // The TriG parser handles nquads
        RdfFormat::TriG
    };

    RdfParser::from_format(format)
        .for_reader(rdf.as_bytes())
        .try_for_each(|q| {
            dataset.insert(&q?);
            Ok::<_, NpError>(())
        })?;
    Ok(dataset)
}

// TODO: improve to use prefixes from `parse_rdf()`, favored over default ones
/// Serialize RDF dataset to Trig
pub fn serialize_rdf(dataset: &Dataset, uri: &str, ns: &str) -> Result<String, NpError> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::TriG).with_prefix("", ns)?;
    for (prefix_name, prefix_iri) in get_prefixes(uri, ns) {
        serializer = serializer.with_prefix(prefix_name, prefix_iri)?;
    }
    let mut serializer = serializer.for_writer(Vec::new());
    for quad in dataset.iter() {
        serializer.serialize_quad(quad)?;
    }
    Ok(String::from_utf8(serializer.finish()?)?)
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
pub fn get_prefixes<'a>(
    np_uri: &'a str,
    np_ns: &'a str,
) -> impl Iterator<Item = (&'static str, &'a str)> + 'a {
    [
        ("this", np_uri),
        ("sub", np_ns),
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
        ("xsd", "http://www.w3.org/2001/XMLSchema#"),
        ("owl", "http://www.w3.org/2002/07/owl#"),
        ("skos", "http://www.w3.org/2004/02/skos/core#"),
        ("np", "http://www.nanopub.org/nschema#"),
        ("npx", "http://purl.org/nanopub/x/"),
        ("dc", "http://purl.org/dc/elements/1.1/"),
        ("dcterms", "http://purl.org/dc/terms/"),
        ("prov", "http://www.w3.org/ns/prov#"),
        ("pav", "http://purl.org/pav/"),
        ("schema", "https://schema.org/"),
        ("foaf", "http://xmlns.com/foaf/0.1/"),
        ("orcid", "https://orcid.org/"),
        ("biolink", "https://w3id.org/biolink/vocab/"),
        ("inforces", "https://w3id.org/biolink/infores/"),
    ]
    .into_iter()
}

/// Extract IRI as `String` from subject term, or error if blank node
pub fn subject_iri_to_string(node: NamedOrBlankNodeRef) -> Result<String, NpError> {
    match node {
        NamedOrBlankNodeRef::NamedNode(iri) => Ok(iri.into_owned().into_string()),
        other => Err(NpError(format!(
            "Failed to extract IRI from subject: Got {other:?}"
        ))),
    }
}

/// Extract IRI as `String` from graph name, or error if not a named node
pub fn graph_iri_to_string(node: GraphNameRef) -> Result<String, NpError> {
    match node {
        GraphNameRef::NamedNode(iri) => Ok(iri.into_owned().into_string()),
        other => Err(NpError(format!(
            "Failed to extract graph name IRI: Got {other:?}"
        ))),
    }
}
