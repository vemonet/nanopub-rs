// #![deny(
//     future_incompatible,
//     nonstandard_style,
//     rust_2018_idioms,
//     trivial_casts,
//     trivial_numeric_casts,
//     unsafe_code,
//     unused_qualifications
// )]

mod nanopub;

use crate::nanopub::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Nanopub Python bindings
#[pymodule]
fn nanopub_sign(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__package__", "nanopub-sign")?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    m.add_class::<NpProfilePy>()?;
    m.add_class::<NanopubPy>()?;
    m.add_wrapped(wrap_pyfunction!(get_np_server))
    // m.add_function(wrap_pyfunction!(sum_as_string, m))
}
