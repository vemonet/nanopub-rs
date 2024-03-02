use crate::constants::{NP_PREF_NS, NP_TEMP_URI};
use crate::error::{NpError, TermError};
use crate::utils::ns;

use regex::Regex;
use serde::{Serialize, Serializer};
use sophia::api::dataset::Dataset;
use sophia::api::ns::{rdf, Namespace};
use sophia::api::quad::Quad;
use sophia::api::term::{matcher::Any, Term};
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use std::fmt;

/// Infos extracted from a nanopublication: graphs URLs, signature, trusty hash...
#[derive(Clone, Serialize, Debug)]
pub struct NpInfo {
    pub uri: Iri<String>,
    #[serde(serialize_with = "serialize_namespace")]
    pub ns: Namespace<String>,
    pub normalized_ns: String,
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
    pub published: Option<String>,
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

/// Extract graphs URLs from a nanopub: nanopub URL, head, assertion, prov, pubinfo
pub fn extract_np_info(dataset: &LightDataset) -> Result<NpInfo, NpError> {
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

    // Get just the Trusty hash from the URI
    let mut trusty_hash: String = "".to_string();
    let re_trusty = Regex::new(r"^.*?[/#\.]?(RA[a-zA-Z0-9-_]*)$")?;
    if let Some(caps) = re_trusty.captures(&np_iri.as_ref()) {
        // The first group captures everything up to a '/' or '#', non-greedy.
        trusty_hash = caps.get(1).map_or("", |m| m.as_str()).to_string();
    }

    // Getting potential ns from head graph (removing the last frag from head)
    let original_ns = if trusty_hash.is_empty() {
        &head_iri[..np_iri.len()]
    } else {
        &head_iri[..np_iri.len() + 1]
    };
    let np_ns = Namespace::new_unchecked(original_ns.to_string());

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
        separator_after_trusty = "#".to_string()
    };

    // Generate normalized namespace without trusty
    let norm_ns = if !trusty_hash.is_empty() {
        format!("{}{}", base_uri, separator_before_trusty)
    } else if original_ns.starts_with(NP_TEMP_URI) {
        NP_PREF_NS.to_string()
    } else if !original_ns.ends_with('#')
        && !original_ns.ends_with('/')
        && !original_ns.ends_with('.')
    {
        format!("{}.", &original_ns)
    } else {
        original_ns.to_string()
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
        [&np_iri, &Iri::new_unchecked(original_ns.to_string())],
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

    let assertion_iri = Iri::new_unchecked(assertion);
    let prov_iri = Iri::new_unchecked(prov);

    Ok(NpInfo {
        uri: np_iri,
        ns: np_ns,
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

// Custom serialize function for Namespace
fn serialize_namespace<S>(ns: &Namespace<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(ns.as_str())
}
