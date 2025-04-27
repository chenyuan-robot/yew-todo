use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Student {
    name: String,
    age: i16,
}

impl Student {}
