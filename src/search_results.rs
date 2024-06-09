use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchResultsProps<T: PartialEq> {
    pub search_items: Vec<T>,
}

#[function_component]
pub fn SearchResults<T>(props: &SearchResultsProps<T>) -> Html
where
    T: PartialEq,
    for<'a> &'a T: Into<Html>,
{
    html! {
        <ul>
            { for props.search_items.iter() }
        </ul>
    }
}
