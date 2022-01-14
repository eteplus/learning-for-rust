use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub done: bool,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            created_at: Utc::now(),
            done: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let done = if self.done { "✅" } else { "⬜️" };
        write!(
            f,
            "{} {:<20} {}",
            done.to_string(),
            self.text,
            self.created_at
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S"),
        )
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;
    // Write the modified task list back into the file
    tasks.push(task);
    serde_json::to_writer(&file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, position: usize) -> Result<()> {
    // Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    // Consume them file's contents as a vector of tasks.
    let mut tasks = collect_tasks(&file)?;

    // Complete the task
    if position == 0 || position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }

    tasks[position - 1].done = true;
    // tasks.remove(position - 1);

    // Rewind and truncate the file
    file.set_len(0)?;

    // Write the modified task list back into the file
    serde_json::to_writer(&file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).open(journal_path)?;

    // Consume them file's contents as a vector of tasks.
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty");
    } else {
        // Print the tasks
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.

    // Consume them file's contents as a vector of tasks.
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}
