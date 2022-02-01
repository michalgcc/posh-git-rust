#!/bin/sh

# Usage in Bash:
# posh-git-rust "`git status --long 2>&1`"

# Example PS1 to add to .bashrc
# PS1='[\u@\h \W] \[$(posh-git-rust "`git status --long 2>&1`")\]\$ '

cargo build --release
cp target/release/posh-git-rust ~/.local/bin/