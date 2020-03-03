use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cwd-history", about = "Manage working directories history")]
pub struct CwdHistory {
    #[structopt(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "mru-files", about = "Manage list of edited files")]
pub struct FilesHistory {
    #[structopt(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cmd-history", about = "Manage commands history")]
pub struct CommandsHistory {
    #[structopt(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(name = "add", about = "add a new entry")]
    Add { entry: String },

    #[structopt(name = "remove", about = "remove an entry")]
    Remove { entry: String },

    #[structopt(name = "clean", about = "clean entriess")]
    Clean {},

    #[structopt(name = "edit", about = "edit database")]
    Edit {},

    #[structopt(name = "list", about = "list entries")]
    List {
        #[structopt(
            long = "--kakoune",
            help = "use output suitable for kakoune integration"
        )]
        kakoune: bool,
    },
}
