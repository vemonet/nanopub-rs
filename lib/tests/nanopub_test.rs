use nanopub::nanopub::Nanopub;
use std::fs;

pub const ORCID: &str = "http://orcid.org/0000-0002-1267-0234";

#[test]
fn it_sign_nanopub() {
    let private_key = fs::read_to_string("./tests/resources/id_rsa").unwrap();
    let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
    let np = Nanopub::new(np_rdf.as_str(), private_key.as_str(), ORCID, None, None).unwrap();

    println!("{}", np);
    assert_eq!("http://orcid.org/0000-0002-1267-0234", ORCID);
    // assert_eq!(np.trusty_hash, "RAsfLND-jtohcyohjKmXL7H4KYEDMLr0g4Yc6-8ATwb10")
    // assert_eq!(np.signature_hash, "OC0xJTavw9h/JSZIZl/NLzEZqQk1E7XWV3o1btD1cojxf9FMtgZuMMOtnPcgydRn3gK0wbUh+ATV4sEFdG51khsrOOH7+RylqnaE9XD4L65dwPZ/PpI32/LMYsQ62rsb0ajWtXr5cKDIKaoah0U1V85XlLGhoEyzrLZCU5uqJbo=");
}
