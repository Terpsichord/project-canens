// TODO: Decide between using the page scraped from here or the search result page
use anyhow::{bail, Context};
use ordered_float::OrderedFloat;
use reqwest::{Client, Response};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tap::Tap;

pub async fn get_tabs_url(song_title: &str, artist: &str) -> anyhow::Result<String> {
    let text = get_search_result_text(song_title, artist)
        .await
        .context("Failed to get tabs search results")?;
    let results = parse_search_results(&text).context("Failed to parse tabs search results")?;

    Ok(get_best_search_result_url(&results)
        .context("Failed to get best tabs result")?
        .to_string())
}

async fn get_search_result_text(song_title: &str, artist: &str) -> anyhow::Result<String> {
    let params = serde_urlencoded::to_string([
        ("search_type", "title"),
        ("value", &format!("{} {}", song_title, artist)),
    ])?;

    let url = format!("https://www.ultimate-guitar.com/search.php?{}", params);
    let request_future = get_cross_origin(&url);

    let text = request_future
        .await
        .context("Failed to perform GET request")?
        .text()
        .await
        .context("Failed to get text from request")?;

    Ok(text)
}

#[cfg(debug_secrets)]
async fn get_cross_origin(url: &str) -> anyhow::Result<Response> {
    Ok(Client::builder()
        .build()?
        .get(&format!("https://proxy.cors.sh/{url}"))
        .header(
            "x-cors-api-key",
            include!(concat!(env!("OUT_DIR"), "/cors_secret.rs")),
        )
        .send()
        .await?)
}

#[cfg(not(debug_secrets))]
async fn get_cross_origin(_url: &str) -> anyhow::Result<Response> {
    bail!("Can't currently make cross-origin requests")
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    tab_url: String,
    artist_name: String,
    song_name: String,
    #[serde(rename = "type")]
    type_: Option<String>,
    rating: Option<f64>,
    votes: Option<usize>,
}

fn parse_search_results(text: &str) -> anyhow::Result<Vec<SearchResult>> {
    let document = Html::parse_document(text);

    let selector = Selector::parse("div.js-store").unwrap();

    let data_content = document
        .select(&selector)
        .next()
        .context("Failed to find div with class 'js-store'")?
        .value()
        .attr("data-content")
        .context("Failed to find 'data-content' attribute on <div class='js-store'>")?;

    let mut json: Value = serde_json::from_str(data_content)
        .context("Failed to parse JSON from 'data-content' attribute on <div class='js-store'>")?;

    let results = serde_json::from_value(
        json["store"]["page"]["data"]["results"]
            .take()
            .tap(|v| log::debug!("json value: {:#?}", v)),
    )?;

    log::debug!("{:?}", results);

    Ok(results)
}

fn get_best_search_result_url(results: &[SearchResult]) -> anyhow::Result<&str> {
    results
        .iter()
        .filter(|result| {
            // TODO: Have buttons/drop-downs for different types (i.e. chords/tabs)
            (result.type_ == Some("Tabs".into()) || result.type_ == Some("Chords".into()))
                && result.rating.is_some()
                && result.votes.is_some()
        })
        .max_by_key(|result| {
            let balanced_rating = result.rating.expect("result.rating.is_some()") - 3.0;
            log::debug!("{} balanced: {}", result.tab_url, balanced_rating);
            OrderedFloat(balanced_rating * result.votes.expect("result.votes.is_some()") as f64)
        })
        .context("Got empty results")
        .map(|result| &*result.tab_url)
}
