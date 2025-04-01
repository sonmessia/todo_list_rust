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
        let mut storage = Storage::new("todos.json").map_err(|e| e.to_string())?;
        
        match self {
            Command::Add { description } => {
                let item = TodoItem::new(description.clone());
                storage.add(item).map_err(|e| e.to_string())?;
            }
            Command::Edit { id, description } => {
                storage.edit(*id, description.clone()).map_err(|e| e.to_string())?;
            }
            Command::Delete { id } => {
                storage.delete(*id).map_err(|e| e.to_string())?;
            }
            Command::List => {
                let items = storage.list().map_err(|e| e.to_string())?;
                for item in items {
                    println!("{}", item);
                }
            }
        }
        Ok(())
    }
}