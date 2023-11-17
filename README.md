# ✍️⚔️ Nanopub cross-platform toolkit

[![Lint and Test](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml) [![Build](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml) [![Deploy docs to GitHub Pages](https://github.com/vemonet/nanopub-rs/actions/workflows/docs.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/docs.yml)

This project aims to provide a comprehensive cross-platform toolkit to sign, publish, and check [Nanopublications](https://nanopub.net).

It is packaged to be used easily through various popular interfaces:

- ⌨️ Binary with a CLI for use in the terminal
- 🦀 Crate `nanopub` for Rust
- 🐍 Pip package `nanopub_sign` for Python
- 📦️ NPM package `@nanopub/sign` for JavaScript (compiled to WebAssembly) in the browser, or with NodeJS

On all platforms:

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows
- 🦊 Web browsers

## 🧑‍💻 Development

[Rust](https://www.rust-lang.org/tools/install), python, and NodeJS are required for development.

Install development dependencies:

```bash
rustup update
rustup component add rustfmt clippy
```

### ✅ Run tests

Run tests for all packages:

```bash
cargo test
```

Display prints:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test sign_nanopub_test_blank -- --nocapture
```

Run all tests:

```bash
cargo test --verbose --all --all-features
```

Test a specific package:

```bash
cargo test lib
```

> Checkout the README in the `python` and `js` folder for the instructions to build and test for each language

Test signing a nanopublication with the CLI:

```bash
cd lib
cargo run -- sign tests/resources/nanopub_test_blank.trig
```

### ✨ Format

```bash
cargo fmt
```

### 🧹 Lint

```bash
cargo clippy --all --all-targets --all-features
```

### 📖 Generate docs

```bash
./scripts/docs-serve.sh
```

### 📦️ Build and run

All packages at once:

```bash
cargo build --all
cargo run --all-features
```

### 🏷️ New release

```bash
cargo install cargo-release
```

1. Make sure dependencies have been updated:

   ```bash
   cargo update
   cargo outdated
   ```

2. Bump the version in the `Cargo.toml` file in folders `lib/`, `python`, `js`

   ```bash
   # patch, minor, major
   cargo release patch --no-tag --no-publish
   ```

3. Commit, push, and create a new release on GitHub

4. The `build.yml` workflow will automatically build artifacts (binary, pip wheel, npm package), and add them to the new release.

## ☑️ To do

- [ ] Add Nanopub test suite
- [ ] Add brew packaging (c.f. [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/pkg/brew/ripgrep-bin.rb))

## ✒️ Nanopub signing process

- preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear (normalized URI: `http://purl.org/np/ARTIFACTCODE-PLACEHOLDER/`, cf. [code](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/trusty/TempUriReplacer.java#L12)); this includes the signature part, except the triple that is stating the actual signature
- preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
- Signature is calculated on this normalized representation (cf. most of the process in the [trusty-uri python lib](https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15), see also [SignatureUtils](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/extra/security/SignatureUtils.java#L196) and [trusty-uri](https://github.com/trustyuri/trustyuri-java/blob/08b61fbb13d20a5cbefde617bd9a9e9b0b03d780/src/main/java/net/trustyuri/rdf/RdfHasher.java#L86))
- Signature triple is added
- Trusty URI code is calculated on normalized representation that includes signature
- Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub
