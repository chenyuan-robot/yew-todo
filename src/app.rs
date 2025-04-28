use crate::header::Header;
use web_sys::window;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // let todo_list = use_state(|| vec![String::default()]);
    let todo_list = use_state(|| vec![]);

    let on_create = {
        let todo_list = todo_list.clone();
        Callback::from(move |name: String| {
            if name.is_empty() {
                window()
                    .unwrap()
                    .alert_with_message("输入内容不能为空")
                    .unwrap();
                return;
            }
            let is_exist = todo_list.contains(&name);
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
            todo_list.set(
                todo_list
                    .iter()
                    .cloned()
                    .chain([name].into_iter()) // [name] 是一个数组 .into_iter() 把它变成一个迭代器。 效果跟 once(name) 是一样的
                    .collect(),
            );
        })
    };

    html! {
        <div id="app">
            <section class="todoapp"></section>
            <Header on_create={on_create} />
            {
                if todo_list.len() > 0 {
                    html! {
                        <div>{"nihao"}</div>
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
