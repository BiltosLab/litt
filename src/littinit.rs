use std::{fs,fs::DirBuilder,fs::File,io::Write};


pub fn init() { // TODO! : Implement an init func aka create files and dirs and setup something idk
    let filestruct = ["./.litt/objects","./.litt/logs","./.litt/ref","./.litt/ref/heads","./.litt/ref/remotes"]; // we can add whatever folders we need to create at init time.
    let files = ["./.litt/config.txt"];
    println!("Init");
    let path = "./.litt";
    mkdir(path);
    for dir in filestruct{
        mkdir(dir);
    }

    for file in files{
        touch(file);
    }
}



fn mkdir(path:&str) { 
    DirBuilder::new()
    .recursive(true)
    .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    
}

fn touch(path:&str) {
let mut f = File::create(path).unwrap();
f.write_all(b"Hello,World!").unwrap();
assert!(fs::metadata(path).unwrap().is_file());

}