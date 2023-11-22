use crate::error::NpError;

/// Publish nanopub RDF string to a given nanopub server URL
pub async fn publish_np(server: &str, np: &str) -> Result<bool, NpError> {
    let server = server.to_string();
    let np = np.to_string();
    let client = reqwest::Client::new();
    let res = client
        .post(&server)
        .body(np)
        .header(reqwest::header::CONTENT_TYPE, "application/trig")
        .send()
        .await?;
    Ok(res.status() == 201)
}

/// Fetch nanopub from its URI
pub async fn fetch_np(uri: &str) -> Result<String, NpError> {
    let client = reqwest::Client::new();
    let res = client
        .get(uri)
        .header(reqwest::header::ACCEPT, "application/trig")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
