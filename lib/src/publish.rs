use crate::error::NpError;

pub async fn publish_np(url: &str, np: &str) -> Result<bool, NpError> {
    let url = url.to_string();
    let np = np.to_string();
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(np)
        .header(reqwest::header::CONTENT_TYPE, "application/trig")
        .send()
        .await?;
    Ok(res.status() == 201)
}
