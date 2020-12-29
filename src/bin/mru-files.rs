use std::path::Path;

use anyhow::Result;
use structopt::StructOpt;

use bwataout::db::Filter;
use bwataout::SubCommand;

const BLACK_LISTED_NAMES: [&str; 2] = [".git/COMMIT_EDITMSG", "git-rebase-todo"];

fn is_blacklisted(entry: &str) -> bool {
    for name in &BLACK_LISTED_NAMES {
        if entry.ends_with(name) {
            return true;
        }
    }
    false
}

#[derive(StructOpt, Debug)]
#[structopt(name = "mru-files", about = "Manage list of edited files")]
pub struct MruFiles {
    #[structopt(subcommand)]
    pub sub_cmd: SubCommand,
}

struct MruFilter {}

impl Filter for MruFilter {
    fn parse(&self, input: &str) -> Option<String> {
        if is_blacklisted(input) {
            None
        } else {
            // No need to canoncalize the input, kakoune does it for us
            Some(input.to_string())
        }
    }

    fn should_clean(&self, value: &str) -> bool {
        let path = Path::new(value);
        !path.exists()
    }
}

fn main() -> Result<()> {
    let cmd = MruFiles::from_args();
    let filter = MruFilter {};
    let kak_script = include_str!("../mru_files.kak");
    let storage_command = bwataout::StorageCommand::new("files", kak_script, filter);
    storage_command.run(cmd.sub_cmd)
}
