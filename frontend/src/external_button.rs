use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ExternalButtonProps {
    pub href: AttrValue,
    pub img_src: AttrValue,
    pub text: AttrValue,
    pub accent_color: AttrValue,
    pub bg_color: AttrValue,
}

#[function_component]
pub fn ExternalButton(props: &ExternalButtonProps) -> Html {
    let ExternalButtonProps {
        href,
        img_src: src,
        text,
        accent_color,
        bg_color,
    } = props.clone();

    let style = format!(
        "color: {}; background-color: {}; border-color: {};",
        accent_color, bg_color, accent_color
    );

    html! {
        <a {href} {style} class={classes!("external-button")}>
            <div>
                <img {src} />
                <span>{ text }</span>
            </div>
        </a>
    }
}
