use nanopub::{get_np_server as get_server, Nanopub, NpProfile};
use pyo3::{exceptions::PyException, prelude::*};
// use pyo3_asyncio::generic::future_into_py;
use tokio::runtime::Runtime;

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
        Nanopub::new(rdf)
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))?
            .check()
            .map(|np| Self { np })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))
    }

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
                .publish(&profile, server_url.as_deref())
                .await
                .map_err(|e| PyErr::new::<PyException, _>(format!("Error Publishing: {e}")))
        });
        result.map(|np| Self { np })
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
    //     // let profile = NpProfile::new(orcid, "", private_key, None).unwrap();
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

    #[pyo3(text_signature = "($self)")]
    fn get_rdf(&self, _py: Python<'_>) -> PyResult<String> {
        // py.allow_threads(|| Ok(self.np.get_rdf()))
        self.np
            .get_rdf()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error getting RDF: {e}")))
    }
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
        NpProfile::new(orcid_id, name, private_key, introduction_nanopub_uri)
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
