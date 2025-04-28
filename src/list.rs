use crate::item::Item;
use yew::prelude::*;

#[function_component(List)]
pub fn list() -> Html {
    html! {
        <div>
            <Item />
        </div>
    }
}
