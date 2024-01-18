use std::{fs::{File, self},io::{BufRead, self, Write},io::Read};

use crate::fileparser::{filetostring, stringtofile, appendstr_to_file};
use crate::staging::{add,scanfile,scanfile_andignore,computehash};

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


pub fn diff_loader(){
    let commitfile= "./.litt/commit_history";
    let mut buffer:Vec<String> = Vec::new();
    let mut ignorelist:Vec<String> = Vec::new();
    ignorelist.push(".litt".to_string());
    ignorelist.push(".git".to_string());
    ignorelist.push("target".to_string());
    let trackedfilelist:Vec<String> = scanfile_andignore(".", &ignorelist);
    
    for file in trackedfilelist{
        appendstr_to_file(&commitfile, format!("{} {}\n",file,computehash(&file))).unwrap();

    }
}

/* "./.litt/trackedfilelist" 

appendstr_to_file(&commitfile, "DOGGIE".to_string()).unwrap();
 if let Err(err) = stringtofile(commitfile,buffer){
        eprintln!("Error writing to file {}",err);
    }
*/