use nanopub::{get_np_server, profile::get_default_profile_path, Nanopub, NpProfile};
use std::{error::Error, fs};

fn get_test_key() -> String {
    fs::read_to_string("./tests/resources/id_rsa").unwrap()
}

#[test]
fn publish_nanopub_simple_rsa() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, None)?;
    // println!("{}", np);
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
    let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;

    let profile = NpProfile::new(
        "https://orcid.org/0000-0000-0000-0000",
        "",
        &get_test_key(),
        None,
    )?;
    println!("{}", profile); // required for coverage

    assert!(get_default_profile_path().ends_with(".nanopub/profile.yml"));
    let np = Nanopub::sign(&np_rdf, &profile)?;
    println!("{}", np.info); // required for coverage
    assert!(!np.published);
    Ok(())
}

#[test]
fn publish_fail() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, Some("failing"))?;
    assert!(!np.published);
    Ok(())
}

#[test]
fn profile_fail() -> Result<(), Box<dyn Error>> {
    let profile = NpProfile::new("", "", "failing", None);
    assert!(profile.is_err());
    Ok(())
}

#[test]
fn check_nanopub_test_blank() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/signed.nanopub_test_blank.trig")?;
    Nanopub::check(&np_rdf)?;
    Ok(())
}

#[test]
fn test_get_np_server() -> Result<(), Box<dyn Error>> {
    let np_server = get_np_server(true);
    println!("{}", np_server);
    let np_server = get_np_server(false);
    assert_eq!(np_server, "http://server.nanopubs.lod.labs.vu.nl/");
    Ok(())
}

#[test]
fn publish_jsonld() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/nanopub.jsonld")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, None)?;
    println!("{}", np);
    assert!(np.published);
    Ok(())
}

// #[test]
// fn fetch_nanopub() {
//     let np_url = "http://orcid.org/0000-0000-0000-0000";
//     let np = fetch(np_url);
// }
