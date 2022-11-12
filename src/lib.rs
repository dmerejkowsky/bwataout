use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;

pub mod db;
pub use db::DB;

use directories::ProjectDirs;

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(about = "dump kakoune initial script")]
    InitKakoune,

    #[clap(about = "add a new entry")]
    Add { entry: String },

    #[clap(about = "remove an entry")]
    Remove { entry: String },

    #[clap(about = "clean invalid entries")]
    Clean {
        #[clap(long = "max", long_help = "only clean the last <max> entries")]
        max: Option<isize>,
    },

    #[clap(about = "list entries")]
    List {
        #[clap(long = "reversed", long_help = "reverse order")]
        reversed: bool,
    },
}

pub fn get_app_dir() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("info", "dmerej", "bwataout")
        .ok_or_else(|| anyhow!("Could not get project dirs"))?;
    let data_dir = project_dirs.data_dir();
    std::fs::create_dir_all(data_dir).map_err(|e| anyhow!("Could not create data dir: {e}"))?;
    Ok(data_dir.to_path_buf())
}

pub struct StorageCommand<T>
where
    T: crate::db::Filter,
{
    name: &'static str,
    filter: T,
    kak_script: &'static str,
}

impl<T> StorageCommand<T>
where
    T: crate::db::Filter,
{
    pub fn new(name: &'static str, kak_script: &'static str, filter: T) -> Self {
        Self {
            name,
            filter,
            kak_script,
        }
    }
    pub fn run(self, cmd: SubCommand) -> Result<()> {
        let app_dir = crate::get_app_dir()?;
        let name = &self.name;
        let db_path = format!("{name}.db");
        let mut db = DB::new(&app_dir.join(db_path), self.filter)?;

        match cmd {
            SubCommand::InitKakoune => {
                let kak_script = &self.kak_script;
                println!("{}", kak_script)
            }
            SubCommand::List { reversed } => {
                let list = if reversed {
                    db.list_reversed()
                } else {
                    db.list()
                }?;
                for v in list {
                    println!("{v}");
                }
            }
            SubCommand::Add { entry } => {
                db.add(&entry)?;
            }
            SubCommand::Remove { entry } => {
                db.remove(&entry)?;
            }
            SubCommand::Clean { max } => db.clean(max)?,
        }
        Ok(())
    }
}
