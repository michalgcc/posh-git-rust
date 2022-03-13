#!/bin/bash

echo -n "Please enter current version: "
read current_version

if [[ ! "$current_version" =~ ^[0-9]*\.[0-9]*\.[0-9]*$ ]]; then
    echo "Invalid version"
    exit 1
fi

echo -n "Please enter new version: "
read new_version

if [[ ! "$new_version" =~ ^[0-9]*\.[0-9]*\.[0-9]*$ ]]; then
    echo "Invalid version"
    exit 1
fi

echo -n "Current version: $current_version. New version: ${new_version}. Do you want to continue? [Y/n] "

read confirmation

if [ "$confirmation" != "Y" ] && [ "$confirmation" != "y" ]; then
    exit 0
fi

# Update Cargo.toml
sed -i "s/version = \"$current_version\"/version = \"$new_version\"/g;" Cargo.toml

# Update README.md
sed -i "s/$current_version/$new_version/g;" README.md

# Build, release, pack

rm -rf artifacts
mkdir artifacts

bin_path=target/release/posh-git-rust

cargo build --release
strip $bin_path

cp $bin_path artifacts

cd artifacts

tar -czf posh-git-rust-$new_version-x86_64-linux-gnu.tar.gz posh-git-rust

cd ..