use super::*;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "dotfiler", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// Files to add to dotfile path
    #[structopt(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
