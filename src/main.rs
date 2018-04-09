extern crate git2;
#[macro_use]
extern crate structopt;

use args::*;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use std::process::exit;
use git2::Repository;
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

    if files.len() < 1 {
        println!("No files provided! Exiting..");
        exit(0);
    }

    let dotfile_path = match var("DOTFILE_PATH") {
        Ok(path) => path,
        Err(_) => setup(),
    };

    match Repository::open(dotfile_path) {
        Ok(_) => process_files(files).unwrap(),
        Err(_) => {
            new_git();
            process_files(files).unwrap();
        }
    };
}

/// Hard links each file provided to your dotfile directory, minus any dot prefixes.
/// Will structure the folders the same way relative to your home directory.
fn process_files(files: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let home: PathBuf = var("HOME").expect("No $HOME variable set!").into();
    let dotfile_path: PathBuf = var("DOTFILE_PATH").unwrap().into();
    let curr_dir: PathBuf = current_dir().unwrap();

    for file in files.into_iter() {
        let name = file.to_str().unwrap();
        let fullpath = curr_dir.join(&file);

        let newpath: PathBuf = get_dest(&fullpath, &home, &dotfile_path);

        // Create new directories in dotfile path if they don't exist
        let newpath_clone = newpath.clone();
        let dirs_only = newpath_clone.parent().unwrap();
        create_dir_all(dirs_only)?;

        // Create hard links
        let output = Command::new("ln")
            .arg(fullpath)
            .arg(newpath)
            .output()
            .expect("Couldn't make static link!");

        if output.status.success() {
            println!(
                "{} has been successfully hard-linked to dotfiles directory.",
                name
            );
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
    let relative_path: PathBuf = fullpath.clone().into_iter().skip(home_path_size).collect();

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

/// Creates a new git repo at the DOTFILE_PATH directory
fn new_git() {
    print!("No git repo found at DOTFILE_PATH, would you like to create one? (y/n) ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();
    if input.as_str() == "y" {
        Repository::init(var("DOTFILE_PATH").unwrap())
            .expect("Couldn't create a new git repo with DOTFILE_PATH");
    } else {
        exit(0);
    }
}

/// Sets up the DOTFILE_PATH environment variable by appending it to your bash_rc or zshrc
fn setup() -> String {
    println!("You haven't set up a DOTFILE_PATH environment variable yet!");
    print!("Would you like to? (y/n) ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();

    if &input != "y" {
        println!("Cannot add files without DOTFILE_PATH, exiting...");
        exit(0);
    } else {
        print!("Do you use bash or zsh? ");
        io::stdout().flush().unwrap();
        let mut shell = String::new();
        io::stdin().read_line(&mut shell).unwrap();
        shell.pop();

        print!("What would you like the new dotfile git path to be? ");
        io::stdout().flush().unwrap();
        let mut path = String::new();
        io::stdin().read_line(&mut path).unwrap();
        path.pop();

        let mut rcfile = var("HOME").unwrap();
        match shell.as_str() {
            "bash" => rcfile.push_str("/.bashrc"),
            "zsh" => rcfile.push_str("/.zshrc"),
            _x => {
                println!("{} is not a supported shell!", _x);
                exit(1);
            }
        }

        let mut file = OpenOptions::new()
            .append(true)
            .open(rcfile)
            .unwrap();

        writeln!(file, "export DOTFILE_PATH={}", path).unwrap();

        Command::new(format!("export DOTFILE_PATH={}", path)).spawn().unwrap();

        create_dir_all(&path).unwrap_or_else(|_| {
            println!("Couldn't create DOTFILE_DIR with that path! Exiting...");
            exit(1);
        });

        return path;
    }
}
