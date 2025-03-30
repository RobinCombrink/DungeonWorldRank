use std::fs;

use anyhow::{Context, Result};

pub trait FromResponse: Sized {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self>;
}

impl<T: serde::de::DeserializeOwned> FromResponse for T {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self> {
        let url = response.url().to_owned();
        let body = response.bytes()?;
        let text = String::from_utf8(body.to_vec())?;
        // println!("{:#?}", text);

        // let file = fs::read("./abc.json").unwrap();
        // Ok(serde_json::from_slice(&file)?)

        serde_json::from_slice::<T>(&body)
            .with_context(|| format!("Failed to deserialize response from URL: {}", url))
    }
}
