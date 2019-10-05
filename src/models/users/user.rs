use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
}


impl User {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}