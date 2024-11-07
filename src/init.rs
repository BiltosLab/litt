use crate::fileops::*;


// I think this is mostly complete now just need the mkdir/touch mode to hide files
pub fn init() {
    if file_exists("./.litt") { // I think this will suffice
        println!("litt repository already exists");
        return;
    } /* TODO : we have to rewrite the mkdir/touch functions or add another one that creates hidden files/folders for windows or modify the same one to have a bool for hidden or not */
    let lines: Vec<String> = vec![ // Some random Index to get it going
        "[header]".to_string(),
        "  signature = DIRC".to_string(),
        "  version = 1".to_string(),
        "  entries = 1".to_string(),
        "".to_string(),
        "[checksum]".to_string(),
        "  checksum = true".to_string(),
        "  sha1 = 1ef0972eb948e6229240668effcb9c600fe5888d".to_string(),
    ];
    let head: Vec<String> = vec![ // Some random Index to get it going
    "master".to_string(),
];

    let filestruct = ["./.litt","./.litt/branches","./.litt/hooks","./.litt/info","./.litt/refs","./.litt/refs/heads","./.litt/refs/tags","./.litt/objects","./.litt/objects/pack","./.litt/objects/info",]; // we can add whatever folders we need to create at init time.
    let files = ["./.litt/info/exclude","./.litt/description","./.litt/HEAD","./.littignore","./.litt/index"];
    println!("Init");
    let path = "./.litt";
    mkdir(path);
    for dir in filestruct{
        mkdir(dir);
    }
    for file in files{
        touch(file);
    }
    stringtofile("./.litt/index", lines).expect("Index Panic");
    stringtofile("./.litt/HEAD",head ).expect("HEAD Corruption");
}



