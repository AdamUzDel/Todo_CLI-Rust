// src/main.rs
use std::io; // for user input

#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task { description, done: false }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
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
                println!("Task added!");
            }
            "2" => list_tasks(&tasks),
            "3" => mark_task_done(&mut tasks),
            "4" => {
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
