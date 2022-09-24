# 🔬🦀 Nanopub rs

A rust toolkit to sign and publish [Nanopublications](https://nanopub.org), with bindings to python and javascript (wasm).

## Nanopub signing process

- preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear; this includes the signature part, except the triple that is stating the actual signature
- preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
- Signature is calculated on this normalized representation
- Signature triple is added
- Trusty URI code is calculated on normalized representation that includes signature
- Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub

## Generate docs

```bash
./scripts/docs-serve.sh
```