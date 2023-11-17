#[cfg(not(target_arch = "wasm32"))]
pub fn publish_np(url: &str, np: &str) -> bool {
    let url = url.to_string();
    let np = np.to_string();
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url)
        .body(np)
        .header(reqwest::header::CONTENT_TYPE, "application/trig")
        // .header(header::ACCEPT, "application/json")
        .send()
        .unwrap();
    res.status() == 201
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_futures::wasm_bindgen]
pub fn publish_np(url: &str, np: &str) -> bool {
    use wasm_bindgen::prelude::*;
    let url = url.to_string();
    let np = np.to_string();
    let mut published: bool = false;
    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .body(np)
            .header(reqwest::header::CONTENT_TYPE, "application/trig")
            // .header(header::ACCEPT, "application/json")
            .send()
            .await;

        match res {
            Ok(res) => {
                // Handle successful response
                if res.status() == 201 {
                    // println!("\n🎉 Nanopublication published at {}{}{}", BOLD, np.uri, END);
                    // np.set_published(true);
                    published = true;
                } else {
                    println!("\n❌ Issue publishing the Nanopublication {:#?}", res);
                    // println!("{:#?}", res.text());
                }
            }
            Err(_e) => {
                // Handle error
            }
        }
    });
    published
}
