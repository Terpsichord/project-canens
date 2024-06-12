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

#[cfg(debug_assertions)]
fn credentials() -> Credentials {
    include!(concat!(env!("OUT_DIR"), "/env.rs"))
}

#[cfg(not(debug_assertions))]
fn credentials() -> Credentials {
    panic!("Can't currently get Spotify credentials in release mode");
}

pub fn authorize_spotify() -> SpotifyClient {
    let client_creds = ClientCredsSpotify::new(credentials());

    wasm_bindgen_futures::spawn_local({
        let client_creds = client_creds.clone();
        async move {
            client_creds
                .request_token()
                .await
                .expect("Couldn't get Spotify access token");
        }
    });

    SpotifyClient {
        client_creds: Rc::new(client_creds),
    }
}
