use sophia::iri::Iri;
use sophia::prefix::Prefix;
// use sophia::ns::Namespace;

// pub const NPX: Namespace<&str> = Namespace::new("http://purl.org/nanopub/x/")?;

pub const NPX: &str = "http://purl.org/nanopub/x/";

// fn normalize_key(key: &str) -> Result<String, Box<dyn Error>> {
pub fn get_prefixes() -> [(Prefix<'static>, Iri<'static>); 10] {
    [
        (
            Prefix::new_unchecked("rdf"),
            Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ),
        (
            Prefix::new_unchecked("rdfs"),
            Iri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#"),
        ),
        (
            Prefix::new_unchecked("xsd"),
            Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#"),
        ),
        (
            Prefix::new_unchecked("schema"),
            Iri::new_unchecked("http://schema.org/"),
        ),
        (
            Prefix::new_unchecked("foaf"),
            Iri::new_unchecked("http://xmlns.com/foaf/0.1/"),
        ),
        (
            Prefix::new_unchecked("biolink"),
            Iri::new_unchecked("https://w3id.org/biolink/vocab/"),
        ),
        (
            Prefix::new_unchecked("np"),
            Iri::new_unchecked("http://www.nanopub.org/nschema#"),
        ),
        (
            Prefix::new_unchecked("prov"),
            Iri::new_unchecked("http://www.w3.org/ns/prov#"),
        ),
        (
            Prefix::new_unchecked("npx"),
            Iri::new_unchecked("http://purl.org/nanopub/x/"),
        ),
        (
            Prefix::new_unchecked("nptemp"),
            Iri::new_unchecked("http://purl.org/nanopub/temp/mynanopub#"),
        ),
    ]
}
