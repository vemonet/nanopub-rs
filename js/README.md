# Nanopub bindings to JavaScript

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

## Install

```bash
dnf install perl
# cf. https://github.com/openssl/openssl/issues/13761
cargo install wasm-pack
```

## Build

```bash
wasm-pack build --target web
```

## Try

```bash
python3 -m http.server
```

## Build for publishing

```bash
wasm-pack build --target bundler
```