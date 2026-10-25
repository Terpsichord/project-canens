use input_yew::CustomInput;

#[derive(PartialEq, Properties)]
pub struct SearchBarProps {
    pub on_search: Callback<String>,
}

#[function_component]
pub fn SearchBar(props: &SearchBarProps) -> Html {
    html! {
        <div class={classes!("search")}>
            <CustomInput name={"search"}/>
            <button onclick={props.on_search}
        </div>
    }
}