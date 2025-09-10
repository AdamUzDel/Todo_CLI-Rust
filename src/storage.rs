use crate::task::Task;
use std::fs;
use std::path::Path;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if Path::new(FILE).exists() {
        let data = fs::read_to_string(FILE).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Unable to serialize tasks");
    fs::write(FILE, data).expect("Unable to write file");
}
