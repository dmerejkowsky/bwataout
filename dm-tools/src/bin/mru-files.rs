use structopt::StructOpt;

fn main() {
    let cmd = dm_tools::cmd::CwdHistory::from_args();
    dm_tools::run_cache_manager(dm_tools::StorageType::MruFiles, cmd.sub_cmd)
}
