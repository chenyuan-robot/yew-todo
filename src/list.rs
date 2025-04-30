use std::rc::Rc;

use crate::{item::Item, TodoEntry};
use uuid::Uuid;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct ListProps {
    pub filter_list: Rc<Vec<TodoEntry>>,
    pub toggle_completed: Callback<Uuid>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    let ListProps {
        filter_list,
        toggle_completed,
    } = props;

    html! {
        <ul class="todo-list">
        {
            for filter_list.iter().map(|todo| {
                let id = todo.id;
                html! {
                        <Item key={id.to_string()} todo={todo.clone()} {toggle_completed}  />
                    }
                })
            }
        </ul>
    }
}
