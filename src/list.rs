use crate::item::Item;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct ListProps {
    // pub todo_list: Vec<String>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    html! {
        <div>
            <Item />
        </div>
    }
}
