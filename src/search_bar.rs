use std::ops::Deref;
use wasm_bindgen_futures::wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchBarProps {
    pub on_search: Callback<String>,
}

#[function_component]
pub fn SearchBar(props: &SearchBarProps) -> Html {
    let input_value = use_state(|| "".to_string());

    let on_submit = {
        let input_value = input_value.clone();
        let on_search = props.on_search.clone();
        Callback::from(move |event: SubmitEvent| {
            // Stops the page from reloading on submit
            event.prevent_default();
            on_search.emit(input_value.deref().clone())
        })
    };

    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap_throw()
                .dyn_into::<HtmlInputElement>()
                .unwrap_throw()
                .value();
            input_value.set(value);
        })
    };

    html! {
        <div class="search-container">
            <form onsubmit={on_submit}>
                <input onchange={on_input_change} type="text" placeholder="Search for song..." name="search"/>
                <button type="submit">{ "Search" }</button>
            </form>
        </div>
    }
}
