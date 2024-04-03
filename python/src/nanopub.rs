use nanopub::{get_np_server as get_server, profile::gen_keys, Nanopub, NpProfile};
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

    // NOTE: we need to use staticmethod because we can't access self.np otherwise

    // #[staticmethod]
    // #[pyo3(text_signature = "(rdf)")]
    // fn check(rdf: &str) -> PyResult<Self> {
    //     Nanopub::new(rdf)
    //         .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))?
    //         .check()
    //         .map(|np| Self { np })
    //         .map_err(|e| PyErr::new::<PyException, _>(format!("Error Checking: {e}")))
    // }

    // NOTE: should we make check a class method (instead of static)?
    // But error with IRI in sophia dataset when cloning
    #[pyo3(text_signature = "($self)")]
    fn check(&self) -> PyResult<Self> {
        self.np
            .clone()
            .check()
            .map(|np| Self { np })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error checking: {e}")))
    }

    #[pyo3(text_signature = "($self, profile)")]
    fn sign(&self, profile: &NpProfilePy) -> PyResult<Self> {
        self.np
            .clone()
            .sign(&profile.profile)
            .map(|np| Self { np })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error signing: {e}")))
    }

    #[pyo3(text_signature = "($self, profile, server_url)")]
    fn publish(&self, profile: &NpProfilePy, server_url: Option<&str>) -> PyResult<Self> {
        let server_url = server_url.map(str::to_string);
        // Use a tokio runtime to wait on the async operation
        let rt = Runtime::new()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Runtime failed: {e}")))?;
        let result: Result<Nanopub, PyErr> = rt.block_on(async move {
            self.np
                .clone()
                .publish(Some(&profile.profile.clone()), server_url.as_deref())
                .await
                .map_err(|e| PyErr::new::<PyException, _>(format!("Error publishing: {e}")))
        });
        result.map(|np| Self { np })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(profile, server_url=None)")]
    fn publish_intro(profile: &NpProfilePy, server_url: Option<&str>) -> PyResult<Self> {
        let server_url = server_url.map(str::to_string);
        // Use a tokio runtime to wait on the async operation
        let rt = Runtime::new()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Runtime failed: {e}")))?;
        let result = rt.block_on(async move {
            // let np = Nanopub::new_intro(&profile.profile)
            //     .map(|np| Self { np })
            //     .map_err(|e| PyErr::new::<PyException, _>(format!("{e}")));
            let np = match Nanopub::new_intro(&profile.profile) {
                Ok(np) => np,
                Err(e) => {
                    return Err(PyErr::new::<PyException, _>(format!(
                        "Failed to create nanopub introduction: {e}"
                    )))
                }
            };
            // match np.publish(Some(&profile), Some(&server_url)).await {
            //     Ok(np) => Ok(JsValue::from(Nanopub { np })),
            //     Err(e) => Err(JsValue::from_str(&format!(
            //         "Error publishing Nanopub Introduction: {e}"
            //     ))),
            // }
            np.publish(Some(&profile.profile), server_url.as_deref())
                .await
                .map_err(|e| PyErr::new::<PyException, _>(format!("Error publishing: {e}")))
        });
        result.map(|np| Self { np })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(uri)")]
    fn fetch(uri: &str) -> PyResult<Self> {
        let rt = Runtime::new()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Runtime failed: {e}")))?;
        let result = rt.block_on(async move {
            Nanopub::fetch(uri)
                .await
                .map_err(|e| PyErr::new::<PyException, _>(format!("Error fetching: {e}")))
        });
        result.map(|np| Self { np })
    }

    #[pyo3(text_signature = "($self)")]
    fn rdf(&self, _py: Python<'_>) -> PyResult<String> {
        // py.allow_threads(|| Ok(self.np.rdf()))
        self.np
            .rdf()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error getting RDF: {e}")))
    }

    #[pyo3(text_signature = "($self)")]
    fn info(&self, py: Python<'_>) -> PyResult<PyObject> {
        pythonize(py, &self.np.info).map_err(|e| {
            PyErr::new::<PyException, _>(format!("Error converting struct info to dict: {e}"))
        })
    }

    // TODO: use pyo3-asyncio https://pyo3.rs/v0.21.1/ecosystem/async-await
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
        introduction_nanopub_uri: Option<String>,
    ) -> PyResult<Self> {
        NpProfile::new(private_key, orcid_id, name, introduction_nanopub_uri)
            .map(|profile| Self { profile })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error getting profile: {e}")))
    }
}

#[pyclass(name = "KeyPair", module = "nanopub_sign")]
#[derive(Clone)]
pub struct KeyPair {
    #[pyo3(get)]
    pub private: String,
    #[pyo3(get)]
    pub public: String,
}

#[pymethods]
impl KeyPair {
    #[new]
    #[pyo3(text_signature = "()")]
    fn new() -> PyResult<Self> {
        gen_keys()
            .map(|(private, public)| Self { private, public })
            .map_err(|e| PyErr::new::<PyException, _>(format!("Error generating key pair: {e}")))
    }
}

/// Return a random server
#[pyfunction]
#[pyo3(text_signature = "(random)")]
pub fn get_np_server(random: Option<bool>) -> PyResult<String> {
    Ok(get_server(random.unwrap_or(true)).to_string())
}
