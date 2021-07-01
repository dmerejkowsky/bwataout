use std::path::PathBuf;

use anyhow::{Context, Result};
use structopt::StructOpt;

pub mod db;
pub use db::DB;

use app_dirs::{AppDataType, AppInfo};

const APP_INFO: AppInfo = AppInfo {
    name: "bwataout",
    author: "Dimitri Merejkowsky",
};

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(name = "init-kakoune", about = "dump kakoune initial script")]
    InitKakoune,

    #[structopt(name = "add", about = "add a new entry")]
    Add { entry: String },

    #[structopt(name = "remove", about = "remove an entry")]
    Remove { entry: String },

    #[structopt(name = "clean", about = "clean entriess")]
    Clean {
        #[structopt(long = "--max", help = "only clean the last <max> entries")]
        max: Option<isize>,
    },

    #[structopt(name = "list", about = "list entries")]
    List {
        #[structopt(long = "--reversed", help = "reverse order")]
        reversed: bool,
    },
}
pub fn get_app_dir() -> Result<PathBuf> {
    app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "")
        .with_context(|| "Could not create app dir")
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
        let db_path = format!("{}.db", self.name);
        let mut db = DB::new(&app_dir.join(db_path), self.filter)?;

        match cmd {
            SubCommand::InitKakoune => println!("{}", &self.kak_script),
            SubCommand::List { reversed } => {
                let list = if reversed {
                    db.list_reversed()
                } else {
                    db.list()
                }?;
                for v in list {
                    println!("{}", v);
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
