use crate::error::NpError;

use base64::{alphabet, engine, Engine as _};
use regex::Regex;
use rsa::{sha2::Digest, sha2::Sha256};
use sophia::api::dataset::{Dataset, MutableDataset};
use sophia::api::quad::Quad;
use sophia::api::term::Term;
use sophia::inmem::dataset::LightDataset;
use sophia::iri::Iri;
use std::collections::HashMap;
use std::{cmp::Ordering, str};

/// Generate TrustyURI using base64 encoding
pub fn make_trusty(
    dataset: &LightDataset,
    base_ns: &str,
    norm_ns: &str,
    separator: &str,
) -> Result<String, NpError> {
    let norm_quads = normalize_dataset(dataset, base_ns, norm_ns, separator)?;
    println!("NORMED QUADS MAKE TRUSTY\n{}", norm_quads);

    let base64_engine = engine::GeneralPurpose::new(
        &alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        )?,
        engine::GeneralPurposeConfig::new().with_encode_padding(false),
    );
    let trusty_hash = format!(
        "RA{}",
        base64_engine.encode(Sha256::digest(norm_quads.as_bytes()))
    );
    println!("TRUUUUUSTY {trusty_hash}");
    Ok(trusty_hash)
}

/// Replace bnodes by URI ending with `_1` in the RDF dataset
pub fn replace_bnodes(
    dataset: &LightDataset,
    base_ns: &str,
    base_uri: &str,
) -> Result<LightDataset, NpError> {
    let mut new_dataset = LightDataset::new();
    let mut bnode_map: HashMap<String, usize> = HashMap::new();
    let mut bnode_counter = 1;
    let re_underscore_uri = Regex::new(&format!(r"{}.?(_+[a-zA-Z0-9^_]+)$", base_uri))?;

    for quad in dataset.quads() {
        let quad = quad?;

        // Replace bnode in subjects, and add 1 underscore for URI using already underscore
        let subject = if quad.s().is_blank_node() {
            let bnode_id = quad.s().bnode_id().unwrap().to_string();
            bnode_map.entry(bnode_id.to_string()).or_insert_with(|| {
                let counter = bnode_counter;
                bnode_counter += 1;
                counter
            });
            format!("{}_{}", base_ns, bnode_map[&bnode_id])
        } else if let Some(caps) = re_underscore_uri.captures(&quad.s().iri().unwrap().as_ref()) {
            let mut subject_iri = quad.s().iri().unwrap().to_string();
            let matche = caps.get(1).unwrap().as_str();
            let new_ending = matche.replacen('_', "__", 1);
            subject_iri.truncate(subject_iri.len() - matche.len()); // Remove the original ending
            subject_iri.push_str(&new_ending);
            subject_iri
        } else {
            quad.s().iri().unwrap().to_string()
        };

        let graph = if let Some(caps) =
            re_underscore_uri.captures(&quad.g().unwrap().iri().unwrap().as_ref())
        {
            let mut graph_iri = quad.g().unwrap().iri().unwrap().to_string();
            let matche = caps.get(1).unwrap().as_str();
            let new_ending = matche.replacen('_', "__", 1);
            graph_iri.truncate(graph_iri.len() - matche.len()); // Remove the original ending
            graph_iri.push_str(&new_ending);
            Some(Iri::new_unchecked(graph_iri))
        } else {
            Some(Iri::new_unchecked(
                quad.g().unwrap().iri().unwrap().to_string(),
            ))
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
                    graph,
                )
                .unwrap();
        } else if quad.o().is_iri() {
            // Handle URI ending with #_ to double _
            if let Some(caps) = re_underscore_uri.captures(&quad.o().iri().unwrap().as_ref()) {
                let mut object_iri = quad.o().iri().unwrap().to_string();
                let matche = caps.get(1).unwrap().as_str();
                let new_ending = matche.replacen('_', "__", 1);
                object_iri.truncate(object_iri.len() - matche.len()); // Remove the original ending
                object_iri.push_str(&new_ending);
                new_dataset.insert(
                    &Iri::new_unchecked(subject),
                    quad.p(),
                    &Iri::new_unchecked(object_iri),
                    graph,
                )?;
            } else {
                new_dataset.insert(&Iri::new_unchecked(subject), quad.p(), quad.o(), graph)?;
            }
        } else {
            new_dataset.insert(&Iri::new_unchecked(subject), quad.p(), quad.o(), graph)?;
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
    let old_ns = old_ns.strip_suffix('.').unwrap_or(old_ns);
    // println!("IN REPLACE: {} {}", old_ns, new_ns);
    let mut new = LightDataset::new();
    for quad in dataset.quads() {
        let quad = quad?;
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
                )?;
            } else {
                new.insert(
                    &subject,
                    quad.p(),
                    &Iri::new_unchecked(o.replace(old_ns, new_ns)),
                    graph,
                )?;
            }
        } else {
            new.insert(&subject, quad.p(), quad.o(), graph)?;
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
    if let Some(last_slash_index) = uri.rfind(' ') {
        let last_frag = &uri[last_slash_index + 1..];
        // println!("IN fix_normed_uri {}", last_frag);
        if last_frag.starts_with(separator) || last_frag.is_empty() {
            uri.to_string()
        } else if last_frag.starts_with('/') || last_frag.starts_with('.') {
            format!(
                "{} {separator}{}",
                &uri[..last_slash_index],
                &uri[last_slash_index + 2..]
            )
        } else {
            format!("{} {separator}{}", &uri[..last_slash_index], last_frag)
        }
    } else {
        uri.to_string()
    }
    // TODO: of separator we do differently
}

