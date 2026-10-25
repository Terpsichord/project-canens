use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AppProps {
    spotify_client_creds: ClientCredsSpotify,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <SearchBar on_search={Callback::from(spotify)}/>
        </main>
    }
}
