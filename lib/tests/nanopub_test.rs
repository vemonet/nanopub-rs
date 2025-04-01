use nanopub::{
    constants::TEST_SERVER,
    extract::extract_np_info,
    get_np_server,
    nanopub::create_base_dataset,
    network::publish_np,
    profile::gen_keys,
    sign::normalize_dataset,
    utils::{ns, parse_rdf},
    Nanopub, ProfileBuilder,
};
use sophia::{api::dataset::MutableDataset, inmem::dataset::LightDataset, iri::Iri};
use std::{error::Error, fs};

fn get_test_key() -> String {
    fs::read_to_string("./tests/resources/id_rsa").unwrap()
}

#[tokio::test]
async fn publish_nanopub_simple_rsa() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let profile = ProfileBuilder::new(get_test_key()).build()?;
    let np = Nanopub::new(&np_rdf)?.publish(Some(&profile), None).await?;

    // NOTE: in case the test server suddenly starts to throw error, you can easily validate that the generated nanopubs
    // are valid, and that the problem is on the test server that is not working.
    // let np = Nanopub::new(&np_rdf)?.publish(Some(&profile), Some(get_np_server(true))).await?;
    // println!("{}", np.rdf()?);

    assert!(np.info.published.is_some());
    // Values compiled with the nanopub java lib using the exact same RDF
    assert_eq!(
        np.info.trusty_hash,
        "RAoNJUYtqPuzxfCgi0ZJughw221g1qIhRDGE5EbRTNJ4o"
    );
    assert_eq!(np.info.signature, "aG7rda/gmsu8hx1fTds9oqvogs4gv8xxkc/SJCtqJjUfgbtH6P3QMafIBdRApFI1WT7qrkYqg3Qs9ugTkOjwq2EJ+IoTJq1lgeo+66th3y2LnSdsI/Lsoa/mE6TIVbjpXvwYAqPGUI4BISISJhAslFFlP54obeBarh2nsiELdf4=");
    Ok(())
}

#[tokio::test]
async fn publish_proteinatlas() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("tests/testsuite/valid/plain/proteinatlas-16-1.trig")?;
    // let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;
    let profile = ProfileBuilder::new(get_test_key()).build()?;
    let np = Nanopub::new(&np_rdf)?
        .publish(Some(&profile), Some(""))
        // .publish(Some(&profile), Some(get_np_server(false))) // in case we need to test directly on prod servers
        .await?;
    assert!(np.info.published.is_some());
    Ok(())
}

#[tokio::test]
async fn publish_already_signed_no_profile() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/signed.simple1-rsa.trig")?;
    let np = Nanopub::new(&np_rdf)?.publish(None, None).await?;
    assert!(np.info.published.is_some());
    Ok(())
}

#[tokio::test]
async fn publish_already_signed_with_profile() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/signed.simple1-rsa.trig")?;
    let profile = ProfileBuilder::new(get_test_key()).build()?;
    let np = Nanopub::new(&np_rdf)?.publish(Some(&profile), None).await?;
    // println!("{}", np.info);
    // println!("{}", np.rdf()?);
    assert!(np.info.published.is_some());
    Ok(())
}

#[tokio::test]
async fn publish_unsigned_no_profile_error() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let np = Nanopub::new(&np_rdf)?.publish(None, None).await;
    assert!(np.is_err());
    Ok(())
}

#[test]
fn sign_nanopub_blank() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;

    let profile = ProfileBuilder::new(get_test_key())
        .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
        .with_name("Test User".to_string())
        .with_intro_nanopub(
            "https://w3id.org/np/RAqz9iOB9hMkNmH9m6e2FvbZRrLgb9aMBfKSklMVh9LrM".to_string(),
        )
        .build()?;
    println!("{}", profile); // to cover fmt in code cov
    let _pubkey = profile.get_public_key(); // cov
    let np = Nanopub::new(&np_rdf)?.sign(&profile)?;
    assert!(np.info.published.is_none());
    Ok(())
}

#[test]
fn load_profile_yml() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/nanopub_test_blank.trig")?;

    let profile = ProfileBuilder::from_file("tests/resources/profile.yml").unwrap();
    let np = Nanopub::new(&np_rdf)?.sign(&profile)?;
    assert!(!np.info.signature.is_empty());

    let profile = ProfileBuilder::from_file("tests/resources/profile_no_orcid.yml").unwrap();
    let np = Nanopub::new(&np_rdf)?.sign(&profile)?;
    assert!(!np.info.signature.is_empty());
    Ok(())
}

#[test]
fn check_valid_unsigned() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let np = Nanopub::new(&np_rdf)?.check();
    assert!(np.is_ok());
    Ok(())
}

#[test]
fn wrong_rdf_file() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./inexistent");
    assert!(np_rdf.is_err());
    let np_rdf = fs::read_to_string("./tests/resources/wrong-rdf.trig")?;
    let np = Nanopub::new(&np_rdf);
    assert!(np.is_err());
    let np = Nanopub::new("{wrong");
    assert!(np.is_err());
    Ok(())
}

#[tokio::test]
async fn publish_fail() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig")?;
    let profile = ProfileBuilder::new(get_test_key()).build()?;
    let np = Nanopub::new(&np_rdf)?
        .publish(Some(&profile), Some("failing"))
        .await;
    assert!(np.is_err());
    Ok(())
}

#[test]
fn profile_fail() -> Result<(), Box<dyn Error>> {
    let profile = ProfileBuilder::new("failing".to_string()).build();
    assert!(profile.is_err());
    Ok(())
}

#[test]
fn check_nanopub_test_blank() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/signed.nanopub_test_blank.trig")?;
    Nanopub::new(&np_rdf)?.check()?;
    Ok(())
}

#[test]
fn test_get_np_server() -> Result<(), Box<dyn Error>> {
    let _np_server = get_np_server(true);
    let np_server = get_np_server(false);
    assert_eq!(np_server, "https://server.np.trustyuri.net/");
    Ok(())
}

#[tokio::test]
async fn publish_jsonld() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/resources/nanopub.jsonld")?;
    let profile = ProfileBuilder::new(get_test_key()).build()?;
    let np = Nanopub::new(&np_rdf)?.publish(Some(&profile), None).await?;
    assert!(np.info.published.is_some());
    Ok(())
}

#[tokio::test]
async fn publish_np_intro() -> Result<(), Box<dyn Error>> {
    let profile = ProfileBuilder::new(get_test_key())
        .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
        .with_name("Test User".to_string())
        .build()?;
    let np = Nanopub::new_intro(&profile)?
        .publish(Some(&profile), None)
        .await?;
    // println!("{}", np);
    assert!(np.info.published.is_some());
    // Test error when Profile not complete
    let profile = ProfileBuilder::new(get_test_key())
        .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
        .build()?;
    assert!(Nanopub::new_intro(&profile).is_err());
    let profile = ProfileBuilder::new(get_test_key())
        .with_name("Test User".to_string())
        .build()?;
    assert!(Nanopub::new_intro(&profile).is_err());
    Ok(())
}

#[test]
fn test_np_info() -> Result<(), Box<dyn Error>> {
    let rdf_str = fs::read_to_string("./tests/resources/nanopub.jsonld")?;
    let dataset = parse_rdf(&rdf_str)?;
    let np_info = extract_np_info(&dataset)?;
    println!("{}", np_info); // Required for coverage
    Ok(())
}

#[test]
fn default_profile_file() -> Result<(), Box<dyn Error>> {
    let _profile = ProfileBuilder::from_file("");
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
    assert!(np.info.published.is_some());
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

#[tokio::test]
async fn publish_from_scratch() -> Result<(), Box<dyn Error>> {
    let mut np = Nanopub::new(create_base_dataset()?)?;
    println!("DEBUG: SCRATCH {}", np.rdf()?);
    let profile = ProfileBuilder::new(get_test_key()).build()?;
    np.dataset.insert(
        Iri::new_unchecked("http://example.org/mosquitoes"),
        Iri::new_unchecked("http://example.org/transmits"),
        Iri::new_unchecked("http://example.org/malaria"),
        Some(&np.info.assertion),
    )?;
    np.dataset.insert(
        &np.info.assertion,
        Iri::new_unchecked("http://www.w3.org/ns/prov#hadPrimarySource"),
        Iri::new_unchecked("http://dx.doi.org/10.3233/ISU-2010-0613"),
        Some(&np.info.prov),
    )?;
    let np = np.publish(Some(&profile), None).await?;
    println!("DEBUG: SCRATCH 2 {}", np.rdf()?);
    // assert!(res.is_err());
    Ok(())
}
