use getrandom::getrandom;
use oxjsonld::JsonLdParser;
use oxrdf::{Dataset, IriParseError, GraphNameRef, NamedOrBlankNodeRef, NamedNode, NamedNodeRef, QuadRef, TermRef};
use oxttl::{TriGParser, TriGSerializer};
use serde::Serialize;
use std::cmp::Ordering;

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

/// Parse RDF from various format to a `Dataset` (trig, nquads, JSON-LD)
pub fn parse_rdf(rdf: &str) -> Result<(Dataset, Vec<(String, String)>), NpError> {
    let mut dataset = Dataset::new();
    // NOTE: an efficient way to differentiate between JSON-LD and TriG is to check if the string starts with '{' or '['
    let prefixes: Vec<(String, String)> = if rdf.trim_start().starts_with(['{', '[']) {
        let mut parser = JsonLdParser::new().for_reader(rdf.as_bytes());
        parser.try_for_each(|q| {
            dataset.insert(&q?);
            Ok::<_, NpError>(())
        })?;
        parser
            .prefixes()
            .map(|(prefix, iri)| (prefix.to_owned(), iri.to_owned()))
            .collect()
    } else {
        // The TriG parser handles nquads
        let mut parser = TriGParser::new().for_reader(rdf.as_bytes());
        parser.try_for_each(|q| {
            dataset.insert(&q?);
            Ok::<_, NpError>(())
        })?;
        parser
            .prefixes()
            .map(|(prefix, iri)| (prefix.to_owned(), iri.to_owned()))
            .collect()
    };
    Ok((dataset, prefixes))
}

/// Serialize RDF dataset to Trig
pub fn serialize_rdf(
    dataset: &Dataset,
    uri: &str,
    ns: &str,
    prefixes: &[(String, String)],
) -> Result<String, NpError> {
    let mut serializer = TriGSerializer::new();
    // Add a set of default prefixes
    for (prefix_name, prefix_iri) in [
        ("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ("rdfs", "http://www.w3.org/2000/01/rdf-schema#"),
        ("xsd", "http://www.w3.org/2001/XMLSchema#"),
        ("owl", "http://www.w3.org/2002/07/owl#"),
        ("skos", "http://www.w3.org/2004/02/skos/core#"),
        ("np", "http://www.nanopub.org/nschema#"),
        ("npx", "http://purl.org/nanopub/x/"),
        ("orcid", "https://orcid.org/"),
    ] {
        serializer = serializer.with_prefix(prefix_name, prefix_iri)?;
    }
    // Add or override prefixes from parsed source document (removing empty prefix which would be used as base)
    for (prefix_name, prefix_iri) in prefixes.iter().filter(|(name, _)| !name.is_empty()) {
        serializer = serializer.with_prefix(prefix_name, prefix_iri)?;
    }
    // Add or override core prefixes
    serializer = serializer
        .with_prefix("this", uri)?
        .with_prefix("sub", ns)?;
    let mut serializer = serializer.for_writer(Vec::new());
    // NOTE: we need to sort ourself the quads
    let mut quads: Vec<QuadRef<'_>> = dataset.iter().collect();
    quads.sort_by(quad_compare);
    for quad in &quads {
        serializer.serialize_quad(*quad)?;
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

/// Compare two quads for sorting: by graph, subject, predicate (rdf:type first), object
fn quad_compare(a: &QuadRef<'_>, b: &QuadRef<'_>) -> Ordering {
    graph_compare(a.graph_name, b.graph_name)
        .then_with(|| subject_compare(a.subject, b.subject))
        .then_with(|| predicate_compare(a.predicate, b.predicate))
        .then_with(|| term_compare(a.object, b.object))
}

/// Sort RDF graphs: default graph first, then named graphs by IRI, then blank nodes
fn graph_compare(a: GraphNameRef<'_>, b: GraphNameRef<'_>) -> Ordering {
    fn graph_rank(g: GraphNameRef<'_>) -> u8 {
        match g {
            GraphNameRef::DefaultGraph => 0,
            GraphNameRef::NamedNode(_) => 1,
            GraphNameRef::BlankNode(_) => 2,
        }
    }
    fn graph_str<'a>(g: GraphNameRef<'a>) -> &'a str {
        match g {
            GraphNameRef::DefaultGraph => "",
            GraphNameRef::NamedNode(n) => n.as_str(),
            GraphNameRef::BlankNode(b) => b.as_str(),
        }
    }
    graph_rank(a)
        .cmp(&graph_rank(b))
        .then_with(|| graph_str(a).cmp(graph_str(b)))
}

/// Compare subjects: named nodes first (by IRI), then blank nodes (by id)
fn subject_compare(a: NamedOrBlankNodeRef<'_>, b: NamedOrBlankNodeRef<'_>) -> Ordering {
    fn rank(n: NamedOrBlankNodeRef<'_>) -> u8 {
        match n {
            NamedOrBlankNodeRef::NamedNode(_) => 0,
            NamedOrBlankNodeRef::BlankNode(_) => 1,
        }
    }
    fn as_str<'a>(n: NamedOrBlankNodeRef<'a>) -> &'a str {
        match n {
            NamedOrBlankNodeRef::NamedNode(n) => n.as_str(),
            NamedOrBlankNodeRef::BlankNode(b) => b.as_str(),
        }
    }
    rank(a).cmp(&rank(b)).then_with(|| as_str(a).cmp(as_str(b)))
}

/// Compare predicates: rdf:type first, then by IRI
fn predicate_compare(a: oxrdf::NamedNodeRef<'_>, b: oxrdf::NamedNodeRef<'_>) -> Ordering {
    const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    fn rank(p: oxrdf::NamedNodeRef<'_>) -> u8 {
        if p.as_str() == RDF_TYPE {
            0
        } else {
            1
        }
    }
    rank(a)
        .cmp(&rank(b))
        .then_with(|| a.as_str().cmp(b.as_str()))
}

/// Compare object terms: named nodes, blank nodes, then literals; within each kind compare by string content
fn term_compare(a: TermRef<'_>, b: TermRef<'_>) -> Ordering {
    fn rank(t: TermRef<'_>) -> u8 {
        match t {
            TermRef::NamedNode(_) => 0,
            TermRef::BlankNode(_) => 1,
            TermRef::Literal(_) => 2,
        }
    }
    fn as_str<'a>(t: TermRef<'a>) -> &'a str {
        match t {
            TermRef::NamedNode(n) => n.as_str(),
            TermRef::BlankNode(b) => b.as_str(),
            TermRef::Literal(l) => l.value(),
        }
    }
    rank(a).cmp(&rank(b)).then_with(|| as_str(a).cmp(as_str(b)))
}

#[derive(Clone, Serialize, Debug)]
pub struct Namespace<String> {
    base: String,
}

impl Namespace<String> {
    pub fn new(base: String) -> Result<Self, IriParseError> {
        Ok(Namespace {
            base
        })
    }
    pub fn new_unchecked(base: String) -> Self {
        Namespace {
            base
        }
    }
    pub fn as_str(&self) -> &str {
        self.base.as_str()
    }
    pub fn as_iri_ref(&self) -> NamedNodeRef<'_> {
        NamedNodeRef::new_unchecked(self.base.as_str())
    }
    pub fn get(&self, local_name: &str) -> Result<NamedNode, IriParseError> {
        let iri_string = format!("{}{}", self.base, local_name);
        NamedNode::new(iri_string)
    }
}
