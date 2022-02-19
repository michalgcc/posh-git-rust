#!/bin/bash

if [[ $(mount | grep 'posh-git-rust/target') ]] ; then
    echo "Already mounted"
else
    mkdir -p target
    rm ./target/* -rf
    sudo mount -t tmpfs -o size=1G tmpfs ./target
    echo "Mounted"
fi
