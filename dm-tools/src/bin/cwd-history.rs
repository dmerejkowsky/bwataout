use structopt::StructOpt;

fn main() {
    let cmd = dm_tools::cmd::CwdHistory::from_args();
    dm_tools::run_cmd(dm_tools::CmdType::CwdHistory, cmd.sub_cmd)
}


