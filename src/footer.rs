use std::any::type_name;

use crate::Filter;
use strum::IntoEnumIterator;
use yew::prelude::*;

fn print_type_of<T>(_: T) {
    log::debug!("Type is: {}", type_name::<T>());
}

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub selected_filter: Filter,
    pub on_filterchange: Callback<Filter>,
    pub clear_completed: Callback<()>,
    pub todos_left: u32,
    pub todos_completed: u32,
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let clear_completed = props.clear_completed.clone();
    let on_filter_change = |filter: Filter| {
        let on_filterchange = props.on_filterchange.clone();
        Callback::from(move |_| {
            on_filterchange.emit(filter);
        })
    };

    let clear_complete_todo = Callback::from(move |_| {
        clear_completed.emit(());
    });

    let items_text = match props.todos_left {
        1 => "item",
        _ => "items",
    };

    html! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{props.todos_left}</strong>
                <span>{format!(" {} left", items_text)}</span>
            </span>
            <ul class="filters">
                {
                    // --  写法一 推荐
                    // for Filter::iter().map(|status| {
                    //     html! {
                    //         // <li>{status.to_string()}</li> -> #/ ...
                    //         <li>{format!("{:?}", status)}</li> // Filter 派生了 #[derive(Debug)]，可以这样写
                    //     }
                    // })

                    // --  写法二 带索引
                    // for Filter::iter().enumerate().map(|(index, status)| {
                    //     log::debug!("result is {:?} - {}", status, index);
                    //     html! {
                    //         // <li>{status.to_string()}</li> -> #/ ...
                    //         <li key={index}>{format!("{:?}", status)}</li> // Filter 派生了 #[derive(Debug)]，可以这样写
                    //     }
                    // })

                    // --  写法三
                    Filter::iter().enumerate().map(|(index, filter)| {
                        html! {
                            <li key={index} onclick={on_filter_change(filter)}>
                                // <a class={classes!("empty", (filter == props.selected_filter).then_some("selected"))}>
                                <a class={classes!("test-class", (filter == props.selected_filter).then(|| Some("selected")))}>
                                    { format!("{:?}", filter) }
                                </a>
                            </li>
                        }
                    }).collect::<Vec<Html>>()
                }
            </ul>
            {
                if props.todos_completed > 0 {
                    html! {
                        <button class="clear-completed" onclick={clear_complete_todo}>
                            {"Clear completed"}
                        </button>
                    }
                } else {
                    html! {}
                }
            }
        </footer>
    }
}
