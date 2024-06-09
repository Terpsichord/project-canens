use implicit_clone::ImplicitClone;
use rspotify::{ClientCredsSpotify, Credentials};
use std::rc::Rc;

#[derive(Clone, ImplicitClone)]
pub struct SpotifyClient {
    pub client_creds: Rc<ClientCredsSpotify>,
}

impl PartialEq for SpotifyClient {
    fn eq(&self, _other: &Self) -> bool {
        //TODO: Not sure what this value should be
        true
    }
}

pub async fn authorize_spotify() -> SpotifyClient {
    let creds = Credentials::new(
        "[**REDACTED RSPOTIFY_CLIENT_ID**]",
        "[**REDACTED RSPOTIFY_CLIENT_SECRET**]",
    ); //::from_env().expect("Couldn't load Spotify credentials from environment");
    let client_creds = ClientCredsSpotify::new(creds);

    client_creds
        .request_token()
        .await
        .expect("Couldn't get Spotify access token");

    SpotifyClient {
        client_creds: Rc::new(client_creds),
    }
}
