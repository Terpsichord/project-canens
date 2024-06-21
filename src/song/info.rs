use std::rc::Rc;

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::error::Error;
use crate::fretmap::Fretmap;
use crate::hooktheory::Hooktheory;
use crate::spotify::SpotifyClient;
use crate::tabs::Tabs;

#[derive(PartialEq, Properties)]
pub struct SongInfoProps {
    pub id: AttrValue,
}

#[function_component]
pub fn SongInfo(props: &SongInfoProps) -> Html {
    let spotify = use_context::<SpotifyClient>().expect("No spotify client provided");

    let id = props.id.clone();
    let song_handle = use_async_with_options(
        async move { spotify.get_song_from_id(id.as_str()).await.map_err(Rc::new) },
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
                {{
                    let filtered_title = AttrValue::from(song.filtered_title);
                    let first_artist = AttrValue::from(song.artists.into_iter().next().unwrap());
                    html! {
                        <>
                            <Hooktheory song_title={filtered_title.clone()} artist={first_artist.clone()}/>
                            <Tabs song_title={filtered_title} artist={first_artist} />
                        </>
                    }}}
            } else if let Some(error) = &song_handle.error {
                <Error {error}/>
            }
        </div>
    }
}
