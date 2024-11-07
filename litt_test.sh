#!/usr/bin/sh


cargo build

cargo run init
cargo run add .
cargo run commit -m "Hello,World first commit"