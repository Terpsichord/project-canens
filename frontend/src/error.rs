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
    eprintln!("{}", props.error);
    html! {
        <div style="white-space: pre-line; color: red; background-color: white">
            <strong>{ "Error: " }</strong>
            { &format!("{:#}", props.error) }
        </div>
    }
}
