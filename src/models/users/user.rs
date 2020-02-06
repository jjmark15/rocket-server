use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<u32>,
    pub name: String,
}

impl User {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
