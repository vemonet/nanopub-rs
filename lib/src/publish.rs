// use crate::error::NpError;
use std::error::Error;
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
        .send();
    match res {
        Ok(r) => r.status() == 201,
        Err(_e) => false,
    }
    // res.status() == 201
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
            Ok(r) => {
                if r.status() == 201 {
                    published = true;
                } else {
                    println!("{}", r.text());
                }
            }
            Err(e) => println!(e),
        }
    });
    published
}
