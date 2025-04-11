use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="todo")]
#[command(about="A simple todo list CLI application", long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        description: String},
    Edit {
        #[arg(short, long)]
        id: usize, description: String},
    Delete {
        #[arg(short, long)]
        id: usize},
    Toggle {
        #[arg(short, long)]
        id: usize},
    List,
}

