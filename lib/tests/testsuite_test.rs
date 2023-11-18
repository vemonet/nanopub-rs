use nanopub::{Nanopub, NpProfile};
use std::{error::Error, fs, path::Path};

const ORCID: &str = "http://orcid.org/0000-0000-0000-0000";
fn get_profile() -> NpProfile {
    NpProfile::new(
        ORCID,
        "",
        &fs::read_to_string("./tests/resources/id_rsa").unwrap(),
        None,
    )
    .unwrap()
}

#[test]
fn testsuite_publish_valid_plain() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/valid/plain");
    // Iterate over files
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if filename.ends_with("trig\"") {
            println!("\n☑️  Testing file publish: {}", filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let np = Nanopub::publish(&np_rdf, &get_profile(), None)?;
            assert!(np.published);
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
        if filename.ends_with("trig\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let _np = Nanopub::check(&np_rdf).expect("Failed check");
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
        if filename.ends_with("trig\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let _np = Nanopub::check(&np_rdf).expect("Failed check");
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
        if filename.ends_with("trig\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let result = std::panic::catch_unwind(|| {
                Nanopub::check(&np_rdf).expect("Failed check");
            });
            assert!(result.is_err(), "The np check did not failed");
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
        if filename.ends_with("trig\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let result = std::panic::catch_unwind(|| {
                Nanopub::check(&np_rdf).expect("Failed check");
            });
            assert!(result.is_err(), "The np check did not failed");
        }
    }
    Ok(())
}

// TODO: define a proper error handling system (use NpError everywhere instead of panics?)
// Improve np_info_extract to fail when criteria not met (e.g. empty assertion graph)

#[test]
fn testsuite_publish_invalid_plain() -> Result<(), Box<dyn Error>> {
    let path = Path::new("tests/testsuite/invalid/plain");
    // Iterate over files
    for (index, entry) in fs::read_dir(path)?.enumerate() {
        let file = entry?;
        let filename = format!("{:?}", file.file_name());
        if filename.ends_with("trig\"") && !filename.contains("simple1-signed-dsa") {
            println!("\n☑️  [{}] Testing file check: {}", index, filename);
            let np_rdf = fs::read_to_string(file.path())?;
            let np = Nanopub::publish(&np_rdf, &get_profile(), None);
            assert!(np.is_err(), "The np check did not failed");
        }
    }
    Ok(())
}
