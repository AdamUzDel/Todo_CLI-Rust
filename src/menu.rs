use crate::storage::save_tasks;
use crate::task::{list_tasks, search_tasks, Task};
use std::io::{self, Write};
use colored::*; // for colorizing tasks

pub fn run_menu(tasks: &mut Vec<Task>) {
    loop {
        println!("\n=== TODO MENU ===");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Done");
        println!("4. Delete Task");
        println!("5. Search Tasks");
        println!("6. Quit");

        let choice = read_input("Enter choice: ");

        match choice.trim() {
            "1" => add_task(tasks),
            "2" => list_tasks(tasks),
            "3" => mark_task_done(tasks),
            "4" => delete_task(tasks),
            "5" => {
                let query = read_input("Enter keyword to search: ");
                search_tasks(tasks, &query);
            }
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_task(tasks: &mut Vec<Task>) {
    let desc = read_input("Enter task description: ");
    let due_input = read_input("Enter due date (YYYY-MM-DD) or leave blank: ");

    let due_date = if due_input.trim().is_empty() {
        None
    } else {
        match Task::parse_date(&due_input) {
            Ok(date) => Some(date),
            Err(_) => {
                println!("{}", "Invalid date format, ignoring due date.".red());
                None
            }
        }
    };

    tasks.push(Task::new(desc, due_date));
    save_tasks(tasks);
    println!("Task added!");
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
            save_tasks(tasks);
            println!("Task marked as done!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Please enter a valid number!");
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
            save_tasks(tasks);
            println!("Task deleted!");
        } else {
            println!("Invalid task number!");
        }
    } else {
        println!("Please enter a valid number!");
    }
}
