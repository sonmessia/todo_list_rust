use structopt::StructOpt;
use crate::models::TodoItem;
use crate::storage::Storage;

#[derive(StructOpt, Debug)]
pub enum Command {
    Add {description: String},
    Edit {id: usize, description: String},
    Delete {id: usize},
    List,
}

impl Command {
    pub fn execute(&self) -> Result<(), String> {
        let storage = Storage::new("todos.json").map_err(|e| e.to_string())?;
        
        match self {
            Command::Add { description } => {
                let mut todo = TodoItem::new(description.clone());
                let id = storage.get_next_id().map_err(|e| e.to_string())?;
                todo.id = id;
                storage.add(todo).map_err(|e| e.to_string())?;
                println!("Added todo with ID: {}", id);
            },
            Command::Edit { id, description } => {
                storage.edit(*id, description.clone())
                    .map_err(|e| format!("Failed to edit todo: {}", e))?;
                println!("Updated todo with ID: {}", id);
            },
            Command::Delete { id } => {
                storage.delete(*id)
                    .map_err(|e| format!("Failed to delete todo: {}", e))?;
                println!("Deleted todo with ID: {}", id);
            },
            Command::List => {
                let todos = storage.list().map_err(|e| e.to_string())?;
                if todos.is_empty() {
                    println!("No todos found");
                } else {
                    println!("Todos:");
                    for todo in todos {
                        println!("{}", todo);
                    }
                }
            }
        }
        Ok(())
    }
}