use crate::app::Route;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(PartialEq, Properties)]
pub struct NavbarProps {}

#[function_component]
pub fn Navbar(_props: &NavbarProps) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });

    html! {
        <div class="navbar">
            <button {onclick}>
                <h4>{ "Home" }</h4>
            </button>
        </div>
    }
}