/// Normalize the quads contained in the nanopub dataset to a string used for signing and generating trusty
pub fn normalize_dataset(
    dataset: &LightDataset,
    base_ns: &str,
    norm_ns: &str,
    separator: &str,
) -> Result<String, NpError> {
    let mut quads_vec: Vec<NormQuad> = vec![];
    let norm_base = format!("{} ", norm_ns.strip_suffix('#').unwrap_or(norm_ns));
    let base_uri = match base_ns.chars().last() {
        Some(_) => &base_ns[..base_ns.len() - 1],
        None => base_ns,
    };
    // Already signed: http://www.nextprot.org/nanopubs#NX_Q9Y6K8_ESTEvidence_TS-2083.RAr9ao0vjXtLf3d9U4glE_uQWSknfYoPlIzKBq6ybOO5k.
    // http://www.proteinatlas.org/about/nanopubs/ENSG00000000003_ih_TS_0030_head
    //   becomes http://www.proteinatlas.org/about/nanopubs/ENSG00000000003_ih_TS_0030.RAyBeXMqokAQZ5psoETKtkOeYzHnoIoXTgNFKRdLM8yzs#__head
    //   last char after trusty becomes # and before .

    // Convert dataset to a list of NormQuad struct
    for quad in dataset.quads() {
        let quad = quad?;
        // Extract components of the quad and convert them to strings. Replace the base URI if present
        let graph = if quad.g().unwrap().iri().unwrap().to_string() == base_ns {
            fix_normed_uri(&norm_base, separator)
        } else {
            fix_normed_uri(
                &quad
                    .g()
                    .unwrap()
                    .iri()
                    .unwrap()
                    .to_string()
                    .replace(base_uri, &norm_base),
                separator,
            )
        };

        let subject = if quad.s().is_blank_node() {
            fix_normed_uri(&quad.s().bnode_id().unwrap(), separator)
        } else if quad.s().iri().unwrap().to_string() == base_ns {
            fix_normed_uri(&norm_base, separator)
        } else {
            fix_normed_uri(
                &quad
                    .s()
                    .iri()
                    .unwrap()
                    .to_string()
                    .replace(base_uri, &norm_base),
                separator,
            )
        };

        let predicate = if quad.p().iri().unwrap().to_string() == base_ns {
            norm_base.to_string()
        } else {
            quad.p()
                .iri()
                .unwrap()
                .to_string()
                .replace(base_uri, &norm_base)
        };

        let object = if quad.o().is_iri() {
            if quad.o().iri().unwrap().to_string() == base_ns {
                fix_normed_uri(&norm_base, separator)
            } else {
                fix_normed_uri(
                    &quad
                        .o()
                        .iri()
                        .unwrap()
                        .to_string()
                        .replace(base_uri, &norm_base),
                    separator,
                )
            }
        } else if quad.o().is_blank_node() {
            // TODO: remove?  Or throw error. This should actually never happen since we replace all bnodes first
            quad.o().bnode_id().unwrap().to_string()
        } else {
            // Double the \\ to bypass rust escaping
            quad.o()
                .lexical_form()
                .unwrap()
                .to_string()
                .replace('\\', "\\\\")
                .replace('\n', "\\n")
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
                // Object => compare_object(&a.object, &b.object),
                // Right now string comes first because starts with ^, but we need the URIs thats starts with "http" to be first
                // Datatype => a.datatype.cmp(&b.datatype),
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

// Compare objects to have ^ and @ ordered after other strings
// fn compare_object(a: &str, b: &str) -> Ordering {
//     println!("ABABAB {} {}", a, b);
//     let starts_special_a = a.starts_with('^') || a.starts_with('@');
//     let starts_special_b = b.starts_with('^') || b.starts_with('@');
//     match (starts_special_a, starts_special_b) {
//         (true, true) | (false, false) => std::cmp::Ord::cmp(&a, &b),
//         (true, false) => Ordering::Greater,
//         (false, true) => Ordering::Less,
//     }
// }

// Compare datatypes to have ^ and @ ordered after other strings
// fn compare_datatype(a: &str, b: &str) -> Ordering {
//     println!("ABABAB compare_datatype {} {}", a, b);
//     let starts_special_a = a.starts_with('^') || a.starts_with('@');
//     let starts_special_b = b.starts_with('^') || b.starts_with('@');
//     match (starts_special_a, starts_special_b) {
//         (true, true) | (false, false) => std::cmp::Ord::cmp(&a, &b),
//         (true, false) => Ordering::Greater,
//         (false, true) => Ordering::Less,
//     }
// }
// left: "RAPMJ82Auq8RcsMQg4OPyTF7LDp532oUGX2n0CNAKgpIA"
// right: "RAr9ao0vjXtLf3d9U4glE_uQWSknfYoPlIzKBq6ybOO5k"
