mod cli;
mod tasks;
use anyhow::anyhow;
use cli::{Aciton::*, CommandLineArgs};
use structopt::StructOpt;
use tasks::Task;

fn main() -> anyhow::Result<()> {
    let file_path = std::env::current_dir().map(|mut p| {
        p.push("journal.json");
        p
    });

    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();
    // println!("{:#?}", cli::CommandLineArgs::from_args());

    // Unpack the journal file
    let journal_file = journal_file
        .or_else(|| {
            if file_path.is_err() {
                None
            } else {
                Some(file_path.unwrap())
            }
        })
        .ok_or(anyhow!("Fialed to get journal file"))?;

    // Perform the action.
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        Done { position } => tasks::complete_task(journal_file, position),
        List => tasks::list_tasks(journal_file),
    }?;
    Ok(())
}
