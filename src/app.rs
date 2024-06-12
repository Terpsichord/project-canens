use crate::home::Home;
use crate::song_info::SongInfo;
use crate::spotify;
use crate::spotify::SpotifyClient;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/song/:id")]
    Song { id: String },
}

fn switch(route: Route) -> Html {
    let search_length = 5;
    log::debug!("Switching");
    match route {
        Route::Home => html! { <Home {search_length} /> },
        Route::Song { id } => html! { <SongInfo {id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let spotify_client = spotify::authorize_spotify();

    html! {
        <main>
            <BrowserRouter>
                <ContextProvider<SpotifyClient> context={spotify_client}>
                    <Switch<Route> render={switch} />
                </ContextProvider<SpotifyClient>>
            </BrowserRouter>
        </main>
    }
}
