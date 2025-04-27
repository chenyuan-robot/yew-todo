use web_sys::{InputEvent, KeyboardEvent};
use yew::prelude::*;

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
        let on_create = props.on_create.clone();
        let input_value_handle = input_value_handle.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key = event.key();
            if key == "Enter" {
                let value = (*input_value_handle).clone();
                let value_1 = input_value_handle.as_str();
                let value_2 = (*input_value_handle).to_owned();
                log::debug!("value is, {:?}", value);
                log::debug!("value_1 is, {:?}", value_1);
                log::debug!("value_2 is, {:?}", value_2);
                on_create.emit(value);
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
