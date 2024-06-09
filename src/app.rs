use crate::song_search::SongSearch;
use crate::spotify;
use yew::prelude::*;

// #[derive(Properties)]
// pub struct AppProps {}

#[function_component(App)]
pub fn app() -> Html {
    let search_length = 10;

    let spotify_client_state = use_state(|| None);

    {
        let spotify_client_state = spotify_client_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            spotify_client_state.set(Some(spotify::authorize_spotify().await));
        });
    }

    html! {
        <main>
            <h1>{ "Hello World!" }</h1>

            if let Some(spotify_client) = &*spotify_client_state {
                <SongSearch {spotify_client} {search_length} />
            } else {
                <p>{ "Spotify client loading" }</p>
            }
        </main>
    }
}
