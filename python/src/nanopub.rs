use nanopub::{get_np_server as get_server, Nanopub, NpProfile};
use pyo3::{exceptions::PyException, prelude::*};
use pythonize::pythonize;
// use pyo3::types::IntoPyDict;
// use pyo3_asyncio::generic::future_into_py;
use tokio::runtime::Runtime;

#[pyclass(name = "Nanopub", module = "nanopub_sign")]
#[derive(Clone)]
pub struct NanopubPy {
    np: Nanopub,
}

#[pymethods]
impl NanopubPy {
    #[new]
    #[pyo3(text_signature = "(rdf)")]
    fn new(rdf: &str) -> PyResult<Self> {
        Nanopub::new(rdf)
            .map(|np| Self { np })
            .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")))
    }

    #[staticmethod]
    #[pyo3(text_signature = "(rdf)")]
    fn check(rdf: &str) -> PyResult<Self> {
        Nanopub::new(rdf)
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))?
            .check()
            .map(|np| Self { np })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))
    }

    // NOTE: should we make check a class method (instead of static)?
    // But error with IRI in sophia dataset when cloning
    // #[pyo3(text_signature = "(rdf)")]
    // fn check(&mut self) -> PyResult<Self> {
    //     self.clone().np.check()
    //         .map(|np| Self { np })
    //         .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))
    // }

    #[staticmethod]
    #[pyo3(text_signature = "(rdf, profile)")]
    fn sign(rdf: &str, profile: &NpProfilePy) -> PyResult<Self> {
        Nanopub::new(rdf)
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Signing: {e}")))?
            .sign(&profile.profile)
            .map(|np| Self { np })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Signing: {e}")))
    }

    #[staticmethod]
    #[pyo3(text_signature = "(rdf, profile, server_url)")]
    fn publish(rdf: &str, profile: &NpProfilePy, server_url: Option<&str>) -> PyResult<Self> {
        let rdf = rdf.to_string();
        let profile = profile.profile.clone();
        let server_url = server_url.map(str::to_string);
        // Use a tokio runtime to wait on the async operation
        let rt = Runtime::new().map_err(|e| {
            PyErr::new::<PyException, _>(format!("Failed to create Tokio runtime: {e}"))
        })?;
        let result = rt.block_on(async move {
            Nanopub::new(&rdf)
                .map_err(|e| PyErr::new::<PyException, _>(format!("Error Publishing: {e}")))?
                .publish(Some(&profile), server_url.as_deref())
                .await
                .map_err(|e| PyErr::new::<PyException, _>(format!("Error Publishing: {e}")))
        });
        result.map(|np| Self { np })
    }

    #[pyo3(text_signature = "($self)")]
    fn get_rdf(&self, _py: Python<'_>) -> PyResult<String> {
        // py.allow_threads(|| Ok(self.np.get_rdf()))
        self.np
            .get_rdf()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error getting RDF: {e}")))
    }

    #[pyo3(text_signature = "($self)")]
    fn info(&self, py: Python<'_>) -> PyResult<PyObject> {
        pythonize(py, &self.np.info).map_err(|e| {
            PyErr::new::<PyException, _>(format!("Error converting struct info to dict: {e}"))
        })
    }

    // ASYNC WITH TOKIO
    // #[staticmethod]
    // #[pyo3(text_signature = "(rdf, profile, server_url)")]
    // fn apublish(py: Python<'_>, rdf: &str, profile: &NpProfilePy, server_url: Option<&str>) -> PyResult<PyObject> {
    //     let rdf = rdf.to_string();
    //     let profile = profile.profile.clone();
    //     let server_url = server_url.map(str::to_string);

    //     let future = async move {
    //         let np = Nanopub::publish(&rdf, &profile, server_url.as_deref()).await.unwrap();
    //         Ok(NanopubPy { np })
    //     };
    //     Ok(future_into_py(py, future)?.into())
    // }

    // SYNC WITH FUTURES
    // #[staticmethod]
    // #[pyo3(text_signature = "(rdf, private_key, server_url=None)")]
    // fn publish(
    //     rdf: &str,
    //     profile: &NpProfilePy,
    //     server_url: Option<&str>,
    //     // py: Python<'_>,
    // ) -> PyResult<Self> {
    //     // py.allow_threads(|| { // Put code in this block to enable true parallel https://pyo3.rs/v0.20.0/parallelism
    //     // let profile = NpProfile::new(private_key, orcid, "", None).unwrap();
    //     let rdf = rdf.to_string();
    //     let profile = profile.profile.clone();
    //     let server_url = server_url.map(str::to_string);

    //     let future = async move {
    //         Self {
    //             np: Nanopub::publish(&rdf, &profile, server_url.as_deref()).await.unwrap()
    //                 // .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("Error: {}", e)))
    //         }
    //     };
    //     Ok(block_on(future))
    // }

    // ASYNC WITH FUTURES
    // #[staticmethod]
    // #[pyo3(text_signature = "(rdf, profile, server_url)")]
    // fn apublish(py: Python<'_>, rdf: &str, profile: &NpProfilePy, server_url: Option<&str>) -> PyResult<PyObject> {
    //     let rdf = rdf.to_string();
    //     let profile = profile.profile.clone();
    //     let server_url = server_url.map(str::to_string);

    //     let future = async move {
    //         let np = Nanopub::publish(&rdf, &profile, server_url.as_deref()).await.unwrap();
    //         Ok(NanopubPy { np })
    //     };

    //     // Use FutureExt to convert the future into a Python-compatible future
    //     let py_future = future.boxed().into_py(py);

    //     // Return the Python future object
    //     Ok(py_future)
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
    #[pyo3(text_signature = "(private_key, orcid_id, name, introduction_nanopub_uri)")]
    fn new(
        private_key: &str,
        orcid_id: &str,
        name: &str,
        introduction_nanopub_uri: Option<&str>,
    ) -> PyResult<Self> {
        NpProfile::new(private_key, orcid_id, name, introduction_nanopub_uri)
            .map(|profile| Self { profile })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error getting profile: {e}")))
    }
}

/// Return a random server
#[pyfunction]
#[pyo3(text_signature = "(random)")]
pub fn get_np_server(random: Option<bool>) -> PyResult<String> {
    Ok(get_server(random.unwrap_or(true)).to_string())
}
