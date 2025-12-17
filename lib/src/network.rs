use crate::error::NpError;

/// Publish nanopub RDF string to a given nanopub server URL
pub async fn publish_np(server: &str, np: &str) -> Result<bool, NpError> {
    let client = reqwest::Client::new();
    let res = client
        .post(server)
        .body(np.to_string())
        .header(reqwest::header::CONTENT_TYPE, "application/trig")
        // .header(reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .send()
        .await?;
    // Get the error message from the response body
    let status = res.status();
    // println!("Response: {:?} {}", status, error_msg);
    match status {
        reqwest::StatusCode::CREATED => Ok(true),
        reqwest::StatusCode::OK => Ok(true),
        _ => {
            let error_msg = res.text().await?;
            if error_msg.is_empty() {
                Err(NpError(format!("{status}")))
            } else {
                Err(NpError(format!("{status}: {error_msg}")))
            }
        }
    }
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
