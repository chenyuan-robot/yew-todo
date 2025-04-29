use std::any::type_name;
use strum::IntoEnumIterator;
use yew::prelude::*;

fn print_type_of<T>(_: T) {
    log::debug!("Type is: {}", type_name::<T>());
}

use crate::Filter;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <span class="todo-count">
                <strong>{"strong"}</strong>
                <span>{"temp"}</span>
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
                            <li key={index}>{format!("{:?}", filter)}</li>
                        }
                    }).collect::<Vec<Html>>()
                }
            </ul>
        </footer>
    }
}
