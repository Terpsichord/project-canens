use rspotify::prelude::*;
use rspotify::Credentials;

fn authorize_spotify() -> ClientCredsSpotify {
    let creds = Credentials::from_env().expect("Couldn't load Spotify credentials from environment");
    let spotify = ClientCredsSpotify::new(creds);

    spotify.request_token().expect("Couldn't get Spotify access token");
}