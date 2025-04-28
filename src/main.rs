mod app;
mod footer;
mod header;
mod item;
mod list;
mod tests;

fn main() {
    // 启用panic打印到console
    tests::tests();
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("log init failed");
    yew::Renderer::<app::App>::new().render();
}
