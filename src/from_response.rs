use anyhow::{Context, Result};

pub trait FromResponse: Sized {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self>;
}

impl<T: serde::de::DeserializeOwned> FromResponse for T {
    fn from_response(response: reqwest::blocking::Response) -> Result<Self> {
        let body = response.bytes()?;
        let text = String::from_utf8(body.to_vec())?;
        println!("{:#?}", text);
        serde_json::from_slice::<T>(&body).with_context(|| "Failed to deserialize response")
    }
}
