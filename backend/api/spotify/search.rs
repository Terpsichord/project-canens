use project_canens_backend::{handle, spotify_client_creds, wrap_response};
use project_canens_common::{SongCardArray, SongCardProps, SongPreview};
use rspotify::model::search::SearchResult;
use rspotify::model::SearchType;
use rspotify::prelude::BaseClient;
use std::collections::HashMap;
use tap::Pipe;
use url::{Host, Url};
use vercel_runtime::{Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    handle(handle_search).await
}

async fn handle_search(req: Request) -> project_canens_backend::Result<SongCardArray> {
    let url = Url::parse(&req.uri().to_string()).expect("Received invalid url");
    println!("{url:?}");
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

    // Check for the needed parameters
    let query = hash_query
        .get("query")
        .ok_or("Missing parameter \"query\"")
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let length = hash_query
        .get("length")
        .ok_or("Missing parameter \"length\"")
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?
        .pipe_deref(str::parse)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(search(query, &spotify_client_creds(), length).await)
}

async fn search(
    query: &str,
    spotify: &rspotify::ClientCredsSpotify,
    search_length: u32,
) -> SongCardArray {
    let search_result = spotify
        .search(
            query,
            SearchType::Track,
            None,
            None,
            Some(search_length),
            None,
        )
        .await;

    let full_tracks = if let Ok(result) = search_result {
        match result {
            SearchResult::Tracks(page) => page.items,
            _ => panic!("Expected SearchResult::Tracks"),
        }
    } else {
        vec![]
    };

    full_tracks
        .into_iter()
        .filter_map(|track| track.try_into().ok())
        .map(|song_preview: SongPreview| SongCardProps { song_preview })
        .collect()
}
