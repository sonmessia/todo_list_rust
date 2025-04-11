use structopt::StructOpt;
use crate::models::TodoItem;
use crate::storage::Storage;
use colored::*;

#[derive(StructOpt, Debug)]
pub enum Command {
    Add {description: String},
    Edit {id: usize, description: String},
    Delete {id: usize},
    Toggle {id: usize},
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
                println!("{}", format!("Added todo with ID: {}", id).green());
            },
            Command::Edit { id, description } => {
                storage.edit(*id, description.clone())
                    .map_err(|e| format!("Failed to edit todo: {}", e))?;
                println!("{}", format!("Updated todo with ID: {}", id).green());
            },
            Command::Delete { id } => {
                storage.delete(*id)
                    .map_err(|e| format!("Failed to delete todo: {}", e))?;
                println!("{}", format!("Deleted todo with ID: {}", id).green());
            },
            Command::Toggle { id } => {
                storage.toggle(*id)
                    .map_err(|e| format!("Failed to toggle todo: {}", e))?;
                println!("{}", format!("Toggled todo with ID: {}", id).green()); 
            },
            Command::List => {
                let todos = storage.list().map_err(|e| e.to_string())?;
                if todos.is_empty() {
                    println!("{}", format!("No todos found").red());    
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