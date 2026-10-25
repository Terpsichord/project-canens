mod app;
mod search;
mod spotify;

use app::App;

fn main() {
    let spotify_client_creds = spotify::authorize_spotify();
    yew::Renderer::<App>::with_props(AppProps { spotify_client_creds }).render();
}
