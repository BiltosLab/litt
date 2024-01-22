use std::{fs,fs::DirBuilder,fs::File,io::{Write, self},io::Read, borrow::{Borrow, BorrowMut}};
use sha2::{Sha256, Digest};
use crate::{filestuff::{filetostring, appendstr_to_file, search_and_destroy,scanfiles_andignore,create_object,compressfile,decompressfile, computehash}, diff::find_diff_lines};



pub fn add() { //template for add func
    let mut ignorelist:Vec<String> = Vec::new();
    let objdir = "./.litt/objects/";    
    ignorelist.push(".git".to_string());
    ignorelist.push("target".to_string());
    // Add logic to parse a new file called 
    //.littignore later that will push everyline to this Vec and ignore anything in that list
   // while !checkfiletracker(scanfile_andignore("./",&ignorelist)){ filetrackupdater(scanfile_andignore("./",&ignorelist));}
    // for an unknown reason running filetrackerupdater once if the file has been changed aka a file has been deleted and list doesnt equal the file because of that it makes the file act funny idk 
    //this seems to have fixed it
    blob("./src/main.rs");
    let f = computehash("./src/main.rs").unwrap();
    catf(format!("{}{}",objdir,f).as_str());
    println!("ok here i did litt cat and first few letters in the commit hash so lets decompress then ill see what i can do");
    println!("Added changes to the staging area.");

}





pub fn blob(filename:&str){
    let a = computehash(filename).unwrap();
    compressfile(filename, ("./.litt/objects/".to_owned()+&a).as_str()).unwrap();

}


pub fn catf(hashoffile:&str){
    let a = "./THEFILE2.rs";
    decompressfile(&hashoffile, &a).unwrap();
    let b = filetostring(&a).unwrap();
    println!("{}",b.join("\n"));
}