# posh-git-rust

Implementation of posh-git written in Rust.
Based on https://github.com/dahlbyk/posh-git.


# Installation
Tested on Fedora Linux 35, Rust 1.58 stable

* Execute `./release-and-copy-to-bin.sh`
  * Binary will be created and copied to /usr/bin
* Update `.bashrc` to include posh-git-rust binary in prompt ``posh-git-rust "`git status --long 2>&1`"``
  * Example: ``PS1='[\u@\h \W] \[$(posh-git-rust "`git status --long 2>&1`")\]\$ '``

# Example:
![example_output](https://raw.githubusercontent.com/michalgcc/posh-git-rust/main/screenshots/example.png "Example output")
