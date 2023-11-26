# Introduction

[![crates.io](https://img.shields.io/crates/v/nanopub.svg)](https://crates.io/crates/nanopub)
[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)
[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)

This project aims to provide a comprehensive cross-platform toolkit to sign, publish, and check **[Nanopublications](https://nanopub.net)**.

Sign and publish providing a private RSA key string, or a `profile.yml` file. Check the validity of signed or unsigned Nanopublications.

It is packaged to be used easily through various popular interfaces:

- ⌨️ Binary with a CLI for use in the terminal
- 🦀 Crate `nanopub` for Rust
- 🐍 Pip package `nanopub-sign` for Python
- 📦️ NPM package `@nanopub/sign` for JavaScript (compiled to WebAssembly) in the browser, or with NodeJS

On all platforms:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

The library automatically handles most RDF serializations supporting quads for the nanopub:

- TriG
- Nquads
- JSON-LD

When signing a nanopub, some metadata in the pubinfo graph are created automatically if they are not already set in the RDF provided:

- Date and time of the Nanopublication creation using `dct:created`.
- ORCID of the creator using `dct:creator`, if an ORCID was provided in the profile used to sign the Nanopublication (we also check if the ORCID has been set with `prov:wasAttributedTo`, or `pav:createdBy`)

> 💡 If you are facing any problem, or have ideas to help improve this project, please [create an issue](https://github.com/vemonet/nanopub-rs/issues) on GitHub.
