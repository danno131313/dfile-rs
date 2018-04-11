# dfile
A small Rust program for organizing your dotfiles. This program hard links your dotfiles and scripts from all across your home directory into a single directory, minus the dot prefixes, and while maintaining folder structure relative to $HOME.

Running ```dfile``` by itself will attempt to add, commit, and push all changes in the dotfile directory to a remote repo, setting one up if none exists.

Run ```dfile --help``` for usage.

*Runs on Unix-based systems and Rust stable 1.25.
