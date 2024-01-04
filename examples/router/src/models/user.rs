use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    id: String,
    name: String,
    age: u8,
}
impl User {
    pub(crate) fn new(id: String, name: String, age: u8) -> Self {
        Self { id, name, age }
    }
}
