//! # dotfiler
//! A program to easily hardlink dotfiles to a directory for git management and backup.
//! Uses your $HOME and $DOTFILE_PATH environment variables.

#[macro_use]
extern crate structopt;

use args::*;
use std::env::current_dir;
use std::env::var;
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;
use std::fs::create_dir_all;

mod args;


fn main() {
    let opt: Opt = Opt::from_args();
    let files = opt.files;

    process_files(files).unwrap();
}

/// Hard links each file provided to your dotfile directory, minus any dot prefixes.
/// Will structure the folders the same way relative to your home directory.
fn process_files(files: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let home: PathBuf = var("HOME").expect("No $HOME variable set!").into();
    let dotfile_path: PathBuf = var("DOTFILE_PATH")
        .expect("No $DOTFILE_PATH variable set!")
        .into();
    let curr_dir: PathBuf = current_dir().unwrap();

    for file in files.into_iter() {
        let name = file.to_str().unwrap();
        let fullpath = curr_dir.join(&file);

        let newpath: PathBuf = get_dest(&fullpath, &home, &dotfile_path);

        // Create new directories in dotfile path if they don't exist
        let newpath_clone = newpath.clone();
        let dirs_only = newpath_clone.parent().unwrap();
        create_dir_all(dirs_only).unwrap();

        // Create hard links
        let output = Command::new("ln")
            .arg(fullpath)
            .arg(newpath)
            .output()
            .expect("Couldn't make static link!");

        if output.status.success() {
            println!("{} has been successfully hard-linked to dotfiles directory.", name);
        } else {
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    Ok(())
}

/// Gets the correct path for your dotfiles to be stored.
/// Removes dots, so $HOME/.config/vimrc is hardlinked to $HOME/dotfile_path/config/vimrc,
/// for example.
fn get_dest(fullpath: &PathBuf, home: &PathBuf, dotfile_path: &PathBuf) -> PathBuf {
    let home_path_size: usize = home.iter().count();

    // Gets path to existing dotfile relative to home dir
    let relative_path: PathBuf = fullpath
        .clone()
        .into_iter()
        .skip(home_path_size)
        .collect();

    // Path of new hard link, with dots
    let newpath: PathBuf = dotfile_path.join(&relative_path);

    // Remove dots
    let mut dotless_newpath = PathBuf::new();
    for part in newpath.components() {
        let part_path: &Path = part.as_ref();
        let mut part_str = part_path.to_str().unwrap();
        if part_str.chars().next() == Some('.') {
            part_str = &part_str[1..];
        }
        dotless_newpath.push(part_str);
    }

    return dotless_newpath;
}
