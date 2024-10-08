use project_canens_common::SongCardProps;
use yew::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component]
pub fn SongCard(SongCardProps { song_preview }: &SongCardProps) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        let route = Route::Song {
            id: song_preview.id.clone(),
        };
        Callback::from(move |_| {
            navigator.push(&route);
        })
    };

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
