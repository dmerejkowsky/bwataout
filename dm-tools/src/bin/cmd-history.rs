use structopt::StructOpt;

use dm_tools::StorageType;

fn main() {
    let cmd = dm_tools::cmd::CwdHistory::from_args();
    dm_tools::run_storage_manager(StorageType::CommandsHistory, cmd.sub_cmd)
}
