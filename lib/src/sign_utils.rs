use crate::utils::NpError;

use base64;
use base64::{alphabet, engine, Engine as _};
use regex::Regex;
use rsa::{sha2::Digest, sha2::Sha256};
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::quad::Quad;
use sophia::api::term::Term;
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use std::collections::HashMap;
use std::error::Error;
use std::{cmp::Ordering, str};

/// Generate TrustyURI using base64 encoding
pub fn make_trusty(dataset: &LightDataset, base_ns: &str) -> Result<String, NpError> {
    let norm_quads = normalize_dataset(&dataset, base_ns, "")
        .expect("Failed to normalise RDF after adding signature");
    // println!("NORMED QUADS AFTER SIGNING\n{}", norm_quads);

    let base64_engine = engine::GeneralPurpose::new(
        &alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        )
        .unwrap(),
        engine::GeneralPurposeConfig::new().with_encode_padding(false),
    );
    let trusty_hash = format!(
        "RA{}",
        base64_engine.encode(Sha256::digest(norm_quads.as_bytes()))
    );
    Ok(trusty_hash)
}

/// Replace bnodes by URI ending with `_1` in the RDF dataset
pub fn replace_bnodes(dataset: &LightDataset, base_ns: &str) -> Result<LightDataset, NpError> {
    let mut new_dataset = LightDataset::new();
    let mut bnode_map: HashMap<String, usize> = HashMap::new();
    let mut bnode_counter = 1;
    let re_underscore_uri = Regex::new(&format!(r"{}(_+\d+)$", base_ns)).unwrap();

    for quad in dataset.quads() {
        let quad = quad.unwrap();

        // Replace bnode in subjects
        let subject = if quad.s().is_blank_node() {
            let bnode_id = quad.s().bnode_id().unwrap().to_string();
            bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                let counter = bnode_counter;
                bnode_counter += 1;
                counter
            });
            format!("{}_{}", base_ns, bnode_map[&bnode_id])
        } else if let Some(mat) = re_underscore_uri.find(&quad.s().iri().unwrap().as_ref()) {
            let mut subject_iri = quad.s().iri().unwrap().to_string();
            let new_ending = mat.as_str().replacen('_', "__", 1);
            subject_iri.truncate(subject_iri.len() - mat.as_str().len()); // Remove the original ending
            subject_iri.push_str(&new_ending);
            subject_iri
        } else {
            quad.s().iri().unwrap().to_string()
        };

        // Replace bnode in objects
        if quad.o().is_blank_node() {
            let bnode_id = quad.o().bnode_id().unwrap().to_string();
            bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                let counter = bnode_counter;
                bnode_counter += 1;
                counter
            });
            let object = format!("{}_{}", base_ns, bnode_map[&bnode_id]);
            new_dataset
                .insert(
                    &Iri::new_unchecked(subject),
                    quad.p(),
                    &Iri::new_unchecked(object),
                    quad.g(),
                )
                .unwrap();
        } else if quad.o().is_iri() {
            // Handle URI ending with #_1 to double _
            if let Some(mat) = re_underscore_uri.find(&quad.o().iri().unwrap().as_ref()) {
                let mut object_iri = quad.s().iri().unwrap().to_string();
                let new_ending = mat.as_str().replacen('_', "__", 1);
                object_iri.truncate(object_iri.len() - mat.as_str().len()); // Remove the original ending
                object_iri.push_str(&new_ending);
                new_dataset
                    .insert(
                        &Iri::new_unchecked(subject),
                        quad.p(),
                        &Iri::new_unchecked(object_iri),
                        quad.g(),
                    )
                    .unwrap();
            } else {
                new_dataset
                    .insert(&Iri::new_unchecked(subject), quad.p(), quad.o(), quad.g())
                    .unwrap();
            }
        } else {
            new_dataset
                .insert(&Iri::new_unchecked(subject), quad.p(), quad.o(), quad.g())
                .unwrap();
        };
    }
    Ok(new_dataset)
}

