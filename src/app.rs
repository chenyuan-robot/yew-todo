use crate::header::Header;
use crate::list::List;
use crate::Filter;
use crate::{footer::Footer, TodoEntry};
use gloo::storage::{LocalStorage, Storage};
use web_sys::window;
use yew::prelude::*;

const KEY: &'static str = "yew.todomvc.self";

#[function_component(App)]
pub fn app() -> Html {
    let filter = use_state(|| Filter::All);
    let todo_list = use_state(|| vec![TodoEntry::new(String::default())]);

    let filter_result = use_memo(filter, move |_| format!("the result is {}", "hello"));
    log::debug!("{:?}", filter_result);

    let on_create = {
        let todo_list = todo_list.to_owned();
        Callback::from(move |name: String| {
            if name.is_empty() {
                window()
                    .unwrap()
                    .alert_with_message("输入内容不能为空")
                    .unwrap();
                return;
            }
            let is_exist = todo_list.iter().any(|x| x.name == name);
            if is_exist {
                window().unwrap().alert_with_message("名称已存在").unwrap();
                return;
            }
            // 方式一：
            // todo_list.set({
            //     let mut todo_list = (*todo_list).clone();
            //     todo_list.push(name);
            //     todo_list
            // });

            // 方式二：
            // todo_list.set(
            //     todo_list
            //         .iter()
            //         .cloned() // 每一项 clone
            //         .chain(std::iter::once(name)) // 加上输入框的内容
            //         .collect(),
            // );

            // 方式三：
            let new_todo_list = todo_list
                .iter()
                .cloned()
                .chain([TodoEntry::new(name)].into_iter()) // [name] 是一个数组 .into_iter() 把它变成一个迭代器。 效果跟 once(name) 是一样的
                .collect::<Vec<TodoEntry>>();

            todo_list.set(new_todo_list.to_owned());

            LocalStorage::set(KEY, new_todo_list.to_vec())
                .expect("save to localstorage occur error");
        })
    };

    use_effect_with((), {
        let todo_list = todo_list.clone();
        move |_| {
            let todo_list_storage: Vec<TodoEntry> =
                LocalStorage::get(KEY).unwrap_or_else(|_| vec![]);
            log::debug!("todo list is {:?}", todo_list_storage);
            todo_list.set(todo_list_storage);
            || {}
        }
    });

    html! {
        <div id="app">
            <section class="todoapp" />
            <Header on_create={on_create} />
            {
                if todo_list.len() > 0 {
                    html! {
                        <>
                            <List />
                            <Footer />
                        </>
                    }
                } else {
                    html! {
                        <div>{"none"}</div>
                    }
                }
            }
        </div>
    }
}
