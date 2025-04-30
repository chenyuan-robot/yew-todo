use uuid::Uuid;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{platform::spawn_local, prelude::*};

use crate::{TodoEntry, TodoStatus};

#[derive(Debug, Properties, PartialEq)]
pub struct ItemProps {
    pub todo: TodoEntry,
    pub clear_todo: Callback<Uuid>,
    pub toggle_completed: Callback<Uuid>,
    pub rename_todo: Callback<(String, Uuid)>,
}

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    let editing = use_state(|| false);
    let input_node_ref = use_node_ref();

    let completed = props.todo.status == TodoStatus::Completed;
    let toggle_completed = props.toggle_completed.clone();
    let rename_todo = props.rename_todo.clone();
    let clear_todo = props.clear_todo.clone();
    let id = props.todo.id;

    let onchange = move |evt: Event, id: Uuid| {
        let input = evt.target_unchecked_into::<web_sys::HtmlInputElement>();
        log::debug!("event is, {:?}", input.checked());
        log::debug!("event id is, {:?}", id);
        toggle_completed.emit(id);
    };

    let handle_edit = {
        let editing = editing.clone();
        Callback::from(move |_| {
            editing.set(true);
            spawn_local(async move {
                gloo::timers::future::TimeoutFuture::new(3000).await;
                // log::debug!("NEW VALUE IS IS {:?}", value); // 三秒后执行
            });
            // log::debug!("continue");
        })
    };

    let handle_blur = {
        let input_node_ref = input_node_ref.clone();
        let clear_todo = clear_todo.clone();
        let editing = editing.clone();
        move |id: Uuid| {
            let input_value = input_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let value = input_value.trim().to_string();
            if value.is_empty() {
                clear_todo.emit(id.clone());
            } else {
                rename_todo.emit((value, id.clone()))
            }
            editing.set(false);
        }
    };

    let handle_submit = {
        let input_node_ref = input_node_ref.clone();
        let clear_todo = clear_todo.clone();
        let editing = editing.clone();
        let rename_todo = props.rename_todo.clone();
        move |e: KeyboardEvent, id: Uuid| {
            let key = e.key();
            match key.as_str() {
                "Enter" => {
                    let input_value = input_node_ref.cast::<HtmlInputElement>().unwrap().value();
                    let value = input_value.trim().to_string();
                    if value.is_empty() {
                        clear_todo.emit(id.clone());
                    } else {
                        rename_todo.emit((value, id.clone()))
                    }
                    editing.set(false);
                }
                "Escape" => {
                    editing.set(false);
                }
                _ => {}
            }
        }
    };

    use_effect_with(editing.clone(), {
        let editing = editing.clone();
        let input_node_ref = input_node_ref.clone();
        move |_| {
            log::debug!("EDIT VALUE IS {:?}", editing);
            if *editing {
                let input = input_node_ref.cast::<HtmlInputElement>();
                let value = input_node_ref.cast::<HtmlInputElement>().unwrap().value();
                input.to_owned().unwrap().focus().unwrap();
                log::debug!("ON NODE IS {:?}", value);
                // input.unwrap().set_value(value.as_str());
            }
        }
    });

    html! {
        <li class={classes!(
            completed.then(|| Some("completed")),
        )}>
            <div class="view">
                <input class="toggle" type="checkbox" checked={completed} onchange={move |e| onchange(e, id)} />
                <label ondblclick={handle_edit}>
                    {&props.todo.name}
                </label>
                <button class="destroy" onclick={move |_|clear_todo.emit(id)} />
            </div>
            {
                if *editing {
                    html! {
                        <input ref={input_node_ref} onkeyup={move |e| handle_submit(e, id)} onblur={move |_| handle_blur(id)} />
                    }
                }
                else {
                    html! {}
                }
            }
        </li>
    }
}
