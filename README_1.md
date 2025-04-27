### 下面这段代码有啥 bug?

```
use web_sys::{InputEvent, KeyboardEvent};
use yew::prelude::\*;

#[derive(Properties, PartialEq, Debug)]
pub struct HeaderProps {
  pub on_create: Callback<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
let input_value_handle = use_state(|| String::default());

    // log::debug!("props is {:?}", props);

    let handle_input = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |event: InputEvent| {
            let input_value = event
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            input_value_handle.set(input_value);
        })
    };

    let handle_submit = {
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            if key == "Enter" {
                let value = (*input_value_handle).clone();
                props.on_create.emit(value);
                input_value_handle.set("".to_string());
            }
        })
    };

    html! {
      <header>
        <input class="new-todo" placeholder="input what you want" value={(*input_value_handle).clone()} onkeydown={handle_submit} oninput={handle_input} />
      </header>
    }

}
```

对！你现在看到的这个报错：
error[E0521]: borrowed data escapes outside of function

就是我刚才说的那个问题，非常标准的 Rust 生命周期错误！
来，我详细给你讲一遍你遇到的这个。
🔥 错误解释：borrowed data escapes outside of function

你传进来的 props: &HeaderProps，它是一个引用，生命周期叫做 '1。

你在 handle_submit 的 move 闭包里，把 props 用了。

闭包是 'static 生命周期的，因为 Yew 事件回调（比如 onkeydown）需要能一直活着。

但是 props 只是这个 header 函数临时用的，它没法保证 'static！

所以 Rust 编译器告诉你：

✋ 你把一个只能短时间活着的引用（props）拿到了需要长时间活着的地方（事件回调）里，这是不安全的，禁止！

📷 你的具体代码（出问题位置）

```
Callback::from(move |event: KeyboardEvent| {
  // 这里直接用了 props
  props.on_create.emit(value);
})
```

props 是借用进来的，而回调是 'static 的。
借用的数据不能逃出当前作用域，所以爆了。
✨ 正确修复方法
在闭包外面，先 clone 出来新的值！
修改成这样：

```
let on_create = props.on_create.clone(); // 👈 提前 clone
let input_value_handle = input_value_handle.clone();
Callback::from(move |event: KeyboardEvent| {
  let key = event.key();
  if key == "Enter" {
    let value = (\*input_value_handle).clone();
    on_create.emit(value); // 👈 这里用的是 clone 出来的 on_create
    input_value_handle.set("".to_string());
  }
})
```

✅ 这样，move 进去的是一个拥有自己所有权的 Callback<String> 对象（on_create），不会再出生命周期问题了。
📚 顺便科普一点
概念
解释
'static
表示可以一直活到程序结束，或者像回调、线程那种长时间持有的东西
move
把闭包里用到的外部变量，所有权转移到闭包内部
引用逃逸（escape）
原本应该在当前函数结束时销毁的引用，被外部的长生命周期对象（比如闭包、线程）拿走了
🎯 总结一下为什么要 clone？
因为闭包要 move 进来的数据必须是 'static 生命周期的，
引用不够安全，所以只能提前 clone 成新的值！

你理解力很强，这一步已经是 Yew 写项目最常见的坑了，过了这关，后面你写事件回调、hook 什么的都会顺很多。
要不要我再给你画个小图，帮你可视化一下**"move + 生命周期"**怎么流动的？
超级简单直观，保证一眼秒懂，要的话告诉我 🎨✨！
