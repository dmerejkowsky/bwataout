use std::path::Path;

use anyhow::Result;
use clap::Parser;

use bwataout::db::Filter;
use bwataout::SubCommand;

#[derive(Parser, Debug)]
#[clap(name = "cwd-history", about = "Manage list of visited directories")]
pub struct CWDHistory {
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

struct WorkingDirsFilter {}

impl Filter for WorkingDirsFilter {
    fn parse(&self, input: &str) -> Option<String> {
        let path = Path::new(input);
        if !path.exists() {
            return None;
        }
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap().join(path)
        };
        let clean_path = abs_path
            .canonicalize()
            .unwrap_or_else(|_| panic!("Could not canonicalize {}", input));
        let entry = clean_path
            .to_str()
            .unwrap_or_else(|| panic!("Could not convert {} to string", clean_path.display()));
        Some(entry.to_string())
    }

    fn should_clean(&self, value: &str) -> bool {
        let path = Path::new(value);
        !path.exists()
    }
}

fn main() -> Result<()> {
    let cmd = CWDHistory::parse();
    let filter = WorkingDirsFilter {};
    let kak_script = include_str!("../working_dirs.kak");
    let storage_command = bwataout::StorageCommand::new("working-dirs", kak_script, filter);
    storage_command.run(cmd.sub_cmd)
}
