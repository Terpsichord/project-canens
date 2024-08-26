use crate::backend::BackendClient;
use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

use crate::home::Home;
use crate::navbar::Navbar;
use crate::song::page::SongPage;

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
        Route::Song { id } => html! { <SongPage {id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let backend = BackendClient::new();
    html! {
        <BrowserRouter>
            <Navbar />
            <main>
                <ContextProvider<BackendClient> context={backend}>
                    <Switch<Route> render={switch} />
                </ContextProvider<BackendClient>>
            </main>
        </BrowserRouter>
    }
}
