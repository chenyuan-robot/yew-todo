use crate::header::Header;
use web_sys::window;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // let todo_list = use_state(|| vec![String::default()]);
    let todo_list = use_state(|| vec!["".to_string()]);

    let on_create = {
        let todo_list = todo_list.clone();
        Callback::from(move |name: String| {
            if name.is_empty() {
                window()
                    .unwrap()
                    .alert_with_message("输入内容不能为空")
                    .unwrap();
            }
        })
    };

    html! {
        <div id="app">
            <section class="todoapp"></section>
            <Header on_create={on_create} />
        </div>
    }
}
