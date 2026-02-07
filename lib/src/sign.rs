use crate::error::NpError;
use crate::utils::{graph_iri_to_string, subject_iri_to_string};

use base64::{alphabet, engine, Engine as _};
use oxrdf::{
    Dataset, GraphNameRef, NamedNode, NamedNodeRef, NamedOrBlankNode, NamedOrBlankNodeRef, QuadRef,
    TermRef,
};
use regex::Regex;
use rsa::{sha2::Digest, sha2::Sha256};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Generate TrustyURI using base64 encoding
pub fn make_trusty(
    dataset: &Dataset,
    base_ns: &str,
    norm_ns: &str,
    separator: &str,
) -> Result<String, NpError> {
    let norm_quads = normalize_dataset(dataset, base_ns, norm_ns, separator)?;
    // println!("NORMED QUADS MAKE TRUSTY\n{}", norm_quads);
    let base64_engine = engine::GeneralPurpose::new(
        &alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        )?,
        engine::GeneralPurposeConfig::new().with_encode_padding(false),
    );
    Ok(format!(
        "RA{}",
        base64_engine.encode(Sha256::digest(norm_quads.as_bytes()))
    ))
}

/// Replace bnodes by URI ending with `_1` in the RDF dataset
pub fn replace_bnodes(
    dataset: &Dataset,
    base_ns: &str,
    base_uri: &str,
) -> Result<Dataset, NpError> {
    let mut new_dataset = Dataset::new();
    let mut bnode_map: HashMap<String, usize> = HashMap::new();
    let mut bnode_count = 1;
    let re_underscore_uri = Regex::new(&format!(r"{base_uri}.?(_+[a-zA-Z0-9^_]+)$"))?;

    for quad in dataset.iter() {
        // Replace bnode in subjects, and add 1 underscore for URI using already underscore
        let subject_str = match quad.subject {
            NamedOrBlankNodeRef::BlankNode(bnode) => {
                let bnode_id = bnode.as_str();
                bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                    let counter = bnode_count;
                    bnode_count += 1;
                    counter
                });
                format!("{}_{}", base_ns, bnode_map[bnode_id])
            }
            NamedOrBlankNodeRef::NamedNode(named) => {
                let mut subject_iri_str = named.as_str().to_owned();
                if let Some(caps) = re_underscore_uri.captures(&subject_iri_str) {
                    let matching = caps
                        .get(1)
                        .ok_or(NpError("Error with regex".to_string()))?
                        .as_str();
                    let new_ending = matching.replacen('_', "__", 1);
                    subject_iri_str.truncate(subject_iri_str.len() - matching.len()); // Remove the original ending
                    subject_iri_str.push_str(&new_ending);
                }
                subject_iri_str
            }
        };

        let GraphNameRef::NamedNode(graph_iri) = quad.graph_name else {
            return Err(NpError("Failed to extract graph name IRI.".to_string()));
        };
        let graph_node = if let Some(caps) = re_underscore_uri.captures(graph_iri.as_str()) {
            let mut graph_string = graph_iri.into_owned().into_string();
            let matching = caps
                .get(1)
                .ok_or(NpError("Error with regex".to_string()))?
                .as_str();
            let new_ending = matching.replacen('_', "__", 1);
            graph_string.truncate(graph_string.len() - matching.len()); // Remove the original ending
            graph_string.push_str(&new_ending);
            &NamedNode::new_unchecked(graph_string)
        } else {
            &graph_iri.into_owned()
        };

        // Replace bnode in objects
        let subject_node = NamedNodeRef::new_unchecked(subject_str.as_str());
        // let object = quad.object;
        match quad.object {
            TermRef::BlankNode(bnode) => {
                let bnode_id = bnode.as_str();
                bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                    let counter = bnode_count;
                    bnode_count += 1;
                    counter
                });
                let object_string = format!("{}_{}", base_ns, bnode_map[bnode_id]);
                let object_node = NamedNodeRef::new_unchecked(object_string.as_str());
                new_dataset.insert(QuadRef::new(
                    subject_node,
                    quad.predicate,
                    object_node,
                    graph_node,
                ));
            }
            TermRef::NamedNode(named) => {
                let object_iri = named.as_str();
                // Handle URI ending with #_ to double _
                if let Some(caps) = re_underscore_uri.captures(object_iri) {
                    let mut object_string = object_iri.to_string();
                    let matching = caps
                        .get(1)
                        .ok_or(NpError("Error with regex".to_string()))?
                        .as_str();
                    let new_ending = matching.replacen('_', "__", 1);
                    object_string.truncate(object_string.len() - matching.len()); // Remove the original ending
                    object_string.push_str(&new_ending);
                    let object_node = NamedNodeRef::new_unchecked(object_string.as_str());
                    new_dataset.insert(QuadRef::new(
                        subject_node,
                        quad.predicate,
                        object_node,
                        graph_node,
                    ));
                } else {
                    new_dataset.insert(QuadRef::new(
                        subject_node,
                        quad.predicate,
                        named,
                        graph_node,
                    ));
                }
            }
            _ => {
                new_dataset.insert(QuadRef::new(
                    subject_node,
                    quad.predicate,
                    quad.object,
                    graph_node,
                ));
            }
        };
    }
    Ok(new_dataset)
}

