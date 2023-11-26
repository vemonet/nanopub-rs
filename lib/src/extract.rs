use crate::error::{NpError, TermError};
use crate::utils::ns;

use regex::Regex;
use sophia::api::dataset::Dataset;
use sophia::api::ns::{rdf, Namespace};
use sophia::api::quad::Quad;
use sophia::api::term::{matcher::Any, Term};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use std::collections::HashSet;
use std::fmt;

/// Infos extracted from a nanopublication: graphs URLs, signature, trusty hash...
#[derive(Clone)]
pub struct NpInfo {
    pub uri: Iri<String>,
    pub ns: Namespace<String>,
    pub head: Iri<String>,
    pub assertion: Iri<String>,
    pub prov: Iri<String>,
    pub pubinfo: Iri<String>,
    pub base_uri: String,
    pub separator_before_trusty: String,
    pub separator_after_trusty: String,
    pub trusty_hash: String,
    pub signature: String,
    pub signature_iri: Iri<String>,
    pub algo: String,
    pub public_key: String,
    pub orcid: String,
}

impl fmt::Display for NpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nNanopub URI: {}", self.uri)?;
        writeln!(f, "Namespace: {}", *self.ns)?;
        writeln!(f, "Base URI: {}", self.base_uri)?;
        writeln!(f, "Trusty Hash: {}", self.trusty_hash)?;
        writeln!(f, "ORCID: {}", self.orcid)?;
        writeln!(f, "Head Graph: {}", self.head)?;
        writeln!(f, "Assertion Graph: {}", self.assertion)?;
        Ok(())
    }
}

// TODO: separate funtion just to extract uri and ns (to pass to serialize_rdf())
// pub fn extract_np_uri(dataset: &LightDataset) -> Result<(String, String), NpError> {
//     let mut np_url: String = "".to_string();
// }

