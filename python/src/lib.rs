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

/// Nanopub Python bindings
#[pymodule]
// fn nanopub_py(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
fn nanopub_py(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
    module.add("__package__", "nanopub_py")?;
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("__author__", env!("CARGO_PKG_AUTHORS").replace(':', "\n"))?;

    module.add_class::<PyNanopub>()

    // module.add_class::<PyNanopub>()?;
    // io::add_to_module(module)
}
