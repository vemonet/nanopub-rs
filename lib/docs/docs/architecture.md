# 🗺️ Architecture details

This page presents the project architecture and some technical details.

## 🏔️ Changelog

Version history is recorded in the [CHANGELOG.md](https://github.com/vemonet/nanopub-rs/blob/main/CHANGELOG.md).

## 🗃️ Folder structure

```
nanopub-rs/
├── lib/
│   ├── src/
│   │   └── 🦀 Source code for the core Rust crate.
│   ├── tests/
│   │   └── 🧪 Tests for the core Rust crate.
│   └── docs/
│       └── 📖 Markdown and HTML files for the documentation website.
├── python/
│   └── 🐍 Python bindings for interacting with the Rust crate.
├── js/
│   └── 🌐 JavaScript bindings for integrating into JS environments.
├── cli/
│   └── ⌨️ Scripts for the command-line interface.
├── scripts/
│   └── 🛠️ Development scripts (build docs, testing).
└── .github/
    └── workflows/
        └── ⚙️ Automated CI/CD workflows.
```

## ✒️ Nanopub signing process

1. Preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear (normalized URI: `https://w3id.org/np/ `, cf. [original java implementation](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/trusty/TempUriReplacer.java#L12)); this includes the signature part, except the triple that is stating the actual signature
2. Preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
3. Signature is calculated on this normalized representation (cf. most of the process in the [trusty-uri python lib](https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15), see also [SignatureUtils](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/extra/security/SignatureUtils.java#L196) and [trusty-uri](https://github.com/trustyuri/trustyuri-java/blob/08b61fbb13d20a5cbefde617bd9a9e9b0b03d780/src/main/java/net/trustyuri/rdf/RdfHasher.java#L86))
4. Signature triple is added
5. Trusty URI code is calculated on normalized representation that includes signature
6. Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub

## 🛠️ Notes about maintenance and stability

Cross-compiling to many targets brings some complexity to the build process, especially that the nanopub lib packs a lot of features: processing RDF, RSA signing and random key generation, querying a HTTP server, getting current datetime.

This means we need to make sure the dependencies we use work for all compilation targets (e.g. aarch64, wasm). And in some case we need to define platform dependant dependencies in the `Cargo.toml` (e.g. reqwest `native-tls` for aarch64 windows instead of the `rustls-tls`)

Packages are built for different targets in the `.github/workflows/build.yml` GitHub action.

## ☑️ To do

- [ ] Add possibility to build the nanopub from scratch for JS and python
- [ ] Integrate to the python `nanopub` library to perform signing?
- [ ] Add Ruby bindings? https://docs.rs/magnus/latest/magnus https://github.com/ankane/tokenizers-ruby
- [ ] Add Java bindings? https://docs.rs/jni/latest/jni
- [ ] Add brew packaging? (cf. [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/pkg/brew/ripgrep-bin.rb))

## ⏱️ Speed comparison

Speed taken when signing a nanopub using different languages implementations (in this order: [java](https://github.com/Nanopublication/nanopub-java), [python](https://github.com/fair-workflows/nanopub), rust):

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `java -jar nanopub.jar sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 319.5 ± 11.2 | 296.0 | 337.5 | 60.74 ± 2.49 |
| `np sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 446.6 ± 3.2 | 441.2 | 457.6 | 84.93 ± 1.93 |
| `target/release/nanopub-cli sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 5.3 ± 0.1 | 5.1 | 6.3 | 1.00 |

> Tested in GitHub actions on Ubuntu.
