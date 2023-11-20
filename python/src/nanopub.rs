use nanopub::{Nanopub, NpProfile};
use pyo3::prelude::*;

#[pyclass(name = "Nanopub", module = "nanopub_sign")]
#[derive(Clone)]
pub struct NanopubPy {
    np: Nanopub,
}

#[pymethods]
impl NanopubPy {
    // #[new]
    #[staticmethod]
    #[pyo3(text_signature = "(rdf)")]
    fn check(rdf: &str) -> PyResult<Self> {
        let np = Nanopub::check(rdf).unwrap();
        Ok(Self { np })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(rdf, private_key, orcid, server_url=None)")]
    fn publish(
        rdf: &str,
        profile: &NpProfilePy,
        // private_key: &str,
        // orcid: &str,
        server_url: Option<&str>,
        // py: Python<'_>,
    ) -> PyResult<Self> {
        // py.allow_threads(|| { // Put code in this block to enable true parallel https://pyo3.rs/v0.20.0/parallelism
        // let profile = NpProfile::new(orcid, "", private_key, None).unwrap();

        let np = Nanopub::publish(rdf, &profile.profile, server_url).unwrap();
        // Nanopub::sign(rdf, &profile).unwrap()
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
    fn get_rdf(&self, _py: Python<'_>) -> PyResult<String> {
        // py.allow_threads(|| Ok(self.np.get_rdf()))
        Ok(self.np.get_rdf())
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

#[pyclass(name = "NpProfile", module = "nanopub_sign")]
#[derive(Clone)]
pub struct NpProfilePy {
    profile: NpProfile,
}

#[pymethods]
impl NpProfilePy {
    #[new]
    #[pyo3(text_signature = "(orcid_id, name, private_key, introduction_nanopub_uri)")]
    fn new(
        orcid_id: &str,
        name: &str,
        private_key: &str,
        introduction_nanopub_uri: Option<&str>,
    ) -> PyResult<Self> {
        let profile =
            NpProfile::new(orcid_id, name, private_key, introduction_nanopub_uri).unwrap();
        Ok(Self { profile })
    }
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
