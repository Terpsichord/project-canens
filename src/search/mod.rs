use crate::search::results::SearchResults;
use crate::search::search_bar::SearchBar;
use implicit_clone::unsync::IArray;

use rspotify::model::{SearchResult, SearchType};
use rspotify::prelude::*;
use rspotify::ClientCredsSpotify;

use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::song::card::{SongCard, SongCardProps};
use crate::song::SongPreview;
use crate::spotify::SpotifyClient;

mod results;
mod search_bar;

async fn get_song_items(
    query: &str,
    spotify: &ClientCredsSpotify,
    search_length: u32,
) -> IArray<SongCardProps> {
    let search_result = spotify
        .search(
            query,
            SearchType::Track,
            None,
            None,
            Some(search_length),
            None,
        )
        .await;

    let full_tracks = if let Ok(result) = search_result {
        match result {
            SearchResult::Tracks(page) => page.items,
            _ => panic!("Expected SearchResult::Tracks"),
        }
    } else {
        vec![]
    };

    full_tracks
        .into_iter()
        .filter_map(|track| track.try_into().ok())
        .map(|song_preview: SongPreview| SongCardProps { song_preview })
        .collect()
}

#[derive(PartialEq, Properties)]
pub struct SongSearchProps {
    pub search_length: u32,
}

#[function_component]
pub fn SongSearch(props: &SongSearchProps) -> Html {
    let spotify = use_context::<SpotifyClient>()
        .expect("No spotify client provided")
        .client_creds;

    #[allow(clippy::redundant_closure)]
    let results = use_state(|| IArray::EMPTY);

    let update_results = {
        let results = results.clone();
        let spotify = spotify.clone();
        let search_length = props.search_length;
        Callback::from(move |query: String| {
            let results = results.clone();
            let spotify = spotify.clone();
            spawn_local(async move {
                let songs = get_song_items(&query, &spotify, search_length).await;

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
