# 📦 Use the packages

[![crates.io](https://img.shields.io/crates/v/nanopub.svg)](https://crates.io/crates/nanopub)
[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)
[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/nanopub-sign.svg?logo=python&label=Python&logoColor=silver)](https://pypi.org/project/nanopub-sign)

You can easily work with Nanopubs from various languages.

This toolkit will enable you to check, sign, and publish Nanopublications from Rust, Python, and TypeScript/JavaScript.

> The core component of this toolkit is written in Rust, with bindings to other languages.

## 📥️ Installation

Install the package for you language:

=== "Python"

    ```bash
    pip install nanopub-sign
    # or
    uv add nanopub-sign
    ```

=== "JavaScript"

    Install the `npm` package:

    ```bash
    npm install --save @nanopub/sign
    # or
    pnpm add @nanopub/sign
    # or
    yarn add @nanopub/sign
    # or
    bun add @nanopub/sign
    ```

    Or directly import from a CDN in JavaScript code:

    ```typescript
    import init, { Nanopub, NpProfile, getNpServer } from "https://unpkg.com/@nanopub/sign";
    ```

=== "Rust"

    ```bash
    cargo add nanopub
    ```

## ✍️ Sign Nanopubs

This process involves signing a Nanopublication RDF string using a specified RSA private key passed through the profile. The signing operation ensures that the Nanopub is authentically created by the holder of the private key.

!!! success "Get a private key"
    You can easily create and register a new private key on the [playground page](https://vemonet.github.io/nanopub-rs/playground.html) after login with your ORCID.

=== "Python"

    !!! info "Build a Nanopublication"
        This package takes an already prepared Nanopublication RDF string as input. If you want to build a Nanopublication programmatically, use the [`nanopub`](https://fair-workflows.github.io/nanopub) pip package. You can then feed the serialized RDF of the built Nanopub to this package functions.

    ```python title="sign.py"
    from nanopub_sign import Nanopub, NpProfile

    # Change the RDF and private key as you wish
    rdf_str = """@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
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
    }"""
    private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc="

    # Instantiate a nanopub profile (ORCID and name are optional)
    profile = NpProfile(
        private_key=private_key,
        orcid_id="https://orcid.org/0000-0000-0000-0000",
        name="Your Name",
    )

    # Sign a nanopub
    np = Nanopub(rdf_str)
    np = np.sign(profile=profile)
    print("Signed info dict:", np.info())
    ```

    Run the script:

    ```bash
    python sign.py
    ```

=== "JavaScript"

    !!! example "Demo"
        Visit the **[playground page](https://vemonet.github.io/nanopub-rs/playground.html)** to sign nanopubs, or generate and register a new key pair, directly in your browser using this NPM package. You can checkout the [`playground.html`](https://github.com/vemonet/nanopub-rs/blob/main/lib/docs/docs/playground.html) file as an example to use the package directly from HTML/JS.

    !!! info "Build a Nanopublication"
        This package takes an already prepared Nanopublication RDF string as input. If you want to easily display nanopubs checkout the [`@nanopub/display`](https://nanopublication.github.io/nanopub-js/modules/_nanopub_display.html) package, if you want to fetch and manipulate nanopubs check the [`@nanopub/utils`](https://nanopublication.github.io/nanopub-js/modules/ _nanopub_utils.html) package.

    ```typescript title="sign.ts"
    import {Nanopub, NpProfile, getNpServer} from "@nanopub/sign";

    // Change the RDF and private key as you wish
    const rdfStr = `@prefix : <http://purl.org/nanopub/temp/mynanopub#> .
    @prefix np: <http://www.nanopub.org/nschema#> .
    @prefix npx: <http://purl.org/nanopub/x/> .
    @prefix prov: <http://www.w3.org/ns/prov#> .
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
    }`
    const privateKey=`MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=`;

    // Instantiate Nanopub profile (ORCID and name are optional)
    const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "Your Name");

    // Sign the Nanopub RDF
    const signed = new Nanopub(rdfStr).sign(profile);
    console.log("Signed:", signed.info(), signed.rdf());
    ```

    !!! warning "Running in the browser requires initialization"

        When writing code that will be executed in the browser you need to first initialize the Wasm binary:

        ```javascript
        import init, { Nanopub, NpProfile, getNpServer } from "@nanopub/sign";

        async function main() {
            await init();
            const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "Your Name");
            const signed = new Nanopub(rdfStr).sign(profile);
        }
        main();
        ```

=== "Rust"

    ```rust title="sign.rs"
    use nanopub::{Nanopub, ProfileBuilder, NpError};
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
    let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=".to_string();

    let profile = ProfileBuilder::new(private_key)
        .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
        .build()
        .unwrap();

    // Sign
    let signed_np = Nanopub::new(np_rdf).unwrap().sign(&profile).unwrap();

    println!("{}", signed_np)
    ```

    You can also build the nanopub from scratch:

    ```rust
    use nanopub::{Nanopub, ProfileBuilder, NpError, create_base_dataset};
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

        let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=".to_string();

        let profile = ProfileBuilder::new(private_key)
            .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
            .build()?;
        let np = np.sign(Some(&profile))?;
        Ok(())
    }
    ```

## 📬 Publish Nanopubs

Signed Nanopubs can be published to a Nanopub server. This makes the Nanopub accessible to others in the network.

Use the `publish` function on a Nanopub, the 2 arguments are optional:

- 🔑 `profile` is required if you want to also sign the nanopub, it is not required if you provide a signed nanopub
- 🧪 If the `server_url` is null it will be published to the [test server](https://np.test.knowledgepixels.com/)

If the provided `server_url` is empty or null, the nanopub will be published to the [test server](https://np.test.knowledgepixels.com/). In this case the nanopub will not be available at [https://w3id.org/np/](https://w3id.org/np/), but at [https://np.test.knowledgepixels.com/](https://np.test.knowledgepixels.com/), e.g. [https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM](https://np.test.knowledgepixels.com/RAKObyGXmbgTYWj2iN0XGgJv0yWNDQd_DTmAWUouGfIsM)

To publish to a production server use `get_np_server(true)`. With true for a random server in the network, and false for the [main nanopub server](https://server.np.trustyuri.net/), defaults to true.

=== "Python"

    ```python
    from nanopub_sign import Nanopub, NpProfile, get_np_server

    np = Nanopub(rdf_str).publish(
        profile=profile,
        server_url=None,
        # On production servers:
        # server_url=get_np_server(),
    )
    print("Published info dict:", np.info())
    print(np.rdf())
    ```

=== "JavaScript"

    ```typescript
    import {Nanopub, NpProfile, getNpServer} from "@nanopub/sign";

    const profile = new NpProfile(privateKey, "https://orcid.org/0000-0000-0000-0000", "Your Name", "");
    const np = await new Nanopub(rdfStr).publish(profile, null);
    // On production servers:
    // const np = await new Nanopub(rdfStr).publish(profile, getNpServer(true));
    console.log("Published:", np.info(), signed.rdf());
    ```

=== "Rust"

    ```rust
    use nanopub::{Nanopub, ProfileBuilder, get_np_server};
    use tokio::runtime;

    // Instantiate nanopub profile
    let private_key = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc=".to_string();

    let profile = ProfileBuilder::new(private_key)
        .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
        .build()
        .unwrap();

    // Publish is async
    let rt = runtime::Runtime::new().expect("Runtime failed");

    let published_np = rt.block_on(async {
        Nanopub::new(np_rdf).unwrap().publish(Some(&profile), None).await.unwrap()
        // Or provide a server to publish to production:
        // Nanopub::new(np_rdf).unwrap().publish(Some(&profile), get_np_server(true)).await.unwrap()
    });
    println!("{}", published_np)
    ```

!!! tip "Provide the nanopub signed or unsigned"
    - [x] If signed nanopub and profile not provided, we publish the signed nanopub as it is
    - [x] If signed nanopub and profile provided, we re-sign the nanopub (signature triples are updated)
    - [x] If unsigned nanopub and profile provided, we sign the nanopub
    - [ ] If unsigned nanopub and profile not provided, we throw an error

## ✅ Check Nanopubs

This operation involves checking the integrity of Nanopubs. It ensures that a Nanopub is valid, regardless of whether it is signed or unsigned.

=== "Python"

    ```python
    from nanopub_sign import Nanopub

    np = Nanopub(rdf_str).check()
    print("Checked info dict:", np.info())
    ```

=== "JavaScript"

    ```typescript
    import {Nanopub} from "@nanopub/sign";

    const checked = new Nanopub(rdfStr).check();
    ```

=== "Rust"

    ```rust
    use nanopub::Nanopub;

    let checked_np = Nanopub::new(&signed_np.rdf().unwrap()).unwrap().check();
    ```

## 📡 Fetch Nanopubs

This function allows you to retrieve Nanopubs from the network using their URI. It's useful for accessing and using Nanopubs created by others.

=== "Python"

    ```python
    from nanopub_sign import Nanopub

    np = Nanopub.fetch("https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw")
    print(np.info())
    ```

=== "JavaScript"

    ```typescript
    const npUri = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
    const np = await Nanopub.fetch(npUri);
    console.log(np.info())
    ```

=== "Rust"

    ```rust
    use nanopub::Nanopub;
    use tokio::runtime;

    let uri = "https://w3id.org/np/RAltRkGOtHoj5LcBJZ62AMVOAVc0hnxt45LMaCXgxJ4fw";
    let rt = runtime::Runtime::new().expect("Runtime failed");

    let np = rt.block_on(async {
        Nanopub::fetch(&uri).await
    }).unwrap();
    ```

## 🔑 Generate private key and publish introduction

You can generate a new private/public key pair, and publish a nanopub introduction to register this key under your ORCID in the Nanopublications network:

=== "Python"

    ```python
    from nanopub_sign import Nanopub, NpProfile, KeyPair, get_np_server

    # Randomly generate a new private/public key pair
    keypair = KeyPair()

    # Create a profile with this new private key
    new_profile = NpProfile(
        private_key=keypair.private,
        orcid_id="https://orcid.org/0000-0000-0000-0000",
        name="Your Name",
    )

    # Publish a nanopub introduction for this profile
    np = Nanopub.publish_intro(new_profile, get_np_server())
    print(np.info())

    # Publish to the production network:
    # np = Nanopub.publish_intro(new_profile, get_np_server())
    ```

=== "JavaScript"

    ```typescript
    import init, {Nanopub, NpProfile, KeyPair} from "@nanopub/sign";

    // Randomly generate a new private/public key pair, and convert it to a JS object
    let keypair = new KeyPair();
    keypair = keypair.toJs();

    // Create a profile with this new private key
    const orcid = "https://orcid.org/0000-0000-0000-0000"
    const profile = new NpProfile(keypair.private, orcid, "Your Name");

    // Publish a nanopub introduction for this profile
    Nanopub.publish_intro(profile, getNpServer(false))
        .then(np => {
            console.log("Published Introduction Nanopub:", np.info());
        })
        .catch(err => {
            console.error("Error publishing the Nanopub Introduction:", err);
        });
    ```

=== "Rust"

    ```rust
    use nanopub::{profile::gen_keys, Nanopub, ProfileBuilder};
    use tokio::runtime;

    // Randomly generate a new private/public key pair
    let (private_key, _pubkey) = gen_keys().unwrap();

    // Create a profile with this new private key
    let profile = ProfileBuilder::new(private_key)
        .with_orcid("https://orcid.org/0000-0000-0000-0000".to_string())
        .with_name("Your Name".to_string())
        .build()
        .unwrap();

    // Publish a nanopub introduction for this profile
    let rt = runtime::Runtime::new().expect("Runtime failed");
    let np = rt.block_on(async {
        Nanopub::new_intro(&profile).unwrap()
            .publish(Some(&profile), None).await.unwrap();
    });
    ```

!!! warning "Key format"
    The key needs to be in format `PKCS8`, the default OpenSSH format is not supported. Alternatively you can generate a key using the following command:

    ```bash
    ssh-keygen -t rsa -m PKCS8 -b 4096 -f ~/.nanopub/id_rsa -C "your@email.com"
    ```
