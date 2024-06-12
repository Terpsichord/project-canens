use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ErrorProps {
    pub message: String,
}

#[function_component]
pub fn Error(props: &ErrorProps) -> Html {
    html! {
        <span style="color: red">
            <strong>{ "Error: " }</strong>
            { &props.message }
        </span>
    }
}
