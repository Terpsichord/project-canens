use anyhow::anyhow;
use project_canens_backend::{spotify_client_creds, wrap_response};
use project_canens_common::Song;
use rspotify::model::TrackId;
use rspotify::prelude::BaseClient;
use rspotify::ClientCredsSpotify;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[allow(unused)]
#[tokio::main]
pub async fn main(req: Request) -> Result<Response<Body>, Error> {
    wrap_response(handle_song(req))
}

async fn handle_song(req: Request) -> project_canens_backend::Result<Song> {
    let url = Url::parse(&req.uri().to_string())?;
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let id = hash_query
        .get("id")
        .ok_or("Missing parameter \"id\"")
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(get_song_from_id(&spotify_client_creds(), id).await)
}
pub async fn get_song_from_id(spotify: &ClientCredsSpotify, id: &str) -> anyhow::Result<Song> {
    // TODO: add caching of song data

    let track_id = TrackId::from_id(id)?;
    let full_track = spotify
        .track(track_id.clone(), None)
        .await
        .map_err(|e| anyhow!("Failed to get track: {}", e))?;
    let audio_features = spotify
        .track_features(track_id)
        .await
        .map_err(|e| anyhow!("Failed to get audio features for track: {}", e))?;

    let song = Song::builder()
        .full_track(full_track)
        .audio_features(audio_features)
        .build()?;

    Ok(song)
}
