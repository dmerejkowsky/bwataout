mod app;
mod storage;
pub mod cmd;

use crate::cmd::SubCommand;

pub enum CmdType {
    CwdHistory,
    MruFiles,
}

pub fn run_cmd(cmd_type: CmdType, cmd: SubCommand) {
    let name = match cmd_type {
        CmdType::CwdHistory => "cwd-history",
        CmdType::MruFiles => "mru-files",
    };
    let command = match cmd_type {
        CmdType::CwdHistory => "change-directory",
        CmdType::MruFiles => "edit -existing",
    };
    let mut app = crate::app::App::new(name);
    match cmd {
        SubCommand::Add{ entry } => app.add(&entry),
        SubCommand::Edit {} => app.edit(),
        SubCommand::List{ kakoune } => {
            if kakoune {
                app.for_kakoune(command)
            } else {
                app.print_self()
            }
        },
    }

}
