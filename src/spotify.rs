use std::rc::Rc;

use anyhow::{anyhow, bail};
use implicit_clone::ImplicitClone;
use rspotify::clients::BaseClient;
use rspotify::model::TrackId;
use rspotify::{ClientCredsSpotify, Credentials};

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
fn credentials() -> anyhow::Result<Credentials> {
    Ok(include!(concat!(env!("OUT_DIR"), "/env.rs")))
}

#[cfg(not(debug_secrets))]
fn credentials() -> anyhow::Result<Credentials> {
    bail!("Can't currently get Spotify credentials in release mode");
}

pub async fn authorize_spotify() -> anyhow::Result<SpotifyClient> {
    let client_creds = ClientCredsSpotify::new(credentials()?);

    client_creds
        .request_token()
        .await
        .map_err(|e| anyhow::anyhow!("Couldn't get Spotify access token: {:?}", e))?;

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
