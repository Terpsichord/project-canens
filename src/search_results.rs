use yew::prelude::*;

#[derive(Properties)]
pub struct SearchResultsProps<T: BaseComponent>
where
    T::Properties: PartialEq + Clone,
{
    pub search_items: Vec<T::Properties>,
}

impl<T: BaseComponent> PartialEq for SearchResultsProps<T>
where
    T::Properties: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.search_items == other.search_items
    }
}

#[function_component]
pub fn SearchResults<T: BaseComponent>(props: &SearchResultsProps<T>) -> Html
where
    T::Properties: PartialEq + Clone,
{
    html! {
        <ul>
            {
                for props.search_items.iter().map(|item_props| html_nested! {
                    <T ..item_props.clone() />
                })
            }
        </ul>
    }
}
