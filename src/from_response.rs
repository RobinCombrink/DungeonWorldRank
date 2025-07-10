use std::fs;

use anyhow::{Context, Result};
use log::LevelFilter;

pub trait FromResponse: Sized {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self>;
}

impl<T: serde::de::DeserializeOwned> FromResponse for T {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self> {
        let url = response.url().to_owned();
        let body = response.bytes()?;

        if log::max_level() == LevelFilter::Debug {
            let file = format!("responses/{}.json", url.path().replace("/", "-"));
            fs::create_dir_all("responses")?;
            let contents = String::from_utf8(body.to_vec())?;
            fs::write(file, contents)?;
        }

        serde_json::from_slice::<T>(&body)
            .with_context(|| format!("Failed to deserialize response from URL: {}", url))
    }
}
