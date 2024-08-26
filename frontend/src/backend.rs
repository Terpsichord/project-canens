use implicit_clone::ImplicitClone;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use std::rc::Rc;

const URL_PREFIX: &str = "https://project-canens.vercel.app/api";

#[derive(Clone, ImplicitClone, Default)]
pub struct BackendClient {
    client: Rc<Client>,
}

impl PartialEq for BackendClient {
    fn eq(&self, _other: &Self) -> bool {
        //TODO: Not sure what this value should be
        true
    }
}

impl BackendClient {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<I, K, V>(&self, path: &str, params: I) -> anyhow::Result<RequestBuilder>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url = Url::parse_with_params(URL_PREFIX, params)?.join(path)?;
        Ok(self.client.get(url))
    }

    pub async fn get_json<T, I, K, V>(&self, path: &str, params: I) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        I: IntoIterator,
        <I as IntoIterator>::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        Ok(self.get(path, params)?.send().await?.json().await?)
    }
}
