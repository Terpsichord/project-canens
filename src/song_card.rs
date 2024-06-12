use crate::app::Route;
use crate::song::Song;
use yew::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct SongCardProps {
    pub song: Song,
}

#[function_component]
pub fn SongCard(SongCardProps { song }: &SongCardProps) -> Html {
    let navigator = use_navigator().unwrap();

    let id = song.id.clone();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Song { id: id.clone() });
    });

    html! {
        <button {onclick} class="song-card">
            <img class="song-card-image" src={ song.cover_url.clone() } />
            <div class="song-card-content">
                <h4>{ &song.title }</h4>
                <p>{ &song.artist }</p>
            </div>
        </button>
    }
}
