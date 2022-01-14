use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Aciton {
    /// Write tasks to the journal file
    Add {
        /// The task decription text
        #[structopt()]
        text: String,
    },
    /// Remove an entry from  the journal file by position
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file
    List,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Journal", about = "A command line to-do app written in Rust")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Aciton,

    /// Use a different journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
