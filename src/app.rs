use crate::song_search::SongSearch;
use crate::spotify;
use yew::prelude::*;

// #[derive(Properties)]
// pub struct AppProps {}

#[function_component(App)]
pub fn app() -> Html {
    let search_length = 5;

    let spotify_client = spotify::authorize_spotify();

    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
            <SongSearch {spotify_client} {search_length} />
        </main>
    }
}
