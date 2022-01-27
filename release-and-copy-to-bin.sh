#!/bin/sh

# Example PS1 to add to .bashrc
# PS1='[\u@\h \W] $(rusty-git-status)\$ '

cargo build --release
cp target/release/rusty-git-status ~/.local/bin/