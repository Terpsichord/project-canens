// mod scraper;

use crate::error::Error;
use crate::external_button::ExternalButton;
use implicit_clone::unsync::IString;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(PartialEq, Properties)]
pub struct TabsProps {
    pub song_title: AttrValue,
    pub artist: AttrValue,
}

fn get_tabs_url(song_title: &str, artist: &str) -> anyhow::Result<String> {
    let params = serde_urlencoded::to_string([
        ("search_type", "title"),
        ("value", &format!("{} {}", song_title, artist)),
    ])?;
    Ok(format!(
        "https://www.ultimate-guitar.com/search.php?{}",
        params
    ))
}

#[function_component]
pub fn Tabs(props: &TabsProps) -> Html {
    let url_handle = {
        let song_title = props.song_title.clone();
        let artist = props.artist.clone();
        use_async_with_options(
            async move {
                get_tabs_url(&song_title, &artist)
                    .map(IString::from)
                    .map_err(Rc::new)
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    html! {
        <>
            if url_handle.loading {
                { "Loading tabs..." }
            } else if let Some(href) = url_handle.data.clone() {
                <ExternalButton
                    {href}
                    img_src="https://tabs.ultimate-guitar.com/static/_img/ug-logo-fb.png"
                    text="Show tabs on ultimate-guitar.com"
                    accent_color="#fec600"
                    bg_color="#111010"
                />
            } else if let Some(error) = &url_handle.error {
                <Error {error} />
            }
        </>
    }
}
