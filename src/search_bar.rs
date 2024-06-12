use wasm_bindgen_futures::wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchBarProps {
    pub on_search: Callback<String>,
    pub on_focus: Callback<()>,
}

#[function_component]
pub fn SearchBar(props: &SearchBarProps) -> Html {
    let onfocus = {
        let on_focus = props.on_focus.clone();
        Callback::from(move |_| {
            on_focus.emit(());
        })
    };

    let oninput = {
        let on_search = props.on_search.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap_throw()
                .dyn_into::<HtmlInputElement>()
                .unwrap_throw()
                .value();
            log::debug!("{}", value);
            on_search.emit(value);
        })
    };

    html! {
        <input {onfocus} {oninput} type="text" placeholder="Search for song..." name="search"/>
    }
}
