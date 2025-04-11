use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use crate::models::{TodoList, TodoItem};
use std::collections::HashMap;

const FILE_PATH: &str = "todos.json";

pub fn load_todo_list() -> TodoList {
    match File::open(FILE_PATH) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let map: HashMap<usize, TodoItem> = serde_json::from_reader(reader).unwrap_or_default();
            TodoList::from_existing(map)
        }
        Err(_) => TodoList::new(),
    }
}

pub fn save_todo_list(todo_list: &TodoList) -> std::io::Result<()> {
    let file = OpenOptions::new().
        write(true).
        create(true).
        truncate(true).
        open(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &todo_list.items)?;
    Ok(())
}