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
        SubCommand::Init { kakoune } => {
            if kakoune {
                storage_manager.init_kakoune()
            }
        }
        SubCommand::Add { entry } => storage_manager.add(&entry),
        SubCommand::Edit {} => storage_manager.edit(),
        SubCommand::Clean {} => storage_manager.clean(),
        SubCommand::Remove { entry } => storage_manager.remove(&entry),
        SubCommand::List { reversed } => {
            if reversed {
                storage_manager.list_reversed()
            } else {
                storage_manager.list()
            }
        }
    }
}
