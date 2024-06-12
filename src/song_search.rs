use crate::search_bar::SearchBar;
use crate::search_results::SearchResults;
use crate::song::Song;
use crate::song_card::{SongCard, SongCardProps};
use crate::spotify::SpotifyClient;
use itertools::Itertools;
use rspotify::model::{FullTrack, SearchResult, SearchType};
use rspotify::prelude::*;
use rspotify::ClientCredsSpotify;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_hooks::prelude::*;

async fn get_song_items(
    query: String,
    spotify: &ClientCredsSpotify,
    search_length: u32,
) -> Vec<SongCardProps> {
    let search_result = spotify
        .search(
            query.as_str(),
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
        .map(|item: FullTrack| SongCardProps {
            song: Song {
                title: item.name,
                artist: item
                    .artists
                    .into_iter()
                    .map(|artist| artist.name)
                    .join(", "),
                cover_url: item
                    .album
                    .images
                    .into_iter()
                    .next()
                    .expect_throw(&format!("{:?} has no cover art", item.href))
                    .url,
                id: item
                    .id
                    .unwrap_or_else(|| panic!("{:?} has no id", item.href))
                    .to_string(),
            },
        })
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
    let results = use_state(|| vec![]);

    let update_results = {
        let results = results.clone();
        let spotify = spotify.clone();
        let search_length = props.search_length;
        Callback::from(move |query: String| {
            let results = results.clone();
            let spotify = spotify.clone();
            spawn_local(async move {
                let songs = get_song_items(query, &spotify, search_length).await;

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
