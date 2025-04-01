use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use crate::models::TodoItem;
use serde_json::{self, Value};


pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> io::Result<Self> {
        Ok(Storage {
            file_path: file_path.to_string(),
        })
    }

    pub fn add(&self, item: TodoItem) -> io::Result<()> {
        let mut items = self.load_items()?;
        items.push(item);
        self.save_items(&items)
    }

    pub fn edit(&self, id: usize, description: String) -> io::Result<()> {
        let mut items = self.load_items()?;
        if let Some(item) = items.get_mut(id) {
            item.description = description;
            self.save_items(&items)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Item not found"))
        }
    }

    pub fn delete(&self, id: usize) -> io::Result<()> {
        let mut items = self.load_items()?;
        if id < items.len() {
            items.remove(id);
            self.save_items(&items)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Item not found"))
        }
    }
    pub fn list(&self) -> io::Result<Vec<TodoItem>> {
        self.load_items()
    }
    fn load_items(&self) -> io::Result<Vec<TodoItem>> {
        let file = File::open(&self.file_path)?;
        let reader = io::BufReader::new(file);
        let items: Vec<TodoItem> = serde_json::from_reader(reader)?;
        Ok(items)
    }
    fn save_items(&self, items: &[TodoItem]) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer(writer, items)?;
        Ok(())
    }
}