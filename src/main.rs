extern crate git2;
extern crate glob;
#[macro_use]
extern crate structopt;
extern crate time;

use args::*;
use commands::*;
use git2::Repository;
use setup::new_git;
use std::env::var;
use std::process::exit;
use structopt::StructOpt;

mod args;
mod commands;
mod setup;

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

    match Repository::open(dotfile_path) {
        Ok(_) => process_files(files).unwrap(),
        Err(_) => {
            new_git();
            process_files(files).unwrap();
        }
    };
}
