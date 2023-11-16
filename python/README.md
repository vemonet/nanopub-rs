## Nanopub bindings to python

https://docs.rs/pyo3/latest/pyo3

Install maturin:

```bash
python -m venv .venv
source .venv/bin/activate
pip install "maturin[patchelf]"
```

## Develop

Start in dev:

```bash
maturin develop
```

Try the python lib by running `python try.py`:

```python
from nanopub_sign import Nanopub
np = Nanopub(rdf="tiii")
print(np.get_rdf())
```

## Build

Build the wheel:

```bash
maturin build
```
