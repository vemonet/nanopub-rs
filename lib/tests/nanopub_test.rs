use nanopub::{
    constants::TEST_SERVER,
    extract::extract_np_info,
    get_np_server,
    profile::gen_keys,
    publish::publish_np,
    sign::normalize_dataset,
    utils::{ns, parse_rdf},
    Nanopub, NpProfile,
};
use sophia::inmem::dataset::LightDataset;
use std::{error::Error, fs};

fn get_test_key() -> String {
    fs::read_to_string("./tests/resources/id_rsa").unwrap()
}

#[tokio::test]
async fn publish_nanopub_simple_rsa() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, None).await?;
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

#[tokio::test]
async fn publish_proteinatlas() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("tests/testsuite/valid/plain/proteinatlas-16-1.trig")?;
    // let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, None).await?;
    assert!(np.published);
    println!("{}", np);
    Ok(())
}

#[test]
fn sign_nanopub_blank() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;

    let profile = NpProfile::new(
        "https://orcid.org/0000-0000-0000-0000",
        "",
        &get_test_key(),
        None,
    )?;
    println!("{}", profile); // cov
    let _pubkey = profile.get_public_key(); // cov
    let np = Nanopub::sign(&np_rdf, &profile)?;
    assert!(!np.published);
    Ok(())
}

#[test]
fn check_valid_unsigned() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let np = Nanopub::check(&np_rdf);
    assert!(np.is_ok());
    Ok(())
}

#[test]
fn wrong_rdf_file() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./inexistent");
    assert!(np_rdf.is_err());
    let np_rdf = fs::read_to_string("./tests/resources/wrong-rdf.trig")?;
    let np = Nanopub::check(&np_rdf);
    assert!(np.is_err());
    let np = Nanopub::check("{wrong");
    assert!(np.is_err());
    Ok(())
}

#[tokio::test]
async fn publish_fail() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, Some("failing")).await;
    assert!(np.is_err());
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
    assert_eq!(np_server, "https://server.nanopubs.lod.labs.vu.nl/");
    Ok(())
}

#[tokio::test]
async fn publish_jsonld() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/nanopub.jsonld")?;
    let profile = NpProfile::new("", "", &get_test_key(), None)?;
    let np = Nanopub::publish(&np_rdf, &profile, None).await?;
    println!("{}", np);
    assert!(np.published);
    Ok(())
}

#[tokio::test]
async fn publish_np_intro() -> Result<(), Box<dyn Error>> {
    let profile = NpProfile::new(
        "https://orcid.org/0000-0000-0000-0000",
        "Test User",
        &get_test_key(),
        None,
    )?;
    let np = Nanopub::publish_intro(&profile, None).await?;
    println!("{}", np);
    assert!(np.published);
    Ok(())
}

#[test]
fn test_np_info() -> Result<(), Box<dyn Error>> {
    let rdf_str = fs::read_to_string("./tests/resources/nanopub.jsonld")?;
    let dataset = parse_rdf(&rdf_str)?;
    let np_info = extract_np_info(&dataset, true)?;
    println!("{}", np_info); // Required for coverage
    Ok(())
}

#[test]
fn default_profile_file() -> Result<(), Box<dyn Error>> {
    let _profile = NpProfile::from_file("");
    Ok(())
}

#[test]
fn test_normalize() -> Result<(), Box<dyn Error>> {
    let dataset = LightDataset::new();
    let _norm = normalize_dataset(&dataset, "", "", "#");
    Ok(())
}

#[test]
fn test_get_ns_empty() -> Result<(), Box<dyn Error>> {
    let ns = std::panic::catch_unwind(|| {
        ns("not there");
    });
    // ns.is_err()
    match ns {
        Ok(_) => panic!("No panic occurred"),
        Err(_) => Ok(()),
    }
}

#[tokio::test]
async fn fetch_nanopub() -> Result<(), Box<dyn Error>> {
    let np_url = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
    let np = Nanopub::fetch(np_url).await?;
    assert!(np.published);
    println!("{}", np);
    Ok(())
}

#[test]
fn test_gen_keys() -> Result<(), Box<dyn Error>> {
    let (privkey, _pubkey) = gen_keys()?;
    println!("{}", privkey);
    assert!(privkey.len() > 10);
    Ok(())
}

#[tokio::test]
async fn unit_publish_np_fail() -> Result<(), Box<dyn Error>> {
    let res = publish_np(TEST_SERVER, "wrong").await;
    assert!(res.is_err());
    Ok(())
}
