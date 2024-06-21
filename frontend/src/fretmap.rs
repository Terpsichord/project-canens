use crate::external_button::ExternalButton;
use crate::song::Key;
use yew::prelude::*;

pub fn fretmap_url_from_key(key: &Key) -> String {
    let note_name = key
        .to_string()
        .to_lowercase()
        .chars()
        .next()
        .unwrap()
        .to_string();
    let key_name = if key.note.is_accidental() {
        note_name + "-flat"
    } else {
        note_name
    };
    let mode = key.mode.to_string().to_lowercase();
    log::debug!("note_name: {mode}");

    let base = "https://fretmap.app/scales/".to_string();
    format!("{}{}-{}-scale", base, key_name, mode)
}

#[derive(PartialEq, Properties)]
pub struct FretmapProps {
    pub song_key: Key,
}

#[function_component]
pub fn Fretmap(props: &FretmapProps) -> Html {
    html! {
        <ExternalButton
            href={fretmap_url_from_key(&props.song_key)}
            img_src="https://fretmap.app/images/icons/icon-144x144.png"
            text="Show scale in fretmap.app"
            accent_color="#cc4b00"
            bg_color="#ffe2a0"
        />
    }
}
