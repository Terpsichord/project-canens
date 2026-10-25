use anyhow::anyhow;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum BackendResult<T: Debug> {
    Ok(T),
    Err { error: String },
}

pub async fn get<T, P>(path: &str, params: P) -> anyhow::Result<T>
where
    T: DeserializeOwned + Debug,
    P: Serialize,
{
    let encoded_params = serde_urlencoded::to_string(params)?;
    let url = create_url(path, &encoded_params);

    let result = Request::get(&url)
        .send()
        .await?
        .json::<BackendResult<T>>()
        .await?;

    match result {
        BackendResult::Ok(response) => Ok(response),
        BackendResult::Err { error } => Err(anyhow!(error).context("Backend returned an error")),
    }
}

#[cfg(debug_assertions)]
fn create_url(path: &str, params: &str) -> String {
    log::debug!("fetching from debug \"/api\" url");
    format!("/api{}?{}", path, params)
}

#[cfg(not(debug_assertions))]
fn create_url(path: &str, params: &str) -> String {
    log::debug!("fetching from release \"shuttleapp.rs\" url");
    format!("https://canens.shuttleapp.rs{}?{}", path, params)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn parse_backend_err() {
        let json = "{\"error\":\"http error: status code 400 Bad Request\"}";

        let result: BackendResult<()> = serde_json::from_str(&json).unwrap();

        assert_eq!(
            result,
            BackendResult::Err {
                error: "http error: status code 400 Bad Request".to_string()
            }
        );
    }

    #[wasm_bindgen_test]
    fn parse_backend_ok() {
        let sample_data = vec![1, 2, 3];

        let json = serde_json::to_string(&sample_data).unwrap();

        let result: BackendResult<Vec<i32>> = serde_json::from_str(&json).unwrap();

        assert_eq!(result, BackendResult::Ok(sample_data));
    }
}
