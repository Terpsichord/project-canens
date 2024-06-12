use crate::song_search::SongSearch;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeProps {
    pub search_length: u32,
}

#[function_component]
pub fn Home(props: &HomeProps) -> Html {
    html! {
        <>
            <h1>{ "Hello World!" }</h1>
            <SongSearch search_length={&props.search_length} />
        </>
    }
}
