use dm_tools::StorageType;
use structopt::StructOpt;

fn main() {
    let cmd = dm_tools::cmd::CwdHistory::from_args();
    dm_tools::run_storage_manager(StorageType::CwdHistory, cmd.sub_cmd)
}
