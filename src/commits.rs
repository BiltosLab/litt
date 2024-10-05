/*
 * LITT - Version Control System
 *
 * Copyright (C) Laith Bilto, 2024
 */
use std::{fs::{File, self},io::{BufRead, self, Write},io::Read};

use crate::filestuff::{filetostring, stringtofile, appendstr_to_file, scanfiles_and_ignoremt, computehash, littignore};
use crate::staging::{add};

pub fn commit() {
    //template for commit func
    //TODO HERE
    /*
    Gen a commit number from the SHA-256 Hash then what we do is we load the diffs to the database with the name of author 
    and all that but also we use the commit hash to return to it and rebuild the file like thaat.
    */
  
    diff_loader(); 
   

    println!("Committed changes.");

}


pub fn diff_loader(){ // .littignore needed.
    let commitfile= "./.litt/commit_history";
    let mut buffer:Vec<String> = Vec::new();
    let mut ignorelist:Vec<String> = Vec::new();
    ignorelist.push(".litt".to_string());
    ignorelist.push(".git".to_string());
    ignorelist.push("target".to_string());
    let trackedfilelist:Vec<String> = scanfiles_and_ignoremt(".");
    
    for file in trackedfilelist{
        if let Err(err) = appendstr_to_file(&commitfile,format!("{}\t{}",file,computehash(&file).unwrap())) {
            eprintln!("Error {}",err)
        }
    }
}

/* "./.litt/trackedfilelist" 

appendstr_to_file(&commitfile, "DOGGIE".to_string()).unwrap();
 if let Err(err) = stringtofile(commitfile,buffer){
        eprintln!("Error writing to file {}",err);
    }
*/

