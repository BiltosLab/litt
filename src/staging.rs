use std::{fs,fs::DirBuilder,fs::File,io::Write,io::Read, borrow::{Borrow, BorrowMut}};
use sha2::{Sha256, Digest};



pub fn add() { //template for add func
    let mut ignorelist:Vec<String> = Vec::new();
    ignorelist.push(".git".to_string());
    ignorelist.push("target".to_string());
    // Add logic to parse a new file called 
    //.littignore later that will push everyline to this Vec and ignore anything in that list
    let list = scanfile("./",&ignorelist);
    let mut f  = File::create("./.litt/config.txt").unwrap();
    

    for l in list{
        println!("{}",l);
        f.write_all(l.as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
        f.write_all(computehash(&l).as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
    }
    
    println!("Added changes to the staging area.");

}

// realpath = Obviously the real path for example "./" , ignore Vector has the names of the files we wanna ignore gonna do something above
// to parse a file called .littignore (I know XD)
pub fn scanfile(realpath:&str,ignore:&Vec<String>) -> Vec<String> { 
    let mut filelist:Vec<String> = Vec::new();
    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){ 
                    if metta.is_dir(){
                        if ignore.contains(path.file_name().to_string_lossy().to_string().borrow_mut()) {continue;}
                        filelist.extend(scanfile(&path.path().to_string_lossy(),ignore));
                    }
                    else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());
                        filelist.push(path.path().to_str().unwrap().to_string());
                        //println!("{:?}",path.path().to_str().unwrap());
                    }
                }
                
            }
            
        }
    }


    return filelist;
}



fn computehash(file: &str) -> String { // Need to change this to return result instead but its fine for testing i guess
    // Open the file
    let mut file = File::open(file).unwrap();

    // Create a SHA-256 "hasher"
    let mut hasher = Sha256::new(); // Rust analyzer thinks this's an error or something just ignore.

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash and get the result as a byte array
    let result = format!("{:x}", hasher.finalize());
    return result;
}

/*
while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    } 
    // another implementation for that loop i feel this is better
    // but w.e we can take care of it later current loop causes no trouble till we deal with the
    // unwrap stuff.
    */