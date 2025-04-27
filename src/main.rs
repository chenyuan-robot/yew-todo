mod app;
mod header;
mod tests;

fn main() {
    // 启用panic打印到console
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("log init failed");
    yew::Renderer::<app::App>::new().render();
}
