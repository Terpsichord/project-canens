use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ExternalButtonProps {
    #[prop_or_default]
    pub class: Classes,
    pub href: AttrValue,
    pub img_src: AttrValue,
    pub text: AttrValue,
}

#[function_component]
pub fn ExternalButton(props: &ExternalButtonProps) -> Html {
    let ExternalButtonProps {
        class,
        href,
        img_src: src,
        text,
    } = props.clone();
    html! {
        <a {href} {class}>
            <div>
                <img {src} />
                { text }
            </div>
        </a>
    }
}
