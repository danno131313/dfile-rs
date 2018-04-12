extern crate git2;
extern crate glob;
#[macro_use]
pub extern crate structopt;
extern crate time;

use commands::{git_update, process_files, run_git};
use git2::Repository;
use setup::new_git;
use std::env::var;
use std::process::exit;
use structopt::StructOpt;

mod commands;
mod setup;

/// A program to easily hardlink dotfiles to a directory for git management and backup.
///
/// Uses your $HOME and $DOTFILE_PATH environment variables.
/// $DOTFILE_PATH should be a folder in your home directory where the hard links will be stored.
///
/// Running 'dfile' alone will attempt to add, commit, and push all changes
/// made to your dotfiles.
///
/// Can also be run as 'dfile git [COMMANDS]' to run native git commands for your
/// dotfile directory.

#[derive(StructOpt, Debug)]
#[structopt(name = "dfile", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// Files to add to dotfile path
    #[structopt(name = "FILES")]
    pub files: Vec<String>,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let files = opt.files;

    let dotfile_path = match var("DOTFILE_PATH") {
        Ok(path) => path,
        Err(_) => {
            println!("You need to set up a DOTFILE_PATH environment variable\nto use this program. Exiting...");
            exit(0);
        }
    };

    if files.len() < 1 {
        let result = git_update(&dotfile_path);
        match result {
            Ok(()) => {
                println!("Successfully updated dotfile git repo.");
                exit(0);
            }

            Err(e) => {
                println!("Error updating dotfile git repo: {}", e);
                exit(1);
            }
        }
    }

    if files[0] == "git" {
        run_git(files, &dotfile_path);
    } else {
        match Repository::open(dotfile_path) {
            Ok(_) => process_files(files).unwrap(),
            Err(_) => {
                new_git();
                process_files(files).unwrap();
            }
        };
    }
}
