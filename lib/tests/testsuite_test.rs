use nanopub::{Nanopub, NpProfile, ProfileBuilder};
use std::{error::Error, fs, path::Path};

fn get_profile() -> NpProfile {
    ProfileBuilder::from_file("tests/resources/profile_no_orcid.yml").unwrap()
}

#[tokio::test]
async fn testsuite_publish_valid_plain() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/valid/plain");
    // Iterate over files
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if filename.ends_with("trig\"") {
            println!("\n☑️  Testing file publish: {}", filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let np = Nanopub::new(&np_rdf)?
                .publish(Some(&get_profile()), None)
                .await?;
            assert!(np.info.published.is_some());
        }
    }
    Ok(())
}

#[test]
fn testsuite_check_valid_signed() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/valid/signed");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if !filename.ends_with("xml\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let _np = Nanopub::new(&np_rdf)?.check().expect("Failed check");
        }
    }
    Ok(())
}

#[test]
fn testsuite_check_valid_trusty() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/valid/trusty");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if !filename.ends_with("xml\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let _np = Nanopub::new(&np_rdf)?.check().expect("Failed check");
        }
    }
    Ok(())
}

#[test]
fn testsuite_check_invalid_signed() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/invalid/signed");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if !filename.ends_with("xml\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let np = Nanopub::new(&np_rdf)?.check();
            assert!(
                np.is_err(),
                "The np check should have failed for file: {}",
                filename
            );
        }
    }
    Ok(())
}

#[test]
fn testsuite_check_invalid_trusty() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/invalid/trusty");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if !filename.ends_with("xml\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let np = Nanopub::new(&np_rdf)?.check();
            assert!(np.is_err(), "The np check should have failed");
        }
    }
    Ok(())
}

#[test]
fn testsuite_check_invalid_plain() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/invalid/plain");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if !filename.ends_with("xml\"") && !filename.contains("valid") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let result = Nanopub::new(&np_rdf).and_then(|np| np.check());
            assert!(
                result.is_err(),
                "The np check should have failed for file: {}",
                filename
            );
        }
    }
    Ok(())
}

#[test]
fn testsuite_publish_invalid_plain() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/invalid/plain");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if !filename.ends_with("xml\"") && !filename.contains("info") {
            println!("\n☑️  [{}] Testing file publish: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let result = Nanopub::new(&np_rdf).and_then(|np| {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(np.publish(Some(&get_profile()), None))
            });
            assert!(
                result.is_err(),
                "The np check should have failed for file: {}",
                filename
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn testsuite_publish_transform_signed_simple1() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/testsuite/transform/signed/rsa-key1/simple1.in.trig")?;
    let np = Nanopub::new(&np_rdf)?
        .publish(Some(&get_profile()), None)
        .await?;
    println!("{}", np.rdf()?);
    println!("{}", get_profile());
    assert!(np.info.published.is_some());
    // assert_eq!(
    //     np.info.trusty_hash,
    //     "RALbDbWVnLmLqpNgOsI_AaYfLbEnlOfZy3CoRRLs9XqVk"
    // );
    // assert_eq!(np.info.signature, "9Z7zk22V1SgJ+jSw4WAkK3yJ7xuoEkIPJWSLEzx0b6OgHiqiioS0DMziQYCjQA8gBWu0zlJr64tj8Ip38fKynxriznwgVtcjBSKtjnLfZEZPZrtasLKxmtrobYbnyNPBi0Geq8oQpeg9Qg5MldhI7HoiEFTaOkmZJEt0TjrOUVc=");
    Ok(())
}

// NOTE: this lib does not support adding a trusty URI to a nanopub without signing it
// so we just check if publishing the given examples signed works
#[tokio::test]
async fn testsuite_publish_transform_trusty_aida() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/testsuite/transform/trusty/aida1.in.trig")?;
    let np = Nanopub::new(&np_rdf)?
        .publish(Some(&get_profile()), None)
        .await?;
    // println!("{}", np);
    assert!(np.info.published.is_some());
    // assert_eq!(np.trusty_hash, "RAPpJU5UOB4pavfWyk7FE3WQiam5yBpmIlviAQWtBSC4M");
    Ok(())
}

#[tokio::test]
async fn testsuite_publish_transform_trusty_simple1() -> Result<(), Box<dyn Error>> {
    let np_rdf = fs::read_to_string("./tests/testsuite/transform/trusty/simple1.in.trig")?;
    let np = Nanopub::new(&np_rdf)?
        .publish(Some(&get_profile()), None)
        .await?;
    assert!(np.info.published.is_some());
    // assert_eq!(np.trusty_hash, "RAtAU6U_xKTH016Eoiu11SswQkBu1elB_3_BoDJWH3arA");
    Ok(())
}
