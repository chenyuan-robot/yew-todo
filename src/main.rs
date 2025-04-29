use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use uuid::Uuid;

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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    Active,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TodoEntry {
    pub name: String,
    pub status: TodoStatus,
    pub id: Uuid,
}

impl TodoEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: TodoStatus::Active,
            id: Uuid::new_v4(),
        }
    }

    pub fn toggle_status(&mut self) {
        self.status = if self.status == TodoStatus::Active {
            TodoStatus::Completed
        } else {
            TodoStatus::Active
        }
    }
}

#[non_exhaustive]
#[derive(PartialEq, EnumIter, Debug)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::All => write!(f, "#/"),
            Filter::Active => write!(f, "#/active"),
            Filter::Completed => write!(f, "#/completed"),
        }
    }
}
