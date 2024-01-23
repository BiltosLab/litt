use std::{fs,fs::DirBuilder,fs::File,io::Write};

use crate::filestuff::*;

pub fn init() { // TODO! : Implement an init func aka create files and dirs and setup something idk
    let filestruct = ["./.litt","./.litt/branches","./.litt/hooks","./.litt/info","./.litt/refs","./.litt/refs/heads","./.litt/refs/tags","./.litt/objects","./.litt/objects/pack","./.litt/objects/info",]; // we can add whatever folders we need to create at init time.
    let files = ["./.litt/info/exclude","./.litt/description","./.litt/HEAD","./.littignore"];
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



