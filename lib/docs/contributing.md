# 🛠️ Contributing

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/vemonet/nanopub-rs/issues)
2. [Fork](https://github.com/vemonet/nanopub-rs/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.

## 🧑‍💻 Development workflow

[![Build](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml) [![Lint and Test](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/vemonet/nanopub-rs/graph/badge.svg?token=BF15PSO6GN)](https://codecov.io/gh/vemonet/nanopub-rs) [![dependency status](https://deps.rs/repo/github/vemonet/nanopub-rs/status.svg)](https://deps.rs/repo/github/vemonet/nanopub-rs)

[Rust](https://www.rust-lang.org/tools/install), python, and NodeJS are required for development.

Install development dependencies:

```bash
# Activate python virtual env
python3 -m venv .venv
source .venv/bin/activate
# Install python dependencies
pip install maturin pre-commit
# Install pre-commit hooks
pre-commit install
# Install rust dev tools
rustup update
rustup component add rustfmt clippy
cargo install wasm-pack cargo-tarpaulin mdbook mdbook-admonish cargo-make
```

### 📥️ Clone the repository

Clone the `nanopub-rs` repository, `cd` into it, and create a new branch for your contribution:

```bash
cd nanopub-rs
git checkout -b add-my-contribution
```

###  🧪 Run tests

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
cargo test sign_nanopub_blank -- --nocapture
```

Test a specific package:

```bash
cargo test lib
```

> Checkout the README in the `python` and `js` folder for the instructions to build and test for each language

Test the `nanopub` create with code coverage:

```bash
cargo tarpaulin -p nanopub --out html
```

Test signing a nanopublication with the CLI:

```bash
cd cli
cargo run -- sign ../lib/tests/resources/nanopub_test_blank.trig
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

2. Bump the version in the `Cargo.toml` file in folders `lib/`, `python`, `js`

   ```bash
   ./scripts/bump.sh 0.0.2
   ```

3. Commit, push, and create a new release on GitHub

4. The `build.yml` workflow will automatically build artifacts (binary, pip wheel, npm package), and add them to the new release.

> TODO: try `cargo release patch --no-tag --no-publish`

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

### ☑️ To do

- [ ] Integrate to the python `nanopub` library to perform signing?
- [ ] Add Ruby bindings? https://docs.rs/magnus/latest/magnus https://github.com/ankane/tokenizers-ruby
- [ ] Add Java bindings? https://docs.rs/jni/latest/jni
- [ ] Add brew packaging (c.f. [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/pkg/brew/ripgrep-bin.rb))?

## ⏱️ Speed comparison

Speed taken when signing a nanopub using different languages implementations (in this order: [java](https://github.com/Nanopublication/nanopub-java), [python](https://github.com/fair-workflows/nanopub), rust):

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `java -jar nanopub.jar sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 319.5 ± 11.2 | 296.0 | 337.5 | 60.74 ± 2.49 |
| `np sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 446.6 ± 3.2 | 441.2 | 457.6 | 84.93 ± 1.93 |
| `target/release/nanopub-cli sign lib/tests/resources/simple1-rsa.trig -k lib/tests/resources/id_rsa` | 5.3 ± 0.1 | 5.1 | 6.3 | 1.00 |

> Tested in GitHub actions on Ubuntu.
