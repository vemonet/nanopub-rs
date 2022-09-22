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

