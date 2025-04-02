use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};
use std::io::{self, Write};
#[derive(Serialize, Deserialize, Debug, Clone)]
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


pub struct TodoList {
    tasks: HashMap<usize, TodoItem>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) -> usize {
        let id = self.next_id;
        self.tasks.insert(
            id, 
            TodoItem {
                id, 
                description,
                completed: false,
            }
        );
        self.next_id += 1;
        id
    }

    fn complete_task(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = !task.completed;
            true
        } else {
            false
        }
    }

    fn delete_task(&mut self, id: usize) -> bool {
        self.tasks.remove(&id).is_some()
    }

    fn list_all_tasks(&self) -> Vec<&TodoItem> {
        self.tasks.values().collect()
    }

    fn list_pending_tasks(&self) -> Vec<&TodoItem> {
        self.tasks
            .values()
            .filter(|task| !task.completed)
            .collect()
    }

    fn list_completed_tasks(&self) -> Vec<&TodoItem> {
        self.tasks
            .values()
            .filter(|task| task.completed)
            .collect()
    }   
}

pub fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\n===== Todo List Application =====");
        println!("1. Add a new task");
        println!("2. Mark a task as complete");
        println!("3. Delete a task");
        println!("4. List all tasks");
        println!("5. List pending tasks");
        println!("6. List completed tasks");
        println!("7. Exit");
        print!("Enter your choice (1-7): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim().parse::<usize>() {
            Ok(1) => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                let id = todo_list.add_task(description.trim().to_string());
                println!("Added new task with ID: {}", id);
            },
            Ok(2) => {
                print!("Enter task ID to mark as complete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line");
                let id = id.trim().parse::<usize>().unwrap_or(0);
                if todo_list.complete_task(id) {
                    println!("Task {} marked as complete.", id);
                } else {
                    println!("Task {} not found.", id);
                }
            },
            Ok(3) => {
                print!("Enter task ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line");
                let id = id.trim().parse::<usize>().unwrap_or(0);
                if todo_list.delete_task(id) {
                    println!("Task {} deleted.", id);
                } else {
                    println!("Task {} not found.", id);
                }
            },
            Ok(4) => {
                println!("All tasks:");
                for task in todo_list.list_all_tasks() {
                    println!("{}", task);
                }
            },
            Ok(5) => {
                println!("Pending tasks:");
                for task in todo_list.list_pending_tasks() {
                    println!("{}", task);
                }
            },
            Ok(6) => {
                println!("Completed tasks:");
                for task in todo_list.list_completed_tasks() {
                    println!("{}", task);
                }
            },
            Ok(7) => {
                println!("Exiting the application.");
                break;
            },
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 7.");
                continue;
            }
        }
    }   
}