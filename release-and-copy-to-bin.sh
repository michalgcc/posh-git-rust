#!/bin/bash

# Usage in Bash:
# posh-git-rust "`git status --long 2>&1`"

# Example PS1 to add to .bashrc
# PS1='[\u@\h \W] '$(posh-git-rust "`git status --long 2>&1`")'$ '

bin_path=target/release/posh-git-rust

cargo build --release
strip $bin_path
sudo cp $bin_path /usr/bin/