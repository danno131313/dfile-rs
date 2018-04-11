use super::*;
use std::path::PathBuf;

/// A program to easily hardlink dotfiles to a directory for git management and backup.
///
/// Uses your $HOME and $DOTFILE_PATH environment variables.
/// $DOTFILE_PATH should be a folder in your home directory where the hard links will be stored.

#[derive(StructOpt, Debug)]
#[structopt(name = "dfile", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// Files to add to dotfile path
    #[structopt(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
