use std::rc::Rc;

use crate::backend::BackendClient;
use project_canens_common::Song;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

use crate::error::Error;
use crate::fretmap::Fretmap;
use crate::hooktheory::Hooktheory;
use crate::tabs::Tabs;

#[derive(PartialEq, Properties)]
pub struct SongPageProps {
    pub id: AttrValue,
}

#[function_component]
pub fn SongPage(props: &SongPageProps) -> Html {
    let backend = use_context::<BackendClient>().expect("No backend client provided");

    let id = props.id.clone();
    let song_handle: UseAsyncHandle<Song, _> = use_async_with_options(
        async move {
            backend
                .get_json("/spotify/song", &[("id", id)])
                .await
                .map_err(Rc::new)
        },
        UseAsyncOptions::enable_auto(),
    );

    html! {
        <div class="song-page">
            if song_handle.loading {
                { "Loading..." }
            } else if let Some(song) = song_handle.data.clone() {
                <img src={song.cover_url} />
                <div class="song-page-contents">
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
                </div>
            } else if let Some(error) = &song_handle.error {
                <Error {error}/>
            }
        </div>
    }
}
