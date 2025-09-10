// src/main.rs
use std::io; // for user input
use serde::{Deserialize, Serialize};  // for writing tasks to json file
use std::fs::{self, File};
use std::io::Write;
use clap::{Parser, Subcommand};  // for commandline parsing

#[derive(Parser)]
#[command(name = "todo_cli", version = "1.0", about = "A simple todo CLI in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as done
    Done {
        index: usize,
    },
    /// Delete a task
    Delete {
        index: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task { description, done: false }
    }
}

const FILE_PATH: &str = "tasks.json";

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks(); // load existing tasks

    match cli.command {
        Some(Commands::Add { description }) => {
            tasks.push(Task::new(description));
            save_tasks(&tasks);
            println!("Task added!");
        }
        Some(Commands::List) => {
            list_tasks(&tasks);
        }
        Some(Commands::Done { index }) => {
            if index > 0 && index <= tasks.len() {
                tasks[index - 1].done = true;
                save_tasks(&tasks);
                println!("Task marked as done!");
            } else {
                println!("Invalid task number!");
            }
        }
        Some(Commands::Delete { index }) => {
            if index > 0 && index <= tasks.len() {
                tasks.remove(index - 1);
                save_tasks(&tasks);
                println!("Task deleted!");
            } else {
                println!("Invalid task number!");
            }
        }
        None => {
            // If no args given, show interactive menu
            run_menu(&mut tasks);
        }
    }

    /* loop {
        println!("\n=== TODO MENU ===");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Done");
        println!("4. Quit");

        let choice = read_input("Enter choice: ");

        match choice.trim() {
            "1" => {
                let desc = read_input("Enter task description: ");
                tasks.push(Task::new(desc));
                save_tasks(&tasks);
                println!("Task added!");
            }
            "2" => list_tasks(&tasks),
            "3" => {
                mark_task_done(&mut tasks);
                save_tasks(&tasks);
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    } */
}

fn run_menu(tasks: &mut Vec<Task>) {
    loop {
        println!("\n=== TODO MENU ===");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Done");
        println!("4. Delete Task");
        println!("5. Quit");

        let choice = read_input("Enter choice: ");

        match choice.trim() {
            "1" => {
                let desc = read_input("Enter task description: ");
                tasks.push(Task::new(desc));
                save_tasks(&tasks);
                println!("Task added!");
            }
            "2" => list_tasks(tasks),
            "3" => {
                mark_task_done(tasks);
                save_tasks(&tasks);
            }
            "4" => {
                delete_task(tasks);
                save_tasks(&tasks);
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}


fn read_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks yet!");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done { "✔" } else { " " };
        println!("{}. [{}] {}", i + 1, status, task.description);
    }
}

fn mark_task_done(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to mark!");
        return;
    }

    list_tasks(tasks);
    let idx_str = read_input("Enter task number to mark done: ");
    if let Ok(idx) = idx_str.trim().parse::<usize>() {
        if idx > 0 && idx <= tasks.len() {
            tasks[idx - 1].done = true;
            println!("Task marked as done!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Please enter a valid number!");
    }
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        let mut file = File::create(FILE_PATH).expect("Could not create file");
        file.write_all(json.as_bytes()).expect("Could not write file");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to delete!");
        return;
    }

    list_tasks(tasks);
    let idx_str = read_input("Enter task number to delete: ");
    if let Ok(idx) = idx_str.trim().parse::<usize>() {
        if idx > 0 && idx <= tasks.len() {
            tasks.remove(idx - 1);
            println!("Task deleted!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Please enter a valid number!");
    }
}
