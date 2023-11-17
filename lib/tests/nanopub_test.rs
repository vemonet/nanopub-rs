use nanopub::{Nanopub, NpProfile};
use std::fs;

#[test]
fn publish_nanopub_simple_rsa() {
    let orcid = "http://orcid.org/0000-0002-1267-0234";
    let private_key = fs::read_to_string("./tests/resources/id_rsa").unwrap();
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
    // let np_rdf = fs::read_to_string("./tests/resources/signed.simple1-rsa.trig").unwrap();

    let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();
    let np = Nanopub::publish(&np_rdf, &profile, None).unwrap();

    println!("{}", np);
    assert!(np.published);
    // Values compiled with the nanopub java lib using the exact same RDF
    assert_eq!(
        np.trusty_hash,
        "RAoNJUYtqPuzxfCgi0ZJughw221g1qIhRDGE5EbRTNJ4o"
    );
    assert_eq!(np.signature_hash, "aG7rda/gmsu8hx1fTds9oqvogs4gv8xxkc/SJCtqJjUfgbtH6P3QMafIBdRApFI1WT7qrkYqg3Qs9ugTkOjwq2EJ+IoTJq1lgeo+66th3y2LnSdsI/Lsoa/mE6TIVbjpXvwYAqPGUI4BISISJhAslFFlP54obeBarh2nsiELdf4=");
}

#[test]
fn sign_nanopub_test_blank() {
    let orcid = "http://orcid.org/0000-0000-0000-0000";
    let private_key = fs::read_to_string("./tests/resources/id_rsa").unwrap();
    let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig").unwrap();

    let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();
    let np = Nanopub::sign(&np_rdf, &profile).unwrap();
    println!("{}", np);
    assert!(!np.published);
    // Values compiled with the nanopub java lib using the exact same RDF
    assert_eq!(
        np.trusty_hash,
        "RAoBtLQgkD--9if2Wl_ziui5lZ_-oBrsKyA_4lrMxmFwI"
    );
    assert_eq!(np.signature_hash, "SVG82DiaVebC48kV/o3uOTlI///60YbICvRHEp5kXuuw2HXn4v5S42vcTNiyo75a3DT8dBxty8anDFgVjMEFh9fgzN+yKQekP/P5L3JGHEg+F2kPtR+y7bW3zfBp2erV+V8dsbq8xps36i8sZxVFgKup3R5zUYm43GfDnG4YCpI=");
}

#[test]
fn check_nanopub_test_blank() {
    let np_rdf = fs::read_to_string("./tests/resources/signed.nanopub_test_blank.trig").unwrap();
    Nanopub::check(&np_rdf).unwrap();
}

// #[test]
// fn fetch_nanopub() {
//     let np_url = "http://orcid.org/0000-0000-0000-0000";
//     let np = fetch(np_url);
// }
