use crate::external_button::ExternalButton;
use deunicode::deunicode;
use yew::prelude::*;

fn filter_string(string: &str) -> String {
    deunicode(string)
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}

fn hooktheory_url_from_song(song_title: &str, artist: &str) -> String {
    let base = "https://www.hooktheory.com/theorytab/view/";
    format!(
        "{}{}/{}",
        base,
        filter_string(artist),
        filter_string(song_title)
    )
}

#[derive(PartialEq, Properties)]
pub struct HooktheoryProps {
    pub song_title: AttrValue,
    pub artist: AttrValue,
}

#[function_component]
pub fn Hooktheory(props: &HooktheoryProps) -> Html {
    html! {
        <ExternalButton
            class="hooktheory"
            href={hooktheory_url_from_song(props.song_title.as_str(), props.artist.as_str())}
            img_src="https://www.hooktheory.com/images/logos/hooktheory-logo-2021.svg"
            text="Show TheoryTab on Hooktheory"
        />
    }
}
