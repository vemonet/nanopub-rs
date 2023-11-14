# 🔬🦀 Nanopub rs

[![Lint and Test](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml) [![Build](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml) [![Deploy docs to GitHub Pages](https://github.com/vemonet/nanopub-rs/actions/workflows/docs.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/docs.yml)

A rust toolkit to sign and publish [Nanopublications](https://nanopub.net), with bindings to python and javascript (wasm).

## Nanopub signing process

- preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear (normalized URI: `http://purl.org/np/ARTIFACTCODE-PLACEHOLDER/`, cf. [code](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/trusty/TempUriReplacer.java#L12)); this includes the signature part, except the triple that is stating the actual signature
- preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
- Signature is calculated on this normalized representation (cf. most of the process in the [trusty-uri python lib](https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15), see also [SignatureUtils](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/extra/security/SignatureUtils.java#L196) and [trusty-uri](https://github.com/trustyuri/trustyuri-java/blob/08b61fbb13d20a5cbefde617bd9a9e9b0b03d780/src/main/java/net/trustyuri/rdf/RdfHasher.java#L86))
- Signature triple is added
- Trusty URI code is calculated on normalized representation that includes signature
- Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub

## Run the library to test

```bash
cd try/
cargo build
cargo run
```

> Checkout the README in the `python` and `js` folder for the instructions to build and test for each language

## Development

Install development dependencies:
```bash
rustup update
rustup component add rustfmt clippy
```

### Build and run all packages

All packages at once:

```bash
cargo build --all
cargo run --all-features
```

Just the rust package:

```bash
cargo run try
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy --all --all-targets --all-features
```

### Run tests

Display prints:

```bash
cargo test -- --nocapture
```

Run all tests:

```bash
cargo test --verbose --all --all-features
```

### Generate docs

```bash
./scripts/docs-serve.sh
```

## Useful links

https://github.com/briansmith/ring

https://github.com/Vanethos/rust-wasm-example-rsa

https://github.com/Harzu/wasm-rsa

https://github.com/frehberg/wasm-sign

Building artefacts for many targets: https://github.com/oxigraph/oxigraph/blob/main/.github/workflows/build.yml
