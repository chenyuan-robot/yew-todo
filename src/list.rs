use std::rc::Rc;

use crate::{item::Item, TodoEntry};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct ListProps {
    pub filter_list: Rc<Vec<TodoEntry>>,
    pub toggle_completed: Callback<Uuid>,
    pub clear_todo: Callback<Uuid>,
    pub toggle_complete_all: Callback<bool>,
    pub rename_todo: Callback<(String, Uuid)>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let ListProps {
        filter_list,
        clear_todo,
        toggle_completed,
        toggle_complete_all,
        rename_todo,
    } = props;

    let onchange = {
        let toggle_complete_all = toggle_complete_all.clone();
        move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            toggle_complete_all.emit(input.checked())
        }
    };

    html! {
        <section class="main">
            <input
                id="toggle-all" class="toggle-all" type="checkbox"
                readonly=true
                onchange={onchange}
            />
            <label for="toggle-all" />

            <ul class="todo-list">
                {
                    for filter_list.iter().map(|todo| {
                        let id = todo.id;
                        html! {
                            <Item {rename_todo} key={id.to_string()} todo={todo.clone()} {toggle_completed} {clear_todo}  />
                        }
                    })
                }
            </ul>
        </section>
    }
}
