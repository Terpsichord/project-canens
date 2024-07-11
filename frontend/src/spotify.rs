use std::env;
use std::rc::Rc;

use anyhow::{anyhow, bail};
use implicit_clone::ImplicitClone;
use rspotify::clients::BaseClient;
use rspotify::model::TrackId;
use rspotify::{
    ClientCredsSpotify, Config, Credentials, DEFAULT_API_BASE_URL, DEFAULT_AUTH_BASE_URL,
};

use crate::song::Song;

#[derive(Clone, ImplicitClone, Default)]
pub struct SpotifyClient {
    pub client_creds: Rc<ClientCredsSpotify>,
}

impl PartialEq for SpotifyClient {
    fn eq(&self, _other: &Self) -> bool {
        //TODO: Not sure what this value should be
        true
    }
}

#[cfg(debug_secrets)]
fn credentials() -> ClientCredsSpotify {
    ClientCredsSpotify::new(include!(concat!(env!("OUT_DIR"), "/spotify_secret.rs")))
}

const SPOTIFY_PROXY_URL: &'static str = "https://project-canens.vercel.app/api/spotify?url=";

#[cfg(not(debug_secrets))]
fn credentials() -> ClientCredsSpotify {
    ClientCredsSpotify::with_config(
        Credentials::new("", ""),
        Config {
            auth_base_url: SPOTIFY_PROXY_URL.to_owned() + DEFAULT_AUTH_BASE_URL,
            ..Default::default()
        },
    )
}

pub async fn authorize_spotify() -> anyhow::Result<SpotifyClient> {
    let client_creds = credentials();

    client_creds
        .request_token()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get Spotify access token: {:?}", e))?;

    Ok(SpotifyClient {
        client_creds: Rc::new(client_creds),
    })
}

impl SpotifyClient {
    pub async fn get_song_from_id(&self, id: &str) -> anyhow::Result<Song> {
        // TODO: add caching of song data
        let track_id = TrackId::from_id(id)?;
        let full_track = self
            .client_creds
            .track(track_id.clone(), None)
            .await
            .map_err(|e| anyhow!("Failed to get track: {}", e))?;
        let audio_features = self
            .client_creds
            .track_features(track_id)
            .await
            .map_err(|e| anyhow!("Failed to get audio features for track: {}", e))?;

        let song = Song::builder()
            .full_track(full_track)
            .audio_features(audio_features)
            .build()?;

        Ok(song)
    }
}