/// Extract graphs URLs from a nanopub: nanopub URL, head, assertion, prov, pubinfo
pub fn extract_np_info(dataset: &LightDataset, check_pubinfo: bool) -> Result<NpInfo, NpError> {
    let mut np_url: String = "".to_string();
    let mut head: String = "".to_string();
    let mut assertion: String = "".to_string();
    let mut prov: String = "".to_string();
    let mut pubinfo: String = "".to_string();

    // Extract nanopub URL and head graph
    for q in dataset.quads_matching(Any, [&rdf::type_], [ns("np").get("Nanopublication")?], Any) {
        if !np_url.is_empty() {
            return Err(NpError("The provided RDF contains multiple Nanopublications. Only one can be provided at a time.".to_string()));
        } else {
            np_url = q?.s().iri().ok_or(TermError())?.to_string();
            head = q?
                .g()
                .ok_or(TermError())?
                .iri()
                .ok_or(TermError())?
                .to_string();
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

    let np_iri: Iri<String> = Iri::new_unchecked(np_url);
    let head_iri: Iri<String> = Iri::new_unchecked(head);

    // Extract assertion, prov, pubinfo, and head graphs URLs
    for q in dataset.quads_matching(
        [&np_iri],
        [ns("np").get("hasAssertion")?],
        Any,
        [Some(&head_iri)],
    ) {
        assertion = q?.o().iri().ok_or(TermError())?.to_string();
    }
    for q in dataset.quads_matching(
        [&np_iri],
        [ns("np").get("hasProvenance")?],
        Any,
        [Some(&head_iri)],
    ) {
        prov = q?.o().iri().ok_or(TermError())?.to_string();
    }
    for q in dataset.quads_matching(
        [&np_iri],
        [ns("np").get("hasPublicationInfo")?],
        Any,
        [Some(&head_iri)],
    ) {
        pubinfo = q?.o().iri().ok_or(TermError())?.to_string();
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

    // Remove last char if it is # or / to get the URI
    let np_iri: Iri<String> =
        if np_iri.ends_with('#') || np_iri.ends_with('/') || np_iri.ends_with('.') {
            match np_iri.chars().last() {
                Some(_) => Iri::new_unchecked(np_iri.to_string()[..np_iri.len() - 1].to_string()),
                None => np_iri,
            }
        } else {
            np_iri
        };

    // Getting potential ns from head graph (removing the last frag from head)
    let np_ns_str = &head_iri[..np_iri.len() + 1];

    // Extract base URI, separator character (# or / or _), and trusty hash (if present) from the np URL
    // Default to empty strings when nothing found
    let mut base_uri: String = "".to_string();
    let mut separator_before_trusty: String = '.'.to_string();
    let mut separator_after_trusty: String = "".to_string();
    let mut trusty_hash: String = "".to_string();

    // Get just the Trusty hash from the URI
    let re_trusty = Regex::new(r"^.*?[/#\.]?(RA[a-zA-Z0-9-_]*)$")?;
    if let Some(caps) = re_trusty.captures(&np_iri.as_ref()) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        trusty_hash = caps.get(1).map_or("", |m| m.as_str()).to_string();
    }

    // Get the base URI and separators from the namespace
    let re_trusty_ns = Regex::new(r"^(.*?)(/|#|\.)?(RA[a-zA-Z0-9-_]*)?([#/\.])?$")?;
    // let re = Regex::new(r"^(.*?)(RA.*)?$")?;
    if let Some(caps) = re_trusty_ns.captures(np_ns_str) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        base_uri = caps.get(1).map_or("", |m| m.as_str()).to_string();
        // The second group captures '/' or '#' if present, defaults to .
        separator_before_trusty = caps
            .get(2)
            .map_or(separator_before_trusty, |m| m.as_str().to_string())
            .to_string();
        // The last group captures everything after 'RA', if present.
        // trusty_hash = caps.get(3).map_or("", |m| m.as_str()).to_string();
        separator_after_trusty = caps
            .get(4)
            .map_or(separator_after_trusty, |m| m.as_str().to_string())
            .to_string();
    }
    if trusty_hash.is_empty() && separator_after_trusty.is_empty() {
        separator_after_trusty = "#".to_string()
    };

    // TODO: handle diff if trusty or not (if not we use default, if trusty we only extract)
    let np_ns =
        if !np_ns_str.ends_with('#') && !np_ns_str.ends_with('/') && !np_ns_str.ends_with('.') {
            if !trusty_hash.is_empty() {
                // TODO: Change the after trusty part?
                Namespace::new_unchecked(np_ns_str.to_string())
            } else {
                Namespace::new_unchecked(format!(
                    "{}.",
                    &np_ns_str.strip_suffix('_').unwrap_or(np_ns_str)
                ))
            }
        } else {
            Namespace::new_unchecked(np_ns_str.to_string())
        };

    // Extract signature and its subject URI
    let pubinfo_iri: Iri<String> = Iri::new_unchecked(pubinfo);
    let mut signature: String = "".to_string();
    let mut signature_iri: Iri<String> = Iri::new_unchecked(np_ns.get("sig")?.to_string());
    for q in dataset.quads_matching(
        Any,
        [ns("npx").get("hasSignature")?],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        signature = q?.o().lexical_form().ok_or(TermError())?.to_string();
        signature_iri = Iri::new_unchecked(q?.s().iri().ok_or(TermError())?.to_string());
    }

    // Extract public key
    let mut pubkey: Option<String> = None;
    for q in dataset.quads_matching(
        [&signature_iri],
        [ns("npx").get("hasPublicKey")?],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        pubkey = Some(q?.o().lexical_form().ok_or(TermError())?.to_string());
    }

    // Extract algo
    let mut algo: Option<String> = None;
    for q in dataset.quads_matching(
        [&signature_iri],
        [ns("npx").get("hasAlgorithm")?],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        algo = Some(q?.o().lexical_form().ok_or(TermError())?.to_string());
    }

    // Extract ORCID
    let mut orcid: Option<String> = None;
    for q in dataset.quads_matching(
        [&np_iri, &Iri::new_unchecked(np_ns_str.to_string())],
        [
            ns("dct").get("creator")?,
            ns("prov").get("wasAttributedTo")?,
            ns("pav").get("createdBy")?,
        ],
        Any,
        [Some(&pubinfo_iri)],
    ) {
        orcid = Some(q?.o().iri().ok_or(TermError())?.to_string());
    }

    // Check minimal required triples in assertion, prov, pubinfo graphs
    let assertion_iri = Iri::new_unchecked(assertion);
    let prov_iri = Iri::new_unchecked(prov);
    if dataset
        .quads_matching(Any, Any, Any, [Some(assertion_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the assertion graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching(Any, Any, Any, [Some(prov_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the provenance graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching([assertion_iri.clone()], Any, Any, [Some(prov_iri.clone())])
        .next()
        .is_none()
    {
        return Err(NpError("Invalid Nanopub: no triples with the assertion graph as subject in the provenance graph.".to_string()));
    }
    if check_pubinfo {
        if dataset
            .quads_matching(Any, Any, Any, [Some(pubinfo_iri.clone())])
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples in the pubinfo graph.".to_string(),
            ));
        }
        if dataset
            .quads_matching(
                [
                    np_iri.clone(),
                    Iri::new_unchecked(np_ns.get("")?.to_string()),
                ],
                Any,
                Any,
                [Some(pubinfo_iri.clone())],
            )
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples with the nanopub URI as subject in the pubinfo graph."
                    .to_string(),
            ));
        }
    }
    let mut graph_names = HashSet::new();
    for g in dataset.graph_names() {
        if let Some(graph_name) = g?.iri() {
            graph_names.insert(graph_name.to_string());
        }
    }
    if graph_names.len() > 4 {
        return Err(NpError(
            format!("Invalid Nanopub: it should have 4 graphs (head, assertion, provenance, pubinfo), but the given nanopub has {} graphs.", graph_names.len())
        ));
    }

    let np_info = NpInfo {
        uri: np_iri,
        ns: np_ns,
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
    };
    // let _ = check_np_info(dataset, &np_info, check_pubinfo);
    Ok(np_info)
}

pub fn check_np_info(
    dataset: &LightDataset,
    np_info: &NpInfo,
    check_pubinfo: bool,
) -> Result<(), NpError> {
    // Check minimal required triples in assertion, prov, pubinfo graphs
    if dataset
        .quads_matching(Any, Any, Any, [Some(np_info.assertion.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the assertion graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching(Any, Any, Any, [Some(np_info.prov.clone())])
        .next()
        .is_none()
    {
        return Err(NpError(
            "Invalid Nanopub: no triples in the provenance graph.".to_string(),
        ));
    }
    if dataset
        .quads_matching(
            [np_info.assertion.clone()],
            Any,
            Any,
            [Some(np_info.prov.clone())],
        )
        .next()
        .is_none()
    {
        return Err(NpError("Invalid Nanopub: no triples with the assertion graph as subject in the provenance graph.".to_string()));
    }
    if check_pubinfo {
        if dataset
            .quads_matching(Any, Any, Any, [Some(np_info.pubinfo.clone())])
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples in the pubinfo graph.".to_string(),
            ));
        }
        if dataset
            .quads_matching(
                [
                    np_info.uri.clone(),
                    Iri::new_unchecked(np_info.ns.get("")?.to_string()),
                ],
                Any,
                Any,
                [Some(np_info.pubinfo.clone())],
            )
            .next()
            .is_none()
        {
            return Err(NpError(
                "Invalid Nanopub: no triples with the nanopub URI as subject in the pubinfo graph."
                    .to_string(),
            ));
        };
    }
    let mut graph_names = HashSet::new();
    for g in dataset.graph_names() {
        if let Some(graph_name) = g?.iri() {
            graph_names.insert(graph_name.to_string());
        }
    }
    if graph_names.len() != 4 {
        return Err(NpError(
            format!("Invalid Nanopub: it should have 4 graphs (head, assertion, provenance, pubinfo), but the given nanopub has {} graphs.", graph_names.len())
        ));
    }
    Ok(())
}
