# Nanopub bindings to JavaScript

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

## Install

```bash
cargo install wasm-pack
```

## Build

```bash
wasm-pack build --target web
```

Build for browser and node:

```bash
npm run build
```

## Try it

Code in the `index.html` file

```bash
python3 -m http.server
```

## Build for publishing

```bash
wasm-pack build --target bundler
```