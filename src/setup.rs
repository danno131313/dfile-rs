use std::io;
use std::io::Write;
use git2::Repository;
use std::env::var;
use std::process::{exit, Command};

/// Creates a new git repo at the DOTFILE_PATH directory
pub fn new_git() {
    let input = prompt("No git repo found at DOTFILE_PATH, would you like to create one? (y/n) ");
    if input.as_str() == "y" {
        Repository::init(var("DOTFILE_PATH").unwrap())
            .expect("Couldn't create a new git repo with DOTFILE_PATH");
    } else {
        exit(0);
    }
}

fn prompt(s: &str) -> String {
    print!("{}", s);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();

    return input;
}

pub fn setup_remote(dotfile_path: &str) {
    let response =
        prompt("You haven't set up a remote for your dotfile repo yet, would you like to? (y/n) ");

    if response == "n" {
        exit(0);
    } else {
        let remote = prompt("What is the address of the git remote repo?: ");
        let mut handle = Command::new("git")
            .arg("-C")
            .arg(format!("{}", dotfile_path))
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(&remote)
            .spawn()
            .unwrap();

        handle.wait().unwrap();
    }
}
