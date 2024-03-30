# 🦀 Use from Rust

[![crates.io](https://img.shields.io/crates/v/nanopub.svg)](https://crates.io/crates/nanopub)

The core component of this toolkit is written in Rust.

You can use the Rust crate to easily sign, publish, or check a Nanopub.

## 🧩 Publish from existing RDF

Check, sign, or publish a nanopub RDF string:

```rust
use nanopub::{Nanopub, NpProfile, NpError};
use tokio::runtime;

let np_rdf = r#"@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix dc: <http://purl.org/dc/terms/> .
@prefix pav: <http://purl.org/pav/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix np: <http://www.nanopub.org/nschema#> .
@prefix npx: <http://purl.org/nanopub/x/> .
@prefix ex: <http://example.org/> .
:Head {
	: np:hasAssertion :assertion ;
		np:hasProvenance :provenance ;
		np:hasPublicationInfo :pubinfo ;
		a np:Nanopublication .
}
:assertion {
	ex:mosquito ex:transmits ex:malaria .
}
:provenance {
	:assertion prov:hadPrimarySource <http://dx.doi.org/10.3233/ISU-2010-0613> .
}
:pubinfo {
	: a npx:ExampleNanopub .
}"#;

// Instantiate nanopub profile
let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
let profile = NpProfile::new(&private_key, "https://orcid.org/0000-0000-0000-0000", "", None).unwrap();

// Sign
let signed_np = Nanopub::new(np_rdf).unwrap().sign(&profile).unwrap();

// Check
let checked_np = Nanopub::new(&signed_np.rdf().unwrap()).unwrap().check();

// Publish is async
let rt = runtime::Runtime::new().expect("Runtime failed");

let published_np = rt.block_on(async {
    Nanopub::new(np_rdf).unwrap().publish(Some(&profile), None).await.unwrap()
    // Or provide a server to publish to production:
    // Nanopub::new(np_rdf).unwrap().publish(Some(&profile), get_np_server(true)).await.unwrap()
});
println!("{}", published_np)
```

The `publish` function takes 2 optional arguments:

- 🔑 `profile` is required if you want to also sign the nanopub, it is not required if you provide a signed nanopub
- 🧫 If the `server_url` is none it will be published to the test server

> Provide the nanopub signed or unsigned:
>
> - [x] If signed nanopub and profile not provided, we publish the signed nanopub as it is
> - [x] If signed nanopub and profile provided, we re-sign the nanopub (signature triples are updated)
> - [x] If unsigned nanopub and profile provided, we sign the nanopub
> - [ ] If unsigned nanopub and profile not provided, we throw an error

#### 🧪 Test and productions servers

If the the last argument of `publish()` is none the nanopub will be published to the [test server](https://np.test.knowledgepixels.com/). In this case the nanopub will not be available at [https://w3id.org/np/](https://w3id.org/np/), but at [https://np.test.knowledgepixels.com/](https://np.test.knowledgepixels.com/), e.g. [https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM](https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM)

You can publish to the production network by getting the URL of a server using `get_np_server(true)` (true will pick a random nanopub server on the production network, while false will pick the [main nanopub server](https://server.np.trustyuri.net/)).

## 🚀 Publish from scratch

You can also build the nanopub from scratch, and publish it:

```rust
use nanopub::{Nanopub, NpProfile, NpError, create_base_dataset};
use sophia::iri::Iri;

#[tokio::test]
async fn np_from_scratch() -> Result<(), NpError> {
    // Create blank nanopub
    let mut np = Nanopub::new(create_base_dataset()?)?;
    // Add triples to the assertion graph
    np.dataset.insert(
        Iri::new_unchecked("http://example.org/mosquitoes"),
        Iri::new_unchecked("http://example.org/transmits"),
        Iri::new_unchecked("http://example.org/malaria"),
        Some(&np.info.assertion),
    )?;
    // Add triples to the provenance graph
    np.dataset.insert(
        &np.info.assertion,
        Iri::new_unchecked("http://www.w3.org/ns/prov#hadPrimarySource"),
        Iri::new_unchecked("http://dx.doi.org/10.3233/ISU-2010-0613"),
        Some(&np.info.prov),
    )?;

    let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=";
    let profile = NpProfile::new(&private_key, "https://orcid.org/0000-0000-0000-0000", "", None)?;
    let np = np.publish(Some(&profile), None).await?;
    Ok(())
}
```

## 📡 Fetch Nanopubs

The `fetch` static function on the `Nanopub` struct allows you to retrieve Nanopubs from the network using their URI. It's useful for accessing and using Nanopubs created by others.

```rust
use nanopub::{Nanopub, NpProfile, NpError};
use tokio::runtime;

let uri = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
let rt = runtime::Runtime::new().expect("Runtime failed");

let np = rt.block_on(async {
    Nanopub::fetch(&uri).await
}).unwrap();
```


### 🔑 Generate private key and publish introduction

You can generate a new private/public key pair, and publish a nanopub introduction to register this key under your ORCID in the Nanopublications network:

```rust
use nanopub::{profile::gen_keys, Nanopub, NpProfile};
use tokio::runtime;

// Randomly generate a new private/public key pair
let (private_key, _pubkey) = gen_keys().unwrap();

// Create a profile with this new private key
let profile = NpProfile::new(&private_key, "https://orcid.org/0000-0000-0000-0000", "Your Name", None).unwrap();

// Publish a nanopub introduction for this profile
let rt = runtime::Runtime::new().expect("Runtime failed");
let np = rt.block_on(async {
    Nanopub::new_intro(&profile).unwrap()
        .publish(Some(&profile), None).await.unwrap();
});
```

## 📖 API reference

Checkout the **[API documentation](https://docs.rs/nanopub)** for more details on how to use the different components and functions of the rust crate.
