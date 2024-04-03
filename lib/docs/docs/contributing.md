# 🛠️ Development

[![Build](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml) [![Lint and Test](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml/badge.svg)](https://github.com/vemonet/nanopub-rs/actions/workflows/test.yml) [![codecov](https://codecov.io/gh/vemonet/nanopub-rs/graph/badge.svg?token=BF15PSO6GN)](https://codecov.io/gh/vemonet/nanopub-rs) [![dependency status](https://deps.rs/repo/github/vemonet/nanopub-rs/status.svg)](https://deps.rs/repo/github/vemonet/nanopub-rs)

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/vemonet/nanopub-rs/issues)
2. [Fork](https://github.com/vemonet/nanopub-rs/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. [Commit](https://www.conventionalcommits.org/en/v1.0.0/), and send a Pull Request.

## 📥️ Clone the repository

Clone the `nanopub-rs` repository, and `cd` into it:

```bash
git clone https://github.com/vemonet/nanopub-rs.git
cd nanopub-rs
```

## ⚙️ Install dependencies

[Rust](https://www.rust-lang.org/tools/install), [Python](https://www.python.org/downloads/), and [NodeJS](https://nodejs.org/en/download) are required for development.

Install development dependencies:

```bash
# Create and activate python virtual env
python3 -m venv .venv
source .venv/bin/activate

# Install python dependencies
pip install -r python/requirements.txt

# Install pre-commit hooks
pre-commit install

# Install rust dev tools
rustup update
cargo install wasm-pack cargo-tarpaulin cargo-deny cargo-outdated git-cliff
```

## 🧪 Run tests

### 🦀 Test Rust crate

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

Test the `nanopub` crate with code coverage (much slower):

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

## 🧼 Format & lint

Automatically format the codebase using `rustfmt`:

```bash
cargo fmt
```

Lint with `clippy`:

```bash
cargo clippy --all --all-targets --all-features
```

## 📖 Generate docs

Start docs website locally with mkdocs:

```bash
./scripts/docs.sh
```

## 🎭️ Work on the demo webpage

Start a web server at [localhost:3000/playground.html](http://localhost:3000/playground.html)

```bash
python -m http.server 3000 --directory ./lib/docs
```

## ️⛓️ Check supply chain

Check the dependency supply chain: licenses (only accept dependencies with OSI or FSF approved licenses), and vulnerabilities (CVE advisories).

```bash
cargo deny check
```

Make sure dependencies have been updated:

```bash
cargo update
cargo outdated
```

## 🏷️ Publish a new release

Building and publishing artifacts will be done by the [`build.yml`](https://github.com/vemonet/nanopub-rs/actions/workflows/build.yml) GitHub actions workflow, make sure you have set the following tokens as secrets for this repository: `PYPI_TOKEN`, `NPM_TOKEN`, `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

Then just run the release script providing the new version following [semantic versioning](https://semver.org), it will bump the version in the `Cargo.toml` files, generate the changelog from commit messages, create a new tag, and push to GitHub:

```bash
./scripts/release.sh 0.0.2
```

!!! success "Automated release"

    The `build.yml` workflow will automatically build artifacts (binaries, pip wheels, npm package), create a new release on GitHub, and add the generated artifacts to the new release.
