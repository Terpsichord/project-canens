use crate::error::Result;
use anyhow::Context;
use axum::extract::{Query, State};
use axum::Json;
use project_canens_common::{Song, SongCardArray, SongCardProps, SongPreview};
use rspotify::clients::BaseClient;
use rspotify::model::{SearchResult, SearchType, TrackId};
use rspotify::ClientCredsSpotify;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    query: String,
    length: u32,
}

#[tracing::instrument(name = "spotify_search")]
pub async fn search(
    State(spotify): State<ClientCredsSpotify>,
    Query(search_query): Query<SearchQuery>,
) -> Result<Json<SongCardArray>> {
    let SearchQuery { query, length } = search_query;

    let search_result = spotify
        .search(&query, SearchType::Track, None, None, Some(length), None)
        .await?;

    let full_tracks = match search_result {
        SearchResult::Tracks(page) => page.items,
        _ => panic!("Expected SearchResult::Tracks"),
    };

    tracing::debug!("fetched tracks from spotify: {:?}", full_tracks);

    Ok(full_tracks
        .into_iter()
        .filter_map(|track| track.try_into().ok())
        .map(|song_preview: SongPreview| SongCardProps { song_preview })
        .collect::<SongCardArray>()
        .into())
}

#[derive(Debug, Deserialize)]
pub struct SongQuery {
    id: String,
}

#[tracing::instrument(name = "spotify_song")]
pub async fn song(
    State(spotify): State<ClientCredsSpotify>,
    Query(song_query): Query<SongQuery>,
) -> Result<Json<Song>> {
    // TODO: add caching of song data

    let track_id = TrackId::from_id(&song_query.id)?;
    let full_track = spotify
        .track(track_id.clone(), None)
        .await
        .context("Failed to get track")?;
    let audio_features = spotify
        .track_features(track_id)
        .await
        .context("Failed to get audio features for track")?;

    let song = Song::builder()
        .full_track(full_track)
        .audio_features(audio_features)
        .build()?;

    Ok(song.into())
}
