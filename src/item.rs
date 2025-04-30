use yew::prelude::*;

use crate::TodoEntry;

#[derive(Debug, Properties, PartialEq)]
pub struct ItemProps {
    pub todo: TodoEntry,
}

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    log::debug!("ITEM PROSP IS {:?}", props);

    html! {
        <li>
            <div class="view">
                <input class="toggle" type="checkbox" />
                <label>
                    {&props.todo.name}
                </label>
                <button class="destroy" />
            </div>
        </li>
    }
}
