use nanopub::{Nanopub, NpProfile};
use std::{error::Error, fs};

fn get_test_key() -> String {
    fs::read_to_string("./tests/resources/id_rsa").unwrap()
}

#[test]
fn publish_nanopub_simple_rsa() -> Result<(), Box<dyn Error>> {
    let orcid = "http://orcid.org/0000-0002-1267-0234";
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;

    let profile = NpProfile::new(orcid, "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, None)?;

    println!("{}", np);
    assert!(np.published);
    // Values compiled with the nanopub java lib using the exact same RDF
    assert_eq!(
        np.trusty_hash,
        "RAoNJUYtqPuzxfCgi0ZJughw221g1qIhRDGE5EbRTNJ4o"
    );
    assert_eq!(np.signature_hash, "aG7rda/gmsu8hx1fTds9oqvogs4gv8xxkc/SJCtqJjUfgbtH6P3QMafIBdRApFI1WT7qrkYqg3Qs9ugTkOjwq2EJ+IoTJq1lgeo+66th3y2LnSdsI/Lsoa/mE6TIVbjpXvwYAqPGUI4BISISJhAslFFlP54obeBarh2nsiELdf4=");
    Ok(())
}

#[test]
fn sign_nanopub_test_blank() -> Result<(), Box<dyn Error>> {
    let orcid = "http://orcid.org/0000-0000-0000-0000";
    let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;

    let profile = NpProfile::new(orcid, "", &get_test_key(), None)?;
    let np = Nanopub::sign(&np_rdf, &profile)?;
    println!("{}", np);
    assert!(!np.published);
    // Values compiled with the nanopub java lib using the exact same RDF
    assert_eq!(
        np.trusty_hash,
        "RAoBtLQgkD--9if2Wl_ziui5lZ_-oBrsKyA_4lrMxmFwI"
    );
    assert_eq!(np.signature_hash, "SVG82DiaVebC48kV/o3uOTlI///60YbICvRHEp5kXuuw2HXn4v5S42vcTNiyo75a3DT8dBxty8anDFgVjMEFh9fgzN+yKQekP/P5L3JGHEg+F2kPtR+y7bW3zfBp2erV+V8dsbq8xps36i8sZxVFgKup3R5zUYm43GfDnG4YCpI=");
    Ok(())
}

#[test]
fn check_nanopub_test_blank() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/signed.nanopub_test_blank.trig")?;
    Nanopub::check(&np_rdf)?;
    Ok(())
}

// #[test]
// fn fetch_nanopub() {
//     let np_url = "http://orcid.org/0000-0000-0000-0000";
//     let np = fetch(np_url);
// }
