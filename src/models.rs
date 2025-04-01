use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(description: String) -> Self {
        TodoItem {
            id: 0,
            description,
            completed: false,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { "✗" };
        write!(f, "[{}] {} (ID: {})", status, self.description, self.id)
    }
}