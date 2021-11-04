use anyhow::Result;
use clap::Parser;

use bwataout::db::Filter;
use bwataout::SubCommand;

#[derive(Parser, Debug)]
#[clap(name = "cmd-history", about = "Manage list of commands")]
pub struct CommandsHistory {
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

struct CommandsFilter {}

impl Filter for CommandsFilter {
    fn parse(&self, input: &str) -> Option<String> {
        if input.starts_with(" ") {
            return None;
        }
        Some(input.to_string())
    }

    fn should_clean(&self, _value: &str) -> bool {
        false
    }
}

fn main() -> Result<()> {
    let cmd = CommandsHistory::parse();
    let filter = CommandsFilter {};
    let kak_script = "";
    let storage_command = bwataout::StorageCommand::new("commands", kak_script, filter);
    storage_command.run(cmd.sub_cmd)
}
