use nanopub_rs::nanopub::Nanopub;
use pyo3::prelude::*;

#[pyclass(name = "Nanopub", module = "nanopub_py")]
#[pyo3(text_signature = "(rdf = None)")]
// #[derive(Clone)]
pub struct PyNanopub {
    np: Nanopub,
}

#[pymethods]
impl PyNanopub {
    #[new]
    fn new(rdf: Option<&str>, py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(|| {
            Ok(Self {
                np: Nanopub::new(&rdf.unwrap_or("default in py").to_string()).unwrap(),
            })
            // Ok(Self {
            //     np: Nanopub::new(&rdf.unwrap_or("default in py").to_string()),
            // })
            // Ok(Self {
            //     np: if let Some(rdf) = rdf {
            //         Nanopub::new(rdf.unwrap_or("default in py"))
            //     } else {
            //         Nanopub::new()
            //     }
            //     .map_err(map_storage_error)?,
            // })
        })
    }


    // #[new]
    // fn new(rdf: Option<&str>, py: Python<'_>) -> PyResult<Self> {
    //     py.allow_threads(|| {
    //         Ok(Self {
    //             np: Nanopub::new(&rdf.unwrap_or("default in py").to_string()),
    //         })
    //         // Ok(Self {
    //         //     np: if let Some(rdf) = rdf {
    //         //         Nanopub::new(rdf.unwrap_or("default in py"))
    //         //     } else {
    //         //         Nanopub::new()
    //         //     }
    //         //     .map_err(map_storage_error)?,
    //         // })
    //     })
    // }

    #[pyo3(text_signature = "($self)")]
    fn get_rdf(&self, py: Python<'_>) -> PyResult<String> {
        py.allow_threads(|| Ok(self.np.get_rdf()))
    }

    // /// >>> store.update('DELETE WHERE { <http://example.com> ?p ?o }')
    // #[pyo3(text_signature = "($self, update, *, base_iri)")]
    // #[args(update, "*", base_iri = "None")]
    // fn update(&self, update: &str, base_iri: Option<&str>, py: Python<'_>) -> PyResult<()> {
    //     py.allow_threads(|| {
    //         let update =
    //             Update::parse(update, base_iri).map_err(|e| map_evaluation_error(e.into()))?;
    //         self.np.update(update).map_err(map_evaluation_error)
    //     })
    // }
}

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn string_sum(py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

//     Ok(())
// }
