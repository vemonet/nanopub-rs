use crate::constants::{NP_PREF_NS, NP_TEMP_URI};
use crate::error::NpError;
use crate::vocab::{dct, np, npx, pav, prov};

use oxrdf::{
    vocab::rdf, Dataset, GraphNameRef, NamedNode, NamedNodeRef, NamedOrBlankNodeRef, TermRef,
};
use regex::Regex;
use serde::Serialize;
use std::fmt;

/// Infos extracted from a nanopublication: graphs URLs, signature, trusty hash...
#[derive(Clone, Serialize, Debug)]
pub struct NpInfo {
    pub uri: NamedNode,
    pub ns: NamedNode,
    pub prefixes: Vec<(String, String)>,
    pub normalized_ns: String,
    pub head: NamedNode,
    pub assertion: NamedNode,
    pub prov: NamedNode,
    pub pubinfo: NamedNode,
    pub base_uri: String,
    pub separator_before_trusty: String,
    pub separator_after_trusty: String,
    pub trusty_hash: String,
    pub signature: String,
    pub signature_iri: NamedNode,
    pub algo: String,
    pub public_key: String,
    pub orcid: String,
    pub published: Option<String>,
}

impl fmt::Display for NpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nNanopub URI: {}", self.uri.as_str())?;
        writeln!(f, "Namespace: {}", self.ns.as_str())?;
        writeln!(f, "Base URI: {}", self.base_uri)?;
        writeln!(f, "Trusty Hash: {}", self.trusty_hash)?;
        writeln!(f, "ORCID: {}", self.orcid)?;
        writeln!(f, "Head Graph: {}", self.head.as_str())?;
        writeln!(f, "Assertion Graph: {}", self.assertion.as_str())?;
        Ok(())
    }
}

/// Extract graphs URLs from a nanopub: nanopub URL, head, assertion, prov, pubinfo
pub fn extract_np_info(
    dataset: &Dataset,
    prefixes: Vec<(String, String)>,
) -> Result<NpInfo, NpError> {
    // Extract nanopub URL and head graph
    let mut head_iterator = dataset
        .quads_for_predicate(rdf::TYPE)
        .filter(|x| x.object == TermRef::NamedNode(np::NANOPUBLICATION));
    let (mut np_iri, head_iri) = match head_iterator.next() {
        Some(q) => {
            let NamedOrBlankNodeRef::NamedNode(np) = q.subject else {
                return Err(NpError("Subject must be a named node.".to_string()));
            };
            let GraphNameRef::NamedNode(head) = q.graph_name else {
                return Err(NpError("Graph name must be a named node.".to_string()));
            };
            (NamedNode::from(np), NamedNode::from(head))
        }
        None => {
            return Err(NpError(
                "The provided RDF does not contain a Nanopublication.".to_string(),
            ))
        }
    };
    if head_iterator.next().is_some() {
        return Err(NpError(
            "The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string(),
        ));
    };
    let mut np_subject_term = NamedOrBlankNodeRef::from(np_iri.as_ref());
    let head_graph = dataset.graph(head_iri.as_ref());

    // Extract assertion, prov, pubinfo, and head graphs URLs
    let assertion_iri =
        match head_graph.object_for_subject_predicate(np_subject_term, np::HAS_ASSERTION) {
            Some(object) => {
                let TermRef::NamedNode(assertion) = object else {
                    return Err(NpError("Object must be a named node.".to_string()));
                };
                NamedNode::from(assertion)
            }
            None => {
                return Err(NpError(
                    "Invalid Nanopub: no Assertion graph found.".to_string(),
                ))
            }
        };
    let prov_iri =
        match head_graph.object_for_subject_predicate(np_subject_term, np::HAS_PROVENANCE) {
            Some(object) => {
                let TermRef::NamedNode(prov) = object else {
                    return Err(NpError("Object must be a named node.".to_string()));
                };
                NamedNode::from(prov)
            }
            None => {
                return Err(NpError(
                    "Invalid Nanopub: no Provenance graph found.".to_string(),
                ))
            }
        };
    let pubinfo_iri =
        match head_graph.object_for_subject_predicate(np_subject_term, np::HAS_PUBLICATION_INFO) {
            Some(object) => {
                let TermRef::NamedNode(pubinfo) = object else {
                    return Err(NpError("Object must be a named node.".to_string()));
                };
                NamedNode::from(pubinfo)
            }
            None => {
                return Err(NpError(
                    "Invalid Nanopub: no PubInfo graph found.".to_string(),
                ))
            }
        };

    // Get just the Trusty hash from the URI
    let re_trusty = Regex::new(r"^.*?[/#\.]?(RA[a-zA-Z0-9-_]*)$")?;
    let trusty_hash = if let Some(caps) = re_trusty.captures(np_iri.as_str()) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        caps.get(1).map_or("", |m| m.as_str()).to_string()
    } else {
        "".to_string()
    };

    // Getting potential ns from head graph (removing the last frag from head)
    let original_ns = if trusty_hash.is_empty() {
        &head_iri.as_str()[..np_iri.as_str().len()]
    } else {
        &head_iri.as_str()[..np_iri.as_str().len() + 1]
    };
    let np_ns_str = original_ns;

    // Remove last char if it is # or / to get the URI
    if np_iri.as_str().ends_with(['#', '/', '.']) {
        np_iri = NamedNode::new_unchecked(np_iri.as_str()[..np_iri.as_str().len() - 1].to_string());
        np_subject_term = NamedOrBlankNodeRef::from(np_iri.as_ref());
    };

    // Extract base URI, separator character (# or / or _), and trusty hash (if present) from the np URL
    // Default to empty strings when nothing found
    let mut base_uri: String = "".to_string();
    let mut separator_before_trusty: String = '.'.to_string();
    let mut separator_after_trusty: String = "".to_string();

    // Get the base URI and separators from the namespace
    let re_trusty_ns = Regex::new(r"^(.*?)(/|#|\.)?(RA[a-zA-Z0-9-_]*)?([#/\.])?$")?;
    if let Some(caps) = re_trusty_ns.captures(original_ns) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        base_uri = caps.get(1).map_or("", |m| m.as_str()).to_string();
        // The second group captures '/' or '#' if present, defaults to .
        separator_before_trusty = caps
            .get(2)
            .map_or(separator_before_trusty, |m| m.as_str().to_string())
            .to_string();
        // The last group captures everything after 'RA', if present.
        separator_after_trusty = caps
            .get(4)
            .map_or(separator_after_trusty, |m| m.as_str().to_string())
            .to_string();
    }
    if trusty_hash.is_empty() && separator_after_trusty.is_empty() {
        separator_after_trusty = "/".to_string()
    };

    // Generate normalized namespace without trusty
    let norm_ns = if !trusty_hash.is_empty() {
        format!("{base_uri}{separator_before_trusty}")
    } else if original_ns.starts_with(NP_TEMP_URI) {
        NP_PREF_NS.to_string()
    } else if !original_ns.ends_with(['#', '/', '.']) {
        format!("{}.", &original_ns)
    } else {
        original_ns.to_string()
    };

    // Extract signature and its subject URI
    let pubinfo_graph = dataset.graph(pubinfo_iri.as_ref());
    let (signature, signature_iri) = match pubinfo_graph
        .triples_for_predicate(npx::HAS_SIGNATURE)
        .next()
    {
        Some(q) => {
            let TermRef::Literal(literal) = q.object else {
                return Err(NpError("Object must be a literal.".to_string()));
            };
            let NamedOrBlankNodeRef::NamedNode(sig_iri) = q.subject else {
                return Err(NpError("Subject must be a named node.".to_string()));
            };
            (literal.value().to_string(), NamedNode::from(sig_iri))
        }
        None => (
            "".to_string(),
            NamedNode::new_unchecked(format!("{}sig", np_ns_str)),
        ),
    };
    let signature_node = NamedOrBlankNodeRef::from(signature_iri.as_ref());

    // Extract public key
    let pubkey =
        match pubinfo_graph.object_for_subject_predicate(signature_node, npx::HAS_PUBLIC_KEY) {
            Some(object) => {
                let TermRef::Literal(literal) = object else {
                    return Err(NpError("Object must be a literal.".to_string()));
                };
                Some(literal.value().to_string())
            }
            None => None,
        };

    // Extract algo
    let algo = match pubinfo_graph.object_for_subject_predicate(signature_node, npx::HAS_ALGORITHM)
    {
        Some(object) => {
            let TermRef::Literal(literal) = object else {
                return Err(NpError("Object must be a literal.".to_string()));
            };
            Some(literal.value().to_string())
        }
        None => None,
    };

    // Extract ORCID
    let orcid = match pubinfo_graph.iter().find(|x| {
        (x.subject == np_subject_term
            || x.subject == NamedOrBlankNodeRef::from(NamedNodeRef::new_unchecked(original_ns)))
            && (x.predicate == dct::CREATOR
                || x.predicate == prov::WAS_ATTRIBUTED_TO
                || x.predicate == pav::CREATED_BY)
    }) {
        Some(q) => {
            let orcid = match q.object {
                TermRef::Literal(literal) => literal.value().to_string(),
                TermRef::NamedNode(literal) => literal.into_owned().into_string(),
                TermRef::BlankNode(_) => {
                    return Err(NpError(
                        "Object must be a literal or a named node, not a blank node.".to_string(),
                    ))
                }
            };
            Some(orcid)
        }
        None => None,
    };

    Ok(NpInfo {
        uri: np_iri,
        ns: NamedNode::new_unchecked(np_ns_str),
        prefixes,
        normalized_ns: norm_ns,
        head: head_iri,
        assertion: assertion_iri,
        prov: prov_iri,
        pubinfo: pubinfo_iri,
        base_uri,
        separator_before_trusty,
        separator_after_trusty,
        trusty_hash,
        signature,
        signature_iri,
        public_key: pubkey.unwrap_or("".to_string()),
        algo: algo.unwrap_or("".to_string()),
        orcid: orcid.unwrap_or("".to_string()),
        published: None,
    })
}
