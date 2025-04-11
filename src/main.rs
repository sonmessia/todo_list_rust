use clap::Parser;
mod models;
mod storage;
mod cli;

use cli::{Cli, Commands};
use storage::{load_todo_list, save_todo_list};

fn main() {
    let cli = Cli::parse();
    let mut todo_list = load_todo_list();
    match cli.command {
        Commands::Add { description } => {
            todo_list.add_item(description);
            save_todo_list(&todo_list).expect("Failed to save todo list");
        },
        Commands::List => {
            todo_list.list_items();
        },
        Commands::Toggle { id } => {
            todo_list.toggle_item(id).expect("Failed to toggle item");
            save_todo_list(&todo_list).expect("Failed to save todo list");
        },
        Commands::Edit { id, description } => {
            todo_list.edit_item(id, description).expect("Failed to edit item");
            save_todo_list(&todo_list).expect("Failed to save todo list");
        },
        Commands::Delete { id } => {
            todo_list.delete_item(id).expect("Failed to delete item");
            save_todo_list(&todo_list).expect("Failed to save todo list");
        },
    }
}