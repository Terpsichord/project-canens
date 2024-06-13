use crate::error::Error;
use crate::home::Home;
use crate::song::info::SongInfo;
use crate::spotify;
use crate::spotify::SpotifyClient;
use std::sync::Arc;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
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
    match route {
        Route::Home => html! { <Home {search_length} /> },
        Route::Song { id } => html! { <SongInfo {id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let spotify_handle = use_async_with_options(
        async move { spotify::authorize_spotify().await.map_err(Arc::new) },
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <main>
            if spotify_handle.loading {
                { "Loading..." }
            } else if let Some(error) = &spotify_handle.error {
                <Error message={error.to_string()}/>
            } else if let Some(spotify_client) = &spotify_handle.data {
                <BrowserRouter>
                    <ContextProvider<SpotifyClient> context={spotify_client}>
                        <Switch<Route> render={switch} />
                    </ContextProvider<SpotifyClient>>
                </BrowserRouter>
            }
        </main>
    }
}
