use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties)]
pub struct ErrorProps {
    pub error: Rc<anyhow::Error>,
}

impl PartialEq for ErrorProps {
    fn eq(&self, _other: &Self) -> bool {
        // TODO: not sure what this value should be
        false
    }
}

#[function_component]
pub fn Error(props: &ErrorProps) -> Html {
    html! {
        <span style="color: red">
            <strong>{ "Error: " }</strong>
            { &format!("{:?}", props.error) }
        </span>
    }
}
