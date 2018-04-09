use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use git2::Repository;
use std::env::var;
use std::fs::create_dir_all;
use std::process::{exit, Command};

/// Creates a new git repo at the DOTFILE_PATH directory
pub fn new_git() {
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
pub fn setup() -> String {
    println!("You haven't set up a DOTFILE_PATH environment variable yet!");
    print!("Would you like to? (y/n) ");
    io::stdout().flush().unwrap();
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
            .expect("Couldn't open rc file!");

        writeln!(file, "export DOTFILE_PATH={}", path).expect("Couldn't write to rc file!");

        create_dir_all(&path).unwrap_or_else(|_| {
            println!("Couldn't create DOTFILE_DIR with that path! Exiting...");
            exit(1);
        });

        Command::new(format!("export DOTFILE_PATH={}", path))
            .spawn()
            .unwrap();

        return path;
    }
}
