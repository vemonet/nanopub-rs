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
    // println!("DEBUG: publish resp: {:#?}", res);
    // Ok(res.status() == 201)
    match res.status() {
        reqwest::StatusCode::CREATED => Ok(true),
        _ => {
            // Get the error message from the response body
            let error_msg = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error while publishing the nanopub".to_string());
            Err(NpError(error_msg))
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
