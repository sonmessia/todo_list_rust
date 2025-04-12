use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: usize, description: String) -> Self {
        TodoItem {
            id,
            description,
            completed: false,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub items: HashMap<usize, TodoItem>,
    #[serde(skip)]
    pub next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn from_existing(items: HashMap<usize, TodoItem>) -> Self {
        let max_id = items.keys().copied().max().unwrap_or(0);
        TodoList { items, next_id: max_id + 1, }
    }

    pub fn add_item(&mut self, description: String) {
        let item = TodoItem::new(self.next_id, description);
        self.items.insert(self.next_id, item);
        self.next_id += 1;
    }

    pub fn edit_item(&mut self, id: usize, description: String) -> Result<(), String> {
        if let Some(item) = self.items.get_mut(&id) {
            item.description = description;
            Ok(())
        } else {
            Err(format!("Item with ID {} not found", id))
        }
    }
    pub fn delete_item(&mut self, id: usize) -> Result<(), String> {
        if self.items.remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Item with ID {} not found", id))
        }
    }

    pub fn toggle_item(&mut self, id: usize) -> Result<(), String> {
        if let Some(item) = self.items.get_mut(&id) {
            item.toggle();
            Ok(())
        } else {
            Err(format!("Item with ID {} not found", id))
        }
    }
    pub fn list_items(&self) {
        let mut list: Vec<&TodoItem> = self.items.values().collect();
        list.sort_by_key(|item| item.id);
        for item in list {
            let status = if item.completed { "✓" } else { "✗" };
            println!("{}: {} [{}]", item.id, item.description, status);
        }
    }

    pub fn list_items_completed(&self) {
        let mut list: Vec<&TodoItem> = self.items.values().filter(|item| item.completed).collect();
        list.sort_by_key(|item| item.id);
        for item in list {
            println!("{}: {}", item.id, item.description);
        }
    }
    pub fn list_items_not_completed(&self) {
        let mut list: Vec<&TodoItem> = self.items.values().filter(|item| !item.completed).collect();
        list.sort_by_key(|item| item.id);
        for item in list {
            println!("{}: {}", item.id, item.description);
        }
    }
    pub fn list_items_by_id(&self, id: usize) {
        if let Some(item) = self.items.get(&id) {
            println!("{}: {}", item.id, item.description);
        } else {
            println!("Item with ID {} not found", id);
        }
    }
}