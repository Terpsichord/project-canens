use crate::app::Route;
use crate::song::SongPreview;
use anyhow::Context;
use itertools::Itertools;
use yew::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct SongCardProps {
    pub song_preview: SongPreview,
}

#[function_component]
pub fn SongCard(SongCardProps { song_preview }: &SongCardProps) -> Html {
    let navigator = use_navigator().unwrap();

    let id = song_preview.id.clone();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Song { id: id.clone() });
    });

    html! {
        <button {onclick} class="song-card">
            <img class="song-card-image" src={ song_preview.cover_url.clone() } />
            <div class="song-card-content">
                <h4>{ &song_preview.title }</h4>
                <p>{ &song_preview.artists.join(", ") }</p>
            </div>
        </button>
    }
}
