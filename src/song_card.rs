use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct SongCardProps {
    pub title: String,
    pub artist: String,
    pub cover_url: String,
}

#[function_component]
pub fn SongCard(props: &SongCardProps) -> Html {
    html! {
        <div class="song-card">
            <img class="song-card-image" src={ props.cover_url.clone() } />
            <div class="song-card-content">
                <h4>{ &props.title }</h4>
                <p>{ &props.artist }</p>
            </div>
        </div>
    }
}
