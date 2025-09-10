mod task;
mod storage;
mod menu;

use clap::{Parser, Subcommand};
use storage::{load_tasks, save_tasks};
use task::{list_tasks, search_tasks, Task};
use colored::*;

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple Rust-based TODO app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String, #[arg(short, long)] due: Option<String> },
    List,
    Done { index: usize },
    Delete { index: usize },
    Search { keyword: String },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Some(Commands::Add { description, due }) => {
            let due_date = match due {
                Some(d) => match Task::parse_date(&d) {
                    Ok(date) => Some(date),
                    Err(_) => {
                        println!("{}", "Invalid date format, ignoring due date.".red());
                        None
                    }
                },
                None => None,
            };

            tasks.push(Task::new(description, due_date));
            save_tasks(&tasks);
            println!("Task added!");
        }
        Some(Commands::List) => list_tasks(&tasks),
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
        Some(Commands::Search { keyword }) => search_tasks(&tasks, &keyword),
        None => menu::run_menu(&mut tasks),
    }
}