/// Replace the dummy nanopub URI by the new one in the RDF dataset
pub fn replace_ns_in_quads(
    dataset: &Dataset,
    old_ns: &str,
    old_uri: &str,
    new_ns: &str,
    new_uri: &str,
) -> Result<Dataset, NpError> {
    let mut new = Dataset::new();
    for quad in dataset.iter() {
        let s = subject_iri_to_string(quad.subject)?;
        // Replace URI in subjects
        let subject_node = if s == old_ns || s == old_uri {
            NamedOrBlankNode::from(NamedNodeRef::new_unchecked(new_uri))
        } else {
            NamedOrBlankNode::from(NamedNodeRef::new_unchecked(
                s.replace(old_ns, new_ns).as_str(),
            ))
        };
        // Replace URI in graphs
        let graph_name = graph_iri_to_string(quad.graph_name)?.replace(old_ns, new_ns);
        let graph = NamedNodeRef::new_unchecked(graph_name.as_str());

        // Replace URI in objects
        match quad.object {
            TermRef::NamedNode(iri) => {
                let o = iri.as_str();
                let object_node = if o == old_ns || o == old_uri {
                    NamedNode::new_unchecked(new_uri)
                } else {
                    let new_uri_string = o.replace(old_ns, new_ns);
                    NamedNode::new_unchecked(new_uri_string.to_string())
                };
                new.insert(QuadRef::new(
                    &subject_node,
                    quad.predicate,
                    &object_node,
                    graph,
                ));
            }
            _ => {
                new.insert(QuadRef::new(
                    &subject_node,
                    quad.predicate,
                    quad.object,
                    graph,
                ));
            }
        };
    }
    Ok(new)
}

#[derive(Debug, Copy, Clone)]
enum Field {
    Graph,
    Subject,
    Predicate,
    Object,
    Datatype,
    Lang,
}

struct NormQuad {
    graph: String,
    subject: String,
    predicate: String,
    object: String,
    datatype: String,
    lang: String,
}

/// Fix normed URIs last fragments. Make sure it starts with #
pub fn fix_normed_uri(uri: &str, separator: &str) -> String {
    if let Some(space_index) = uri.rfind(' ') {
        let last_frag = &uri[space_index + 1..];
        if uri.ends_with(&format!(" {separator}")) || last_frag.is_empty() {
            uri.strip_suffix(separator).unwrap_or(uri).to_string()
        } else if last_frag.starts_with(separator) {
            uri.to_string()
        } else {
            format!("{} {separator}{}", &uri[..space_index], last_frag)
        }
    } else {
        uri.to_string()
    }
}

/// Normalize the quads contained in the nanopub dataset to a string used for signing and generating trusty
pub fn normalize_dataset(
    dataset: &Dataset,
    base_ns: &str,
    norm_ns: &str,
    separator: &str,
) -> Result<String, NpError> {
    let mut quads_vec: Vec<NormQuad> = vec![];
    let norm_uri = format!("{norm_ns} ");
    // println!("DEBUG: NORMALIZE {} {} {}", base_ns, norm_ns, separator);
    // Example already signed: http://www.nextprot.org/nanopubs#NX_Q9Y6K8_ESTEvidence_TS-2083.RAr9ao0vjXtLf3d9U4glE_uQWSknfYoPlIzKBq6ybOO5k.
    // Not signed yet: http://www.proteinatlas.org/about/nanopubs/ENSG00000000003_ih_TS_0030_head
    //   becomes http://www.proteinatlas.org/about/nanopubs/ENSG00000000003_ih_TS_0030.RAyBeXMqokAQZ5psoETKtkOeYzHnoIoXTgNFKRdLM8yzs#__head
    //   last char after trusty becomes # and before .
    // Default tmp URI: http://purl.org/nanopub/temp/
    //   becomes: https://w3id.org/np/RAyBeXMqokAQZ5psoETKtkOeYzHnoIoXTgNFKRdLM8yzs#Head

    // Convert dataset to a list of NormQuad struct
    for quad in dataset.iter() {
        // Extract components of the quad and convert them to strings. Replace the base URI if present
        let graph = fix_normed_uri(
            &graph_iri_to_string(quad.graph_name)?.replace(base_ns, &norm_uri),
            separator,
        );

        let mut datatype = "".to_string();
        let mut lang = "".to_string();

        let subject = if subject_iri_to_string(quad.subject)? == base_ns {
            norm_uri.clone()
        } else {
            fix_normed_uri(
                &subject_iri_to_string(quad.subject)?.replace(base_ns, &norm_uri),
                separator,
            )
        };

        let predicate = quad
            .predicate
            .into_owned()
            .into_string()
            .replace(base_ns, &norm_uri);

        let object = match quad.object {
            TermRef::NamedNode(iri) => {
                if iri.as_str() == base_ns {
                    norm_uri.to_string()
                } else {
                    fix_normed_uri(&iri.as_str().replace(base_ns, &norm_uri), separator)
                }
            }
            TermRef::Literal(literal) => {
                // Extract datatype and language if available
                datatype = literal.datatype().into_owned().into_string();
                lang = literal.language().unwrap_or_default().to_owned();
                // Double the \\ to bypass rust escaping
                literal.value().replace('\\', "\\\\").replace('\n', "\\n")
            }
            other => {
                return Err(NpError(format!(
                    "Failed to extract literal from object: Got {other:?}"
                )));
            }
        };

        // Create a NormQuad struct and push it to the vector
        quads_vec.push(NormQuad {
            graph,
            subject,
            predicate,
            object,
            datatype,
            lang,
        });
    }

    // Order the list of nquads
    use Field::*;
    let orders = [Graph, Subject, Predicate, Lang, Datatype, Object];
    quads_vec.sort_by(|a, b| {
        orders.iter().fold(Ordering::Equal, |acc, &field| {
            acc.then_with(|| match field {
                Graph => a.graph.cmp(&b.graph),
                Subject => a.subject.cmp(&b.subject),
                Predicate => a.predicate.cmp(&b.predicate),
                Lang => a.lang.cmp(&b.lang),
                Datatype => a.datatype.cmp(&b.datatype),
                Object => a.object.cmp(&b.object),
            })
        })
    });

    // Format the ordered list in the normalized string that will be encrypted
    let mut normed_quads = String::new();
    for quad in quads_vec {
        normed_quads.push_str(&format!("{}\n", quad.graph));
        normed_quads.push_str(&format!("{}\n", quad.subject));
        normed_quads.push_str(&format!("{}\n", quad.predicate));

        let formatted_object = if !quad.lang.is_empty() {
            format!("@{} {}", quad.lang, quad.object)
        } else if !quad.datatype.is_empty() {
            format!("^{} {}", quad.datatype, quad.object)
        } else {
            quad.object
        };
        normed_quads.push_str(&formatted_object);
        normed_quads.push('\n');
    }
    Ok(normed_quads)
}