/// Replace the dummy nanopub URI by the new one in the RDF dataset
pub fn replace_ns_in_quads(
    dataset: &LightDataset,
    old_ns: &str,
    old_uri: &str,
    new_ns: &str,
    new_uri: &str,
) -> Result<LightDataset, NpError> {
    let mut new = LightDataset::new();
    for quad in dataset.quads() {
        let quad = quad.unwrap();
        let s = quad.s().iri().unwrap().to_string();
        // Replace URI in subjects
        let subject = if s == old_ns || s == old_uri {
            Iri::new_unchecked(new_uri.to_string())
        } else {
            Iri::new_unchecked(s.replace(old_ns, new_ns))
        };
        // Replace URI in graphs
        let graph = Some(Iri::new_unchecked(
            quad.g()
                .unwrap()
                .iri()
                .unwrap()
                .to_string()
                .replace(old_ns, new_ns),
        ));

        // Replace URI in objects
        if quad.o().is_iri() {
            let o = quad.o().iri().unwrap().to_string();
            if o == old_ns || o == old_uri {
                new.insert(
                    &subject,
                    quad.p(),
                    &Iri::new_unchecked(new_uri.to_string()),
                    graph,
                )
                .unwrap();
            } else {
                new.insert(
                    &subject,
                    quad.p(),
                    &Iri::new_unchecked(o.replace(old_ns, new_ns)),
                    graph,
                )
                .unwrap();
            }
        } else {
            new.insert(&subject, quad.p(), quad.o(), graph).unwrap();
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
    // Lang,
}

struct NormQuad {
    graph: String,
    subject: String,
    predicate: String,
    object: String,
    datatype: String,
    lang: String,
}

/// Returns all the quads contained in the nanopub.
pub fn normalize_dataset(
    dataset: &LightDataset,
    base_ns: &str,
    _hash_str: &str,
) -> Result<String, Box<dyn Error>> {
    let mut quads_vec: Vec<NormQuad> = vec![];
    // let norm_base = "http://purl.org/np/ ";
    let norm_base = "https://w3id.org/np/ ";
    let base_uri = match base_ns.chars().last() {
        Some(_) => &base_ns[..base_ns.len() - 1],
        None => base_ns,
    };

    // Convert dataset to a list of NormQuad struct
    for quad in dataset.quads() {
        let quad = quad.unwrap();

        // Extract components of the quad and convert them to strings. Replace the base URI if present
        let graph = if quad.g().unwrap().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.g()
                .unwrap()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, norm_base)
        };

        let subject = if quad.s().is_blank_node() {
            quad.s().bnode_id().unwrap().to_string()
        } else if quad.s().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.s()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, norm_base)
        };

        let predicate = if quad.p().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.p()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, norm_base)
        };

        let object = if quad.o().is_iri() {
            if quad.o().iri().unwrap().to_string() == base_ns {
                norm_base.to_string()
            } else {
                quad.o()
                    .iri()
                    .unwrap()
                    .to_string()
                    .replace(base_uri, norm_base)
            }
        } else if quad.o().is_blank_node() {
            quad.o().bnode_id().unwrap().to_string()
        } else {
            quad.o().lexical_form().unwrap().to_string()
        };

        // Extract datatype and language if available
        let datatype = if quad.o().datatype().is_some() {
            quad.o().datatype().unwrap().to_string()
        } else {
            "".to_string()
        };
        let lang = if quad.o().language_tag().is_some() {
            quad.o().language_tag().unwrap().to_string()
        } else {
            "".to_string()
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
    let orders = [Graph, Subject, Predicate, Object, Datatype];
    quads_vec.sort_by(|a, b| {
        orders.iter().fold(Ordering::Equal, |acc, &field| {
            acc.then_with(|| match field {
                Graph => a.graph.cmp(&b.graph),
                Subject => a.subject.cmp(&b.subject),
                Predicate => a.predicate.cmp(&b.predicate),
                Object => a.object.cmp(&b.object),
                Datatype => a.datatype.cmp(&b.datatype),
                Lang => a.lang.cmp(&b.lang),
            })
        })
    });
    // println!(quads_vec);

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
