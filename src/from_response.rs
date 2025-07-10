use std::fs;

use anyhow::{Context, Result};
use bytes::Bytes;
use log::LevelFilter;
use reqwest::{Response, Url};

pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> Result<Self>;
}

impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: Response) -> Result<Self> {
        let url = response.url().to_owned();
        let body = response.bytes().await?;

        if log::max_level() == LevelFilter::Debug {
            create_response_file(&url, &body)?;
        }

        serde_json::from_slice::<T>(&body)
            .with_context(|| format!("Failed to deserialize response from URL: {}", url))
    }
}

fn create_response_file(url: &Url, body: &Bytes) -> Result<()> {
    let file = format!("responses/{}.json", url.path().replace("/", "-"));
    fs::create_dir_all("responses")
        .with_context(|| "Could not create debug response output directory")?;
    let contents = String::from_utf8(body.to_vec())?;
    fs::write(&file, contents)
        .with_context(|| format!("Could not write debug response output file: {}", file))
}
