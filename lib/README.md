
# Rust library for Nanopublications

A rust toolkit to sign and publish [Nanopublications](https://nanopub.org).

## Build

```bash
cargo build
```

## Try

Try in the `try` folder:

```bash
cd ../try
cargo run
```

## Build docs

```bash
cargo install mdbook
```

Docs from markdown:

```bash
mdbook build
```

Doc from comments:

```bash
rustdoc --crate-name nanopub_rs src/lib.rs -o docs/target/doc -L dependency=docs/target/debug/deps
```