use std::{fs,env, process::exit,fs::DirBuilder};

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {exit(1)}
    let cmd = &args[1];
    match cmd.as_str() {
        "init" => init(),
        "add" => add(),
        "commit" => commit(),
        "status" => status(),
        "log" => log(),
        _ => println!("Unknown command: {}", cmd),
    }
    println!("DONE!");
    list_all(".", ".c", true);
    exit(0);
}

fn list_all(realpath:&str,search:&str,hidden:bool){
    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){
                    if metta.is_dir(){
                        if path.file_name().to_str().unwrap().starts_with(".") && hidden{
                            continue;
                        }
                        //println!("{:?}",path.path().to_str().unwrap());
                        list_all(&path.path().to_string_lossy(),search,hidden);
                    }
                    else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());

                    }
                }
                
            }
            
        }
    }

}


fn init() { // TODO! : Implement an init func aka create files and dirs and setup something idk
    println!("Init");
    let path = "./.litt";
    DirBuilder::new()
    .recursive(true)
    .create(path).unwrap();

    assert!(fs::metadata(path).unwrap().is_dir());
}

fn add() { //template for add func
    println!("Added changes to the staging area.");
}

fn commit() { //template for commit func
    println!("Committed changes.");
}

fn status() { //template for status func
    println!("Status: No changes");
}

fn log() {  //template for log func
    println!("Commit history:");
}