use std::{fs,fs::DirBuilder,fs::File,io::Write};




pub fn add() { //template for add func
    let list = scanfile("./");
    let mut f  = File::create("./.litt/config.txt").unwrap();
    

    for l in list{
        println!("{}",l);
        f.write_all(l.as_bytes()).unwrap();
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



