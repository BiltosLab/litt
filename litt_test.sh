#!/usr/bin/sh


cargo build

cargo run init
cargo run add .
cargo run commit -m "Hello,World first commit"
cargo run branch newbranch
cargo run checkout newbranch
echo "hi" >> temp.txt
cargo run add temp.txt
cargo run commit -m "Hello,World second commit"
cargo run status