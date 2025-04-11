use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use crate::models::TodoItem;
use serde_json;


pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> io::Result<Self> {
        // Create the file if it doesn't exist
        if !std::path::Path::new(file_path).exists() {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_path)?;
            
            let writer = io::BufWriter::new(file);
            serde_json::to_writer(writer, &Vec::<TodoItem>::new())?;
        }
        
        Ok(Storage {
            file_path: file_path.to_string(),
        })
    }

    pub fn get_next_id(&self) -> io::Result<usize> {
        let items = self.load_items()?;
        if items.is_empty() {
            return Ok(1);
        }
        
        let max_id = items.iter()
            .map(|item| item.id)
            .max()
            .unwrap_or(0);
            
        Ok(max_id + 1)
    }

    pub fn add(&self, mut item: TodoItem) -> io::Result<()> {
        let mut items = self.load_items()?;
        
        // Set the ID if it's not already set
        if item.id == 0 {
            item.id = self.get_next_id()?;
        }
        
        items.push(item);
        self.save_items(&items)
    }

    pub fn edit(&self, id: usize, description: String) -> io::Result<()> {
        let mut items = self.load_items()?;
        
        let item_pos = items.iter().position(|item| item.id == id);
        
        if let Some(pos) = item_pos {
            items[pos].description = description;
            self.save_items(&items)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, format!("Item with ID {} not found", id)))
        }
    }

    pub fn delete(&self, id: usize) -> io::Result<()> {
        let mut items = self.load_items()?;
        
        let item_pos = items.iter().position(|item| item.id == id);
        
        if let Some(pos) = item_pos {
            items.remove(pos);
            self.save_items(&items)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, format!("Item with ID {} not found", id)))
        }
    }

    pub fn list(&self) -> io::Result<Vec<TodoItem>> {
        self.load_items()
    }

    pub fn get(&self, id: usize) -> io::Result<Option<TodoItem>> {
        let items = self.load_items()?;
        Ok(items.iter().find(|item| item.id == id).cloned())
    }

    fn load_items(&self) -> io::Result<Vec<TodoItem>> {
        match File::open(&self.file_path) {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(items) => Ok(items),
                    Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
                }
            },
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(e)
        }
    }
    
    fn save_items(&self, items: &[TodoItem]) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, items)?;
        Ok(())
    }

    pub fn toggle(&self, id: usize) -> io::Result<()> {
        let mut items = self.load_items()?;
        
        if let Some(item) = items.iter_mut().find(|item| item.id == id) {
            item.completed = !item.completed;
            self.save_items(&items)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, format!("Item with ID {} not found", id)))
        }
    }
}