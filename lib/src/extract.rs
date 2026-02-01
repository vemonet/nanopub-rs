use crate::constants::{NP_PREF_NS, NP_TEMP_URI};
use crate::error::NpError;
use crate::utils::{
    graph_iri_to_string, object_iri_to_string, object_literal_to_strings, subject_iri_to_string,
    DatasetExt,
};
use crate::vocab::{dct, np, npx, pav, prov};

use oxrdf::{vocab::rdf, Dataset, GraphNameRef, NamedNode, NamedOrBlankNodeRef, TermRef};
use regex::Regex;
use serde::Serialize;
use std::fmt;

/// Infos extracted from a nanopublication: graphs URLs, signature, trusty hash...
#[derive(Clone, Serialize, Debug)]
pub struct NpInfo {
    pub uri: NamedNode,
    pub ns: NamedNode,
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
pub fn extract_np_info(dataset: &Dataset) -> Result<NpInfo, NpError> {
    let mut np_url: String = "".to_string();
    let mut head: String = "".to_string();
    let mut assertion: String = "".to_string();
    let mut prov: String = "".to_string();
    let mut pubinfo: String = "".to_string();

    // Extract nanopub URL and head graph
    let nanopublication_object_term = TermRef::NamedNode(np::NANOPUBLICATION);
    for q in dataset.quads_match(&[], &[rdf::TYPE], &[nanopublication_object_term], &[]) {
        if !np_url.is_empty() {
            return Err(NpError("The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string()));
        } else {
            np_url = subject_iri_to_string(q.subject)?;
            head = graph_iri_to_string(q.graph_name)?;
        }
    }
    if np_url.is_empty() {
        return Err(NpError(
            "The provided RDF does not contain a Nanopublication.".to_string(),
        ));
    }
    if head.is_empty() {
        return Err(NpError("Invalid Nanopub: no Head graph found.".to_string()));
    }

    let np_iri = NamedNode::new_unchecked(np_url);
    let head_iri = NamedNode::new_unchecked(head);
    let mut np_subject_term = NamedOrBlankNodeRef::from(np_iri.as_ref());
    let head_graph = GraphNameRef::from(head_iri.as_ref());

    // Extract assertion, prov, pubinfo, and head graphs URLs
    for q in dataset.quads_match(&[np_subject_term], &[np::HAS_ASSERTION], &[], &[head_graph]) {
        assertion = object_iri_to_string(q.object)?;
    }
    for q in dataset.quads_match(
        &[np_subject_term],
        &[np::HAS_PROVENANCE],
        &[],
        &[head_graph],
    ) {
        prov = object_iri_to_string(q.object)?;
    }
    for q in dataset.quads_match(
        &[np_subject_term],
        &[np::HAS_PUBLICATION_INFO],
        &[],
        &[head_graph],
    ) {
        pubinfo = object_iri_to_string(q.object)?;
    }

    if assertion.is_empty() {
        return Err(NpError(
            "Invalid Nanopub: no Assertion graph found.".to_string(),
        ));
    }
    if prov.is_empty() {
        return Err(NpError(
            "Invalid Nanopub: no Provenance graph found.".to_string(),
        ));
    }
    if pubinfo.is_empty() {
        return Err(NpError(
            "Invalid Nanopub: no PubInfo graph found.".to_string(),
        ));
    }

    // Get just the Trusty hash from the URI
    let mut trusty_hash: String = "".to_string();
    let re_trusty = Regex::new(r"^.*?[/#\.]?(RA[a-zA-Z0-9-_]*)$")?;
    if let Some(caps) = re_trusty.captures(&np_iri.as_str()) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        trusty_hash = caps.get(1).map_or("", |m| m.as_str()).to_string();
    }

    // Getting potential ns from head graph (removing the last frag from head)
    let original_ns = if trusty_hash.is_empty() {
        &head_iri.as_str()[..np_iri.as_str().len()]
    } else {
        &head_iri.as_str()[..np_iri.as_str().len() + 1]
    };
    let np_ns_str = original_ns;

    // Remove last char if it is # or / to get the URI
    let np_iri =
        if np_iri.as_str().ends_with(['#', '/', '.']) {
            match np_iri.as_str().chars().last() {
                Some(_) => NamedNode::new_unchecked(np_iri.as_str()[..np_iri.as_str().len() - 1].to_string()),
                None => np_iri,
            }
        } else {
            np_iri
        };
    np_subject_term = NamedOrBlankNodeRef::from(np_iri.as_ref());

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
    } else if !original_ns.ends_with(['#', '/', '.'])
    {
        format!("{}.", &original_ns)
    } else {
        original_ns.to_string()
    };

    // Extract signature and its subject URI
    let signature_string = format!("{}sig", np_ns_str);
    let pubinfo_iri = NamedNode::new_unchecked(pubinfo);
    let pubinfo_graph = GraphNameRef::from(pubinfo_iri.as_ref());
    let mut signature: String = "".to_string();
    let mut signature_iri = NamedNode::new_unchecked(signature_string);
    for q in dataset.quads_match(&[], &[npx::HAS_SIGNATURE], &[], &[pubinfo_graph]) {
        let (val, _, _) = object_literal_to_strings(q.object)?;
        signature = val;
        signature_iri = NamedNode::new_unchecked(subject_iri_to_string(q.subject)?);
    }
    let signature_node = NamedOrBlankNodeRef::from(signature_iri.as_ref());

    // Extract public key
    let mut pubkey: Option<String> = None;
    for q in dataset.quads_match(
        &[signature_node],
        &[npx::HAS_PUBLIC_KEY],
        &[],
        &[pubinfo_graph],
    ) {
        let (val, _, _) = object_literal_to_strings(q.object)?;
        pubkey = Some(val);
    }

    // Extract algo
    let mut algo: Option<String> = None;
    for q in dataset.quads_match(
        &[signature_node],
        &[npx::HAS_ALGORITHM],
        &[],
        &[pubinfo_graph],
    ) {
        let (val, _, _) = object_literal_to_strings(q.object)?;
        algo = Some(val);
    }

    // Extract ORCID
    let mut orcid: Option<String> = None;
    let original_ns_iri = NamedNode::new_unchecked(original_ns.to_string());
    let original_ns_subject_term = NamedOrBlankNodeRef::from(original_ns_iri.as_ref());
    for q in dataset.quads_match(
        &[np_subject_term, original_ns_subject_term],
        &[dct::CREATOR, prov::WAS_ATTRIBUTED_TO, pav::CREATED_BY],
        &[],
        &[pubinfo_graph],
    ) {
        let (val, _, _) = object_literal_to_strings(q.object)?;
        orcid = Some(val);
    }

    let assertion_iri = NamedNode::new_unchecked(assertion);
    let prov_iri = NamedNode::new_unchecked(prov);

    Ok(NpInfo {
        uri: np_iri,
        ns: NamedNode::new_unchecked(np_ns_str),
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
