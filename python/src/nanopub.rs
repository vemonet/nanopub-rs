use nanopub::{Nanopub, NpProfile};
use pyo3::prelude::*;

#[pyclass(name = "Nanopub", module = "nanopub_sign")]
#[pyo3(text_signature = "(rdf, private_key, orcid, server_url=None, publish=False)")]
// #[derive(Clone)]
pub struct NanopubPy {
    np: Nanopub,
}

#[pymethods]
impl NanopubPy {
    #[new]
    fn new(
        rdf: &str,
        private_key: &str,
        orcid: &str,
        server_url: &str,
        publish: bool,
        py: Python<'_>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let profile = NpProfile::new(orcid, "", private_key, None).unwrap();
            let np = if publish {
                Nanopub::publish(
                    // &rdf.unwrap_or("default in py").to_string(),
                    rdf,
                    &profile,
                    Some(server_url),
                )
                .unwrap()
            } else {
                Nanopub::sign(rdf, &profile).unwrap()
            };
            Ok(Self { np })

            // Ok( Self {
            //     rdf: nq_stringifier.serialize_dataset(&mut dataset)?.to_string(),
            //     dataset: dataset,
            //     public_key: public_key.to_string(),
            //     private_key: private_key.to_string(),
            //     orcid: orcid.to_string(),
            //     server_url: if let Some(server_url) = server_url {
            //         server_url.to_string()
            //     } else{
            //         TEST_SERVER.to_string()
            //     },
            //     publish: if let Some(publish) = publish {
            //         publish.clone()
            //     } else {
            //         false
            //     }
            // })

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
