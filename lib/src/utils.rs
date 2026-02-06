// use rand::{thread_rng, Rng as _};
use getrandom::getrandom;
use oxjsonld::JsonLdProfileSet;
use oxrdf::{Dataset, GraphNameRef, NamedNodeRef, NamedOrBlankNodeRef, QuadRef, TermRef};
use oxrdfio::{RdfFormat, RdfParser, RdfSerializer};

use crate::constants::LIST_SERVERS;
use crate::error::NpError;

/// Extension trait for `Dataset` providing efficient quad matching with optional filters.
///
/// This trait adds a `quads_match` method that efficiently queries quads by
/// selecting the best internal index based on which parameters are provided.
pub trait DatasetExt {
    /// Returns an iterator over quads matching the given subjects, predicates, objects, and graphs.
    ///
    /// Pass an empty slice `&[]` for any parameter to match all values for that position.
    /// Pass a non-empty slice to match any of the values in the slice.
    ///
    /// # Example
    ///
    /// ```
    /// use oxrdf::{Dataset, GraphNameRef, NamedNodeRef, QuadRef, TermRef};
    /// use nanopub::DatasetExt;
    /// use nanopub::vocab::{dct, pav, prov};
    /// let mut dataset = Dataset::new();
    /// let s = NamedNodeRef::new_unchecked("http://ex.org/s");
    /// let other = NamedNodeRef::new_unchecked("http://ex.org/other");
    /// let o1 = NamedNodeRef::new_unchecked("http://ex.org/o1");
    /// let o2 = NamedNodeRef::new_unchecked("http://ex.org/o2");
    /// dataset.insert(QuadRef::new(s, dct::CREATOR, o1, GraphNameRef::DefaultGraph));
    /// dataset.insert(QuadRef::new(s, pav::CREATED_BY, o2, GraphNameRef::DefaultGraph));
    /// dataset.insert(QuadRef::new(other, dct::CREATOR, o1, GraphNameRef::DefaultGraph));
    /// dataset.insert(QuadRef::new(other, prov::WAS_ATTRIBUTED_TO, o2, GraphNameRef::DefaultGraph));
    /// let subjects = [s.into()];
    /// let mut count = 0;
    /// for q in dataset.quads_match(&subjects, &[dct::CREATOR, pav::CREATED_BY], &[], &[]) {
    ///     match q.object {
    ///         TermRef::NamedNode(iri) => {
    ///             assert!(iri == o1 || iri == o2, "unexpected object: {iri:?}");
    ///         }
    ///         _ => panic!("unexpected non-named object in test"),
    ///     }
    ///     count += 1;
    /// }
    /// assert_eq!(count, 2);
    /// ```
    fn quads_match<'a>(
        &'a self,
        subjects: &'a [NamedOrBlankNodeRef<'a>],
        predicates: &'a [NamedNodeRef<'a>],
        objects: &'a [TermRef<'a>],
        graphs: &'a [GraphNameRef<'a>],
    ) -> Box<dyn Iterator<Item = QuadRef<'a>> + 'a>;
}

impl DatasetExt for Dataset {
    fn quads_match<'a>(
        &'a self,
        subjects: &'a [NamedOrBlankNodeRef<'a>],
        predicates: &'a [NamedNodeRef<'a>],
        objects: &'a [TermRef<'a>],
        graphs: &'a [GraphNameRef<'a>],
    ) -> Box<dyn Iterator<Item = QuadRef<'a>> + 'a> {
        if !subjects.is_empty() {
            Box::new(
                subjects
                    .iter()
                    .flat_map(|s| self.quads_for_subject(*s))
                    .filter(move |q| {
                        (predicates.is_empty() || predicates.contains(&q.predicate))
                            && (objects.is_empty() || objects.contains(&q.object))
                            && (graphs.is_empty() || graphs.contains(&q.graph_name))
                    }),
            )
        } else if !predicates.is_empty() {
            Box::new(
                predicates
                    .iter()
                    .flat_map(|p| self.quads_for_predicate(*p))
                    .filter(move |q| {
                        (objects.is_empty() || objects.contains(&q.object))
                            && (graphs.is_empty() || graphs.contains(&q.graph_name))
                    }),
            )
        } else if !objects.is_empty() {
            Box::new(
                objects
                    .iter()
                    .flat_map(|o| self.quads_for_object(*o))
                    .filter(move |q| graphs.is_empty() || graphs.contains(&q.graph_name)),
            )
        } else if !graphs.is_empty() {
            Box::new(graphs.iter().flat_map(|g| self.quads_for_graph_name(*g)))
        } else {
            Box::new(self.iter())
        }
    }
}

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

/// Extract IRI as `String` from subject term
pub fn subject_iri_to_string(node: NamedOrBlankNodeRef) -> Result<String, NpError> {
    match node {
        NamedOrBlankNodeRef::NamedNode(iri) => Ok(iri.into_owned().into_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!(
                "Failed to extract IRI from subject: Got {}",
                variant_name
            )))
        }
    }
}

/// Extract IRI as `String` from object term
pub fn object_iri_to_string(node: TermRef) -> Result<String, NpError> {
    match node {
        TermRef::NamedNode(iri) => Ok(iri.into_owned().into_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!(
                "Failed to extract IRI from object: Got {}",
                variant_name
            )))
        }
    }
}

/// Extract blank node ID as `&str` from object term
pub fn object_blank_to_str(node: TermRef<'_>) -> Result<&str, NpError> {
    match node {
        TermRef::BlankNode(id) => Ok(id.as_str()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!(
                "Failed to extract blank node ID from object: Got {}",
                variant_name
            )))
        }
    }
}

/// Extract literal as `String` tuple from object term
pub fn object_literal_to_strings(node: TermRef) -> Result<(String, String, String), NpError> {
    match node {
        TermRef::Literal(literal) => Ok((
            literal.value().to_string(),
            literal.datatype().into_owned().into_string(),
            literal.language().unwrap_or_default().to_owned(),
        )),
        TermRef::NamedNode(iri) => Ok((
            iri.into_owned().into_string(),
            "".to_string(),
            "".to_string(),
        )),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!(
                "Failed to extract literal from object: Got {}",
                variant_name
            )))
        }
    }
}

/// Extract IRI as `String` from graph name
pub fn graph_iri_to_string(node: GraphNameRef) -> Result<String, NpError> {
    match node {
        GraphNameRef::NamedNode(iri) => Ok(iri.into_owned().into_string()),
        other => {
            let debug_str = format!("{:?}", other);
            let variant_name = debug_str
                .split('(')
                .next()
                .and_then(|s| s.split("::").last())
                .unwrap_or("Unknown");
            Err(NpError(format!(
                "Failed to extract graph name IRI: Got {}",
                variant_name
            )))
        }
    }
}
