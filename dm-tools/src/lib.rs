pub mod cmd;
mod commands;
mod list_helpers;
mod mru_files;
mod storage;
mod storage_manager;
mod working_dirs;

use crate::cmd::SubCommand;

pub use storage_manager::StorageManager;
pub use storage_manager::StorageType;

pub fn run_storage_manager(storage_type: StorageType, cmd: SubCommand) {
    let mut storage_manager = StorageManager::new(storage_type);
    match cmd {
        SubCommand::Add { entry } => storage_manager.add(&entry),
        SubCommand::Edit {} => storage_manager.edit(),
        SubCommand::Clean {} => storage_manager.clean(),
        SubCommand::Remove { entry } => storage_manager.remove(&entry),
        SubCommand::List { kakoune } => {
            if kakoune {
                storage_manager.for_kakoune()
            } else {
                storage_manager.print_self()
            }
        }
    }
}
