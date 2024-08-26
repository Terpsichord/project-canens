use implicit_clone::unsync::IArray;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::backend::BackendClient;
use crate::search::results::SearchResults;
use crate::search::search_bar::SearchBar;
use crate::song::card::SongCard;

mod results;
mod search_bar;

#[derive(PartialEq, Properties)]
pub struct SongSearchProps {
    pub search_length: u32,
}

#[function_component]
pub fn SongSearch(props: &SongSearchProps) -> Html {
    let backend = use_context::<BackendClient>().expect("No backend client provided");

    #[allow(clippy::redundant_closure)]
    let results = use_state(|| IArray::EMPTY);

    let update_results = {
        let results = results.clone();
        let backend = backend.clone();
        let search_length = props.search_length;
        Callback::from(move |query: String| {
            let results = results.clone();
            let backend = backend.clone();
            spawn_local(async move {
                let songs = backend
                    .get_json(
                        "/spotify/search",
                        &[("query", query), ("length", search_length.to_string())],
                    )
                    .await
                    .expect("Failed to get search results");

                results.set(songs);
            });
        })
    };

    let focused = use_state(|| false);

    let on_focus = {
        let focused = focused.clone();
        Callback::from(move |_| focused.set(true))
    };

    let node = use_node_ref();

    use_click_away(node.clone(), {
        let focused = focused.clone();
        move |_| {
            focused.set(false);
        }
    });

    html! {
        <div ref={node} class="search-container">
            <SearchBar on_search={update_results} {on_focus} />
            if *focused {
                <SearchResults<SongCard> search_items={(*results).clone()}/>
            }
        </div>
    }
}
