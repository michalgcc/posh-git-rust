#!/bin/sh

# Example PS1 to add to .bashrc
# PS1='[\u@\h \W] \[$(posh-git-rust)\]\$ '

cargo build --release
cp target/release/posh-git-rust ~/.local/bin/