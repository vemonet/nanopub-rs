# Introduction

[![crates.io](https://img.shields.io/crates/v/nanopub.svg)](https://crates.io/crates/nanopub)
[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)
[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)

This project aims to provide a comprehensive cross-platform toolkit to sign, publish, and verify **[Nanopublications](https://nanopub.net)**.

Whether you're a developer looking to integrate nanopub functionalities into your application or a researcher seeking an efficient way to handle nanopublications, `nanopub-rs` offers a suite of tools tailored to meet  your needs.

## 🔑 Key Features

### ✨ Nanopub management

- ✍️ **Sign & Publish** nanopubs using a RSA private key. Customize your workflow with a `profile.yml` file.
- 🔍 **Verify**: ensure the integrity of nanopubs by checking their validity, whether they are signed or unsigned.
- 📥 **Fetch** nanopubs from the network using their URI.

### 📦️ Packaged for multiple interfaces

This library is packaged for easy use across various interfaces and languages:

- ⌨️  **Terminal enthusiasts**: binary with a Command Line Interface (CLI) for straightforward terminal operations.
- 🦀 **Rust developers**: available as a Rust crate `nanopub`.
- 🐍 **Python programmers**: available as a Python pip package `nanopub-sign`.
- 🌐 **Web developers**: available as a NPM package `@nanopub/sign`, compiled to [WebAssembly](https://webassembly.org/), for browser integrations with JavaScript, or NodeJS.

### ⚔️ Cross-platform support

It runs seamlessly on:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

### 🧩 RDF serialization support

The library handles most RDF serializations supporting quads, including TriG, Nquads, and JSON-LD.

### 📝 Automated metadata creation

When you sign a nanopub, if it has not already been defined in the pubinfo graph yet:

- 🕓 **Automatic timestamp**: the `dct:created` attribute is used to add the date and time of creation.
- 🪪 **Creator identification**: if an ORCID is provided in the profile, it's added using `dct:creator`. The library also checks for ORCID set with `prov:wasAttributedTo` or `pav:createdBy`.

## 👆 Interactive demo

Experience `nanopub-rs` in action! Visit the **[demo page](https://vemonet.github.io/nanopub-rs/demo.html)** to sign Nanopublications, or generate and register a new key pair, directly in your browser using the NPM version of this library.

## 🚀 Getting started

Checkout the page most adapted to your use-case to get started!

> 💡 **Need Help or Have Suggestions?** We welcome your input and feedback! If you encounter any issues or have ideas to enhance this tool, please [create an issue](https://github.com/vemonet/nanopub-rs/issues) on our GitHub repository.
