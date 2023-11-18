#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::{spawn_local, wasm_bindgen};

// Blocking API not available on wasm, we need to use async with wasm_bindgen_futures

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
    // if res.status() != 201 {
    //     println!("{}", res.text().unwrap());
    // };
    res.status() == 201
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn publish_np(url: &str, np: &str) -> bool {
    let url = url.to_string();
    let np = np.to_string();
    let mut published: bool = false;
    spawn_local(async move {
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
