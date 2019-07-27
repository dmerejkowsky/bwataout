use structopt::StructOpt;

use dm_tools::StorageType;

fn main() {
    let cmd = dm_tools::cmd::FilesHistory::from_args();
    dm_tools::run_storage_manager(StorageType::FilesHistory, cmd.sub_cmd)
}
