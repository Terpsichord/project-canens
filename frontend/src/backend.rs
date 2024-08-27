use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub async fn get<T, P>(path: &str, params: P) -> anyhow::Result<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    let query_params = serde_urlencoded::to_string(params)?;
    #[cfg(debug_assertions)]
    let url = format!("/api{}?{}", path, query_params);
    #[cfg(not(debug_assertions))]
    let url = format!("https://canens.shuttleapp.rs{}?{}", path, query_params);

    Ok(Request::get(&url).send().await?.json().await?)
}
