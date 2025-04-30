use uuid::Uuid;
use yew::prelude::*;

use crate::{TodoEntry, TodoStatus};

#[derive(Debug, Properties, PartialEq)]
pub struct ItemProps {
    pub todo: TodoEntry,
    pub toggle_completed: Callback<Uuid>,
}

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    log::debug!("ITEM PROSP IS {:?}", props);
    let completed = props.todo.status == TodoStatus::Completed;
    let toggle_completed = props.toggle_completed.clone();
    let id = props.todo.id;
    log::debug!("ITEM completed IS {:?}", completed);

    let onchange = move |evt: Event, id: Uuid| {
        let input = evt.target_unchecked_into::<web_sys::HtmlInputElement>();
        log::debug!("event is, {:?}", input.checked());
        log::debug!("event id is, {:?}", id);
        toggle_completed.emit(id);
    };

    html! {
        html! {
                <li>
                    <div class="view">
                        <input class="toggle" type="checkbox" checked={completed} onchange={move |e| onchange(e, id)} />
                        <label>
                            {&props.todo.name}
                        </label>
                        <button class="destroy" />
                    </div>
            </li>
        }
    }
}
