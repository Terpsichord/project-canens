use crate::spotify::SpotifyClient;
use rspotify::clients::BaseClient;
use rspotify::model::TrackId;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SongInfoProps {
    pub id: String,
}

#[function_component]
pub fn SongInfo(props: &SongInfoProps) -> Html {
    let spotify = use_context::<SpotifyClient>()
        .expect("No spotify client provided")
        .client_creds;
    let song_handle = use_state(|| None);

    {
        let song_handle = song_handle.clone();
        let id = props.id.clone();
        spawn_local(async move {
            let mut song = spotify
                .track(
                    TrackId::from_uri(&id.clone()).unwrap_or_else(|e| panic!("Error ({}) with {id}", e)),
                    None,
                )
                .await
                .expect("Couldn't find song with that id");
            let cover = song.album.images.remove(0).url;
            song_handle.set(Some(cover));
        });
    }

    html! {
        <div>
            if let Some(cover_url) = (*song_handle).clone() {
                <img src={cover_url} />
            }
        </div>
    }
}
