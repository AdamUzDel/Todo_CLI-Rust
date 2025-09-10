use serde::{Deserialize, Serialize};  // for writing tasks to json file
use colored::*; // for colorizing tasks

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task { description, done: false }
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", "No tasks found!".red());
        return;
    }

    println!("{}", "=== TASKS ===".cyan().bold());
    for (i, task) in tasks.iter().enumerate() {
        let index = format!("{}. ", i + 1).blue();
        let desc = if task.done {
            task.description.green().strikethrough()
        } else {
            task.description.yellow()
        };
        println!("{}{}", index, desc);
    }
}

pub fn search_tasks(tasks: &Vec<Task>, keyword: &str) {
    let keyword_lower = keyword.to_lowercase();
    let filtered: Vec<(usize, &Task)> = tasks
        .iter()
        .enumerate()
        .filter(|(_, t)| t.description.to_lowercase().contains(&keyword_lower))
        .collect();

    if filtered.is_empty() {
        println!("{}", "No matching tasks found.".red());
    } else {
        println!("{}", "=== SEARCH RESULTS ===".cyan().bold());
        for (i, task) in filtered {
            let index = format!("{}. ", i + 1).blue();
            let desc = if task.done {
                task.description.green().strikethrough()
            } else {
                task.description.yellow()
            };
            println!("{}{}", index, desc);
        }
    }
}
