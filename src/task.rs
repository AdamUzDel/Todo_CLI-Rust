use chrono::{NaiveDate, ParseError}; // for adding due dates
use serde::{Deserialize, Serialize};  // for writing tasks to json file
use colored::*; // for colorizing tasks

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub description: String,
    pub done: bool,
    pub due_date: Option<NaiveDate>, // Optional due date
}

impl Task {
    pub fn new(description: String, due_date: Option<NaiveDate>) -> Self {
        Task { description, done: false, due_date }
    }
}

impl Task {
    pub fn parse_date(input: &str) -> Result<NaiveDate, ParseError> {
        NaiveDate::parse_from_str(input, "%Y-%m-%d")
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", "No tasks found!".red());
        return;
    }

    println!("{}", "=== TASKS ===".cyan().bold());

    // Sort by due date first, then done status
    let mut sorted = tasks.clone();
    sorted.sort_by(|a, b| {
        match (&a.due_date, &b.due_date) {
            (Some(d1), Some(d2)) => d1.cmp(d2),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    for (i, task) in tasks.iter().enumerate() {
        let index = format!("{}. ", i + 1).blue();
        let desc = if task.done {
            task.description.green().strikethrough()
        } else {
            task.description.yellow()
        };

        let due = if let Some(date) = task.due_date {
            format!(" (due {})", date).magenta().to_string()
        } else {
            "".to_string()
        };

        println!("{}{}{}", index, desc, due);
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
