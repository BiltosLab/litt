use crate::filestuff::{compress_files_in_parallel, computehashmt};
use crate::{file_exists, filestuff::{compressfile, decompressfile, filetostring, scanfiles_and_ignoremt}, scan_objects};
use colored::Colorize;
use std::borrow::Borrow;

pub fn add(args:Vec<String>) { //template for add func
    if !file_exists("./.litt") { // I think this will suffice
        println!("{}: not a litt repository ","fatal".red());
        return;
    }
    if args[0] == "." {
        let a = compress_files_in_parallel(scanfiles_and_ignoremt(".")).expect("TODO: panic message");

    }
    else { // IDK if this needs to be multithreaded too because the user prob will enter the names of like 5 or 6 files max ?
        for file in args { // DO NOT USE THIS NOW! // TODO
            if file_exists(&file) {
                blob(&file); // This needs to be changed to add file to index
                println!("File compressed {} :",file);}
            else {
                println!("'{}' did not match any file",file.to_string().red());
            }
        }
    }
 
    // Change Code to include each file compressed in "staging area"
    println!("ok here i did litt cat and first few letters in the commit hash so lets decompress then ill see what i can do");
    println!("Added changes to the staging area.");

}





pub fn blob(filename:&str){
    let a = computehashmt(filename).unwrap();
    compressfile(filename, ("./.litt/objects/".to_owned()+&a).as_str()).unwrap();

}


pub fn catfile(hashoffile:&str){
    let obj= scan_objects(hashoffile);
    println!("{}",obj);
    decompressfile(&obj, "./.litt/tempf").unwrap();
    println!("{}",filetostring("./.litt/tempf").unwrap().join("\n").blue());

}