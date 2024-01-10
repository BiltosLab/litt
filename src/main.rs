use std::{fs,env, process::exit};

use colored::*;

fn main() {
    println!("DONE!");
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