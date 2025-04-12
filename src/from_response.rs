use anyhow::{Context, Result};

pub trait FromResponse: Sized {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self>;
}

impl<T: serde::de::DeserializeOwned> FromResponse for T {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self> {
        let url = response.url().to_owned();
        let body = response.bytes()?;

        serde_json::from_slice::<T>(&body)
            .with_context(|| format!("Failed to deserialize response from URL: {}", url))
    }
}
