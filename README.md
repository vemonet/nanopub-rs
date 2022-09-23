## Nanopub signing process

- preliminary nanopub is created with blank space in URIs at the places where the trusty URI code will appear; this includes the signature part, except the triple that is stating the actual signature
- preliminary nanopub is serialized in a normalized fashion (basically each quad on four lines with minimal escaping)
- Signature is calculated on this normalized representation
- Signature triple is added
- Trusty URI code is calculated on normalized representation that includes signature
- Trusty URI code is added in place of all the occurrences of blank spaces in the URIs, leading to the final trusty nanopub

## Python

https://docs.rs/pyo3/latest/pyo3/


Install maturin:

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin
```


Try the thing:

```python
import string_sum
string_sum.sum_as_string(5, 20)
>>> '25'
```

Start in dev:

```bash
maturin develop
```

Build the wheel:

```bash
maturin build
```

## JavaScript

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

```bash
cargo install wasm-pack
```

Build:

```bash
wasm-pack build --target web
```

