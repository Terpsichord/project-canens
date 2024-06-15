use std::sync::Arc;

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::error::Error;
use crate::fretmap::Fretmap;
use crate::hooktheory::Hooktheory;
use crate::spotify::SpotifyClient;

#[derive(PartialEq, Properties)]
pub struct SongInfoProps {
    pub id: String,
}

#[function_component]
pub fn SongInfo(props: &SongInfoProps) -> Html {
    let spotify = use_context::<SpotifyClient>().expect("No spotify client provided");

    let id = props.id.clone();
    let song_handle = use_async_with_options(
        async move { spotify.get_song_from_id(&id).await.map_err(Arc::new) },
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <div>
            if song_handle.loading {
                { "Loading..." }
            } else if let Some(song) = song_handle.data.clone() {
                <img src={song.cover_url} />
                <h4>{song.title}</h4>
                <p><strong>{"By: "}</strong>{song.artists.join(", ")}</p>
                <p><strong>{"Key: "}</strong>{song.key.clone()}</p>
                <p><strong>{"Tempo: "}</strong>{song.tempo}</p>
                <Fretmap song_key={song.key} />
                <Hooktheory song_title={song.filtered_title} artist={song.artists.into_iter().next().unwrap()}/>
            } else if let Some(error) = &song_handle.error {
                <Error message={error.to_string()}/>
            }
        </div>
    }
}
