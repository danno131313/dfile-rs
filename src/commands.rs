use super::setup::setup_remote;
use git2::Repository;
use glob::glob;
use std::env::current_dir;
use std::env::var;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::io;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;
use time::{now, strftime};

/// Hard links each file provided to your dotfile directory, minus any dot prefixes.
/// Will structure the folders the same way relative to your home directory.
pub fn process_files(files: Vec<String>) -> Result<(), io::Error> {
    let home: PathBuf = var("HOME").expect("No $HOME variable set!").into();
    let dotfile_path: PathBuf = var("DOTFILE_PATH").unwrap().into();
    let curr_dir: PathBuf = current_dir().unwrap();

    for maybe_glob in files.into_iter() {
        let globbed = glob(&maybe_glob).unwrap();

        for maybe_file in globbed {
            let file = maybe_file.unwrap();
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

                let dotfile_str = dotfile_path.to_str().unwrap();
                let newpath_str = newpath_clone.to_str().unwrap();

                let _ = Command::new("git")
                    .arg("-C")
                    .arg(format!("{}", dotfile_str))
                    .arg("add")
                    .arg(format!("{}", newpath_str))
                    .output()
                    .unwrap();

                let _ = Command::new("git")
                    .arg("-C")
                    .arg(format!("{}", dotfile_str))
                    .arg("commit")
                    .arg("-m")
                    .arg(format!("\"add {}\"", name))
                    .output()
                    .unwrap();
            } else {
                println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    Ok(())
}

/// Adds all files in dotfile path to be committed, commits them,
/// and pushes the changes to the remote repository.
pub fn git_update(dotfile_path: &str) -> Result<(), &'static str> {
    let mut add = Command::new("git")
        .arg("-C")
        .arg(format!("{}", dotfile_path))
        .arg("add")
        .arg(".")
        .spawn()
        .map_err(|_| "could not add updated files to git")?;

    add.wait().unwrap();

    let commit = Command::new("git")
        .arg("-C")
        .arg(format!("{}", dotfile_path))
        .arg("commit")
        .arg("-m")
        .arg(format!("\"update changes: {}\"", get_current_time()))
        .output()
        .map_err(|_| "could not commit changes")?;

    let out = commit.stdout;
    let out_msg: String = OsString::from_vec(out).into_string().unwrap();

    if out_msg.contains("nothing to commit") && out_msg.contains("up to date") {
        println!("Nothing to update, exiting...");
        exit(0);
    }

    if has_remote(&dotfile_path) {
        let mut push = Command::new("git")
            .arg("-C")
            .arg(format!("{}", dotfile_path))
            .arg("push")
            .spawn()
            .map_err(|_| "could not push changes")?;

        push.wait().unwrap();
    } else {
        setup_remote(&dotfile_path);
        let mut push = Command::new("git")
            .arg("-C")
            .arg(format!("{}", dotfile_path))
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg("master")
            .spawn()
            .map_err(|_| "could not push changes to new remote repo")?;

        push.wait().unwrap();
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

fn get_current_time() -> String {
    strftime("%b %d, %Y (%H:%M:%S)", &now()).unwrap()
}

fn has_remote(dotfile_path: &str) -> bool {
    let repo = Repository::open(dotfile_path).unwrap();
    return repo.remotes().unwrap().len() > 0;
}
