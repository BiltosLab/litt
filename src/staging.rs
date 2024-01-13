use std::{fs,fs::DirBuilder,fs::File,io::Write,io::Read};
use sha2::{Sha256, Digest};



pub fn add() { //template for add func
    let list = scanfile("./");
    let mut f  = File::create("./.litt/config.txt").unwrap();
    

    for l in list{
        println!("{}",l);
        f.write_all(l.as_bytes()).unwrap();
        f.write_all(b"\t").unwrap();
        f.write_all(computehash(&l).as_bytes()).unwrap();
        f.write_all(b"\n").unwrap();
    }
    
    println!("Added changes to the staging area.");

}


pub fn scanfile(realpath:&str) -> Vec<String> {
    let mut filelist:Vec<String> = Vec::new();

    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){ 
                    if metta.is_dir(){
                        filelist.extend(scanfile(&path.path().to_string_lossy()));
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
    let mut hasher = Sha256::new();

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