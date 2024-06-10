use crate::search_bar::SearchBar;
use crate::search_results::SearchResults;
use crate::song_card::{SongCard, SongCardProps};
use crate::spotify::SpotifyClient;
use itertools::Itertools;
use rspotify::model::{FullTrack, SearchResult, SearchType};
use rspotify::prelude::*;
use rspotify::ClientCredsSpotify;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

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
        })
        .collect()
}

#[derive(PartialEq, Properties)]
pub struct SongSearchProps {
    pub spotify_client: SpotifyClient,
    pub search_length: u32,
}

#[function_component]
pub fn SongSearch(props: &SongSearchProps) -> Html {
    let spotify = &props.spotify_client.client_creds;

    #[allow(clippy::redundant_closure)]
    let results = use_state(|| vec![]);

    let update_results = {
        let results = results.clone();
        let spotify = spotify.clone();
        let search_length = props.search_length;
        Callback::from(move |query: String| {
            let results = results.clone();
            let spotify = spotify.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let songs = get_song_items(query, &spotify, search_length).await;

                results.set(songs);
            });
        })
    };

    html! {
        <div>
            <SearchBar on_search={update_results}/>
            <SearchResults<SongCard> search_items={(*results).clone()}/>
        </div>
    }
}
