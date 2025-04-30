use crate::header::Header;
use crate::list::{self, List};
use crate::{footer::Footer, TodoEntry};
use crate::{Filter, TodoStatus};
use gloo::storage::{LocalStorage, Storage};
use web_sys::window;
use yew::prelude::*;

const KEY: &'static str = "yew.todomvc.self";

#[function_component(App)]
pub fn app() -> Html {
    let filter = use_state(|| Filter::All);
    let todo_list = use_state(|| vec![TodoEntry::new(String::default())]);

    let filter_list = use_memo((filter.clone(), todo_list.clone()), {
        let filter = filter.clone();
        let todo_list = todo_list.clone();
        move |_| match *filter {
            Filter::All => todo_list.to_vec(),
            Filter::Active => {
                let result = todo_list
                    .to_vec()
                    .iter()
                    .filter(|x| x.status == TodoStatus::Active)
                    .cloned()
                    .collect::<Vec<TodoEntry>>();
                result
            }
            Filter::Completed => {
                let result = todo_list
                    .to_vec()
                    .iter()
                    .filter(|x| x.status == TodoStatus::Completed)
                    .cloned()
                    .collect::<Vec<TodoEntry>>();
                result
            }
        }
    });

    let on_filterchange: Callback<Filter> = {
        let filter = filter.clone();
        Callback::from(move |params| {
            log::debug!("PARAMS {:?}", params);
            filter.set(params);
        })
    };

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
            // region: my section
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
            // endregion
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

    // 方式一
    let calc_todo_left = || {
        let mut acc: u32 = 0;
        let v1 = todo_list.clone();
        for item in v1.iter() {
            if item.status == TodoStatus::Active {
                acc += 1;
            }
        }
        acc
    };

    // 方式二
    let calc_active_todo_list = || {
        todo_list.iter().fold(0u32, |acc, todo| {
            if todo.status == TodoStatus::Active {
                acc + 1
            } else {
                acc
            }
        })
    };

    let toggle_completed = {
        let todo_list = todo_list.clone();
        Callback::from(move |id| {
            // 方式零： 推荐
            todo_list.set({
                let mut new_list = todo_list.to_vec();
                for list in &mut new_list {
                    if list.id == id {
                        list.toggle_status();
                    }
                }
                new_list
            });

            // 方式一：
            // let mut new_list = todo_list.to_vec();
            // let find = new_list.iter_mut().find(|todo| todo.id == id);
            // if let Some(find_item) = find {
            //     find_item.toggle_status();
            //     todo_list.set(new_list);
            // } else {
            //     window()
            //         .unwrap()
            //         .alert_with_message("some error occur")
            //         .unwrap();
            // }

            // 方式二： 每次手动 clone()
            // let new_list = todo_list
            //     .iter()
            //     .map(|todo| {
            //         if todo.id == id {
            //             TodoEntry {
            //                 status: if todo.status == TodoStatus::Active {
            //                     TodoStatus::Completed
            //                 } else {
            //                     TodoStatus::Active
            //                 },
            //                 ..todo.clone()
            //             }
            //         } else {
            //             TodoEntry { ..todo.clone() }
            //         }
            //     })
            //     .collect::<Vec<_>>();
            // log::debug!("{:?}", new_list);

            // 方式二： 直接迭代器上.cloned()
            // let new_list_clone = todo_list
            //     .iter()
            //     .cloned()
            //     .map(|todo| {
            //         if todo.id == id {
            //             TodoEntry {
            //                 status: if todo.status == TodoStatus::Active {
            //                     TodoStatus::Completed
            //                 } else {
            //                     TodoStatus::Active
            //                 },
            //                 ..todo
            //             }
            //         } else {
            //             TodoEntry { ..todo }
            //         }
            //     })
            //     .collect::<Vec<_>>();
            // todo_list.set(new_list_clone);
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
                    let todos_left = calc_todo_left();
                    html! {
                        <>
                            <List {filter_list} {toggle_completed} />
                            <Footer todos_left={todos_left} selected_filter={*filter} {on_filterchange} todos_completed={todo_list.len() as u32 - todos_left} />
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
