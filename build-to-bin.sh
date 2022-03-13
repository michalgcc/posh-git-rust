#!/bin/bash

bin_path=target/release/posh-git-rust

cargo build --release
strip $bin_path
sudo cp $bin_path /usr/bin/