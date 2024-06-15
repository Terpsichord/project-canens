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
    pub song_title: String,
    pub artist: String,
}

#[function_component]
pub fn Hooktheory(props: &HooktheoryProps) -> Html {
    let url = hooktheory_url_from_song(&props.song_title, &props.artist);
    html! {
        <a href={url.clone()} class="hooktheory">
            <div>
                <img src={"https://www.hooktheory.com/images/logos/hooktheory-logo-2021.svg"} />
                { "Show TheoryTab on Hooktheory" }
            </div>
        </a>
    }
}
