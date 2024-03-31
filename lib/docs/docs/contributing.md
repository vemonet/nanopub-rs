# 🛠️ Contributing

[![Build](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml) [![Lint and Test](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/vemonet/nanopub-rs/graph/badge.svg?token=BF15PSO6GN)](https://codecov.io/gh/vemonet/nanopub-rs) [![dependency status](https://deps.rs/repo/github/vemonet/nanopub-rs/status.svg)](https://deps.rs/repo/github/vemonet/nanopub-rs)

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/vemonet/nanopub-rs/issues)
2. [Fork](https://github.com/vemonet/nanopub-rs/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.

## ️🗺️ Architecture details

### 🗃️ Folder structure

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

### ✒️ Nanopub signing process

- Preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear (normalized URI: `https://w3id.org/np/ `, cf. [code](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/trusty/TempUriReplacer.java#L12)); this includes the signature part, except the triple that is stating the actual signature
- Preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
- Signature is calculated on this normalized representation (cf. most of the process in the [trusty-uri python lib](https://github.dev/trustyuri/trustyuri-python/blob/9f29732c4abae9d630d36e6da24720e02f543ebf/trustyuri/rdf/RdfHasher.py#L15), see also [SignatureUtils](https://github.com/Nanopublication/nanopub-java/blob/22bba0e79508309f1c6163970f49ab596beadeb0/src/main/java/org/nanopub/extra/security/SignatureUtils.java#L196) and [trusty-uri](https://github.com/trustyuri/trustyuri-java/blob/08b61fbb13d20a5cbefde617bd9a9e9b0b03d780/src/main/java/net/trustyuri/rdf/RdfHasher.java#L86))
- Signature triple is added
- Trusty URI code is calculated on normalized representation that includes signature
- Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub

### Notes about maintenance and stability

Cross-compiling to many targets brings some complexity to the build process, especially that the nanopub lib packs a lot of features: processing RDF, RSA signing and key generation, querying a HTTP server, getting current datetime access.

This means we need to make sure the dependencies we use works for all targets.

### ☑️ To do

- [ ] Add possibility to build the nanopub from scratch for JS and python
- [ ] Integrate to the python `nanopub` library to perform signing?
- [ ] Add Ruby bindings? https://docs.rs/magnus/latest/magnus https://github.com/ankane/tokenizers-ruby
- [ ] Add Java bindings? https://docs.rs/jni/latest/jni
- [ ] Add brew packaging? (cf. [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/pkg/brew/ripgrep-bin.rb))

## 🧑‍💻 Development workflow

[Rust](https://www.rust-lang.org/tools/install), python, and NodeJS are required for development.

Install development dependencies:

```bash
# Create and activate python virtual env
python3 -m venv .venv
source .venv/bin/activate

# Install python dependencies
pip install maturin pre-commit

# Install pre-commit hooks
pre-commit install

# Install rust dev tools
rustup update
cargo install wasm-pack cargo-tarpaulin cargo-deny git-cliff
```

### 📥️ Clone the repository

Clone the `nanopub-rs` repository, `cd` into it, and create a new branch for your contribution:

```bash
cd nanopub-rs
git checkout -b add-my-contribution
```

###  🧪 Test Rust crate

Run tests for the rust crate:

```bash
cargo test
```

!!! example "More options"

    Display prints:

    ```bash
    cargo test -- --nocapture
    ```

    Run a specific test:

    ```bash
    cargo test sign_nanopub_blank -- --nocapture
    ```

    If tests panic without telling on which test it failed:

    ```bash
    cargo test -- --test-threads=1
    ```

Test the `nanopub` crate with code coverage:

```bash
cargo tarpaulin -p nanopub --out html
```

### 🐍 Test Python package

Build the pip package and run `pytest` tests:

```bash
./scripts/test-python.sh
```

Or just run the tests:

```bash
source .venv/bin/activate
cd python
pytest
```

### 🟨 Test JavaScript package

Build the npm package and run `jest` tests:

```bash
./scripts/test-js.sh
```

Start a web server to access the dev webpage:

```bash
python -m http.server 3000 --directory ./js
```

Open [localhost:3000](http://localhost:3000) in your web browser.

### ⌨️ Test CLI

Test signing a nanopublication with the commandline interface:

```bash
cd cli
cargo run -- sign ../lib/tests/resources/nanopub_test_blank.trig
```

### 🌈 Run all tests

```bash
./scripts/test-all.sh
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
./scripts/docs.sh
```

### 🎭️ Work on the demo webpage

Start a web server at [localhost:3000/playground.html](http://localhost:3000/playground.html)

```bash
python -m http.server 3000 --directory ./lib/docs
```

### 📦️ Build and run

All packages at once:

```bash
cargo build --all
cargo run --all-features
```

### ️⛓️ Check supply chain

Check the dependency supply chain: licenses (only accept dependencies with OSI or FSF approved licenses), and vulnerabilities (CVE advisories).

```bash
cargo deny check
```

### 🏷️ New release

Publishing artifacts will be done by the `build.yml` workflow, make sure you have set the following tokens as secrets for this repository: `PYPI_TOKEN`, `NPM_TOKEN`, `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

Install dependency:

```bash
cargo install cargo-release cargo-outdated
```

1. Make sure dependencies have been updated:

   ```bash
   cargo update
   cargo outdated
   ```

2. Run the release script, it will bump the version in the `Cargo.toml` files, generate the changelog, commit, create a new tag, and push to GitHub

   ```bash
   ./scripts/release.sh 0.0.2
   ```

4. The `build.yml` workflow will automatically build artifacts (binary, pip wheel, npm package), create a new release on GitHub, and add the artifacts to the new release.

## ⏱️ Speed comparison

Speed taken when signing a nanopub using different languages implementations (in this order: [java](https://github.com/Nanopublication/nanopub-java), [python](https://github.com/fair-workflows/nanopub), rust):

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `java -jar nanopub.jar sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 319.5 ± 11.2 | 296.0 | 337.5 | 60.74 ± 2.49 |
| `np sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 446.6 ± 3.2 | 441.2 | 457.6 | 84.93 ± 1.93 |
| `target/release/nanopub-cli sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 5.3 ± 0.1 | 5.1 | 6.3 | 1.00 |

> Tested in GitHub actions on Ubuntu.

## 🏔️ Changelog

Version history is recorded in the [CHANGELOG.md](https://github.com/vemonet/nanopub-rs/blob/main/CHANGELOG.md).
