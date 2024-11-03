use crate::fileops::{blob, compress_files_in_parallel};
use crate::{file_exists, fileops::scanfiles_and_ignoremt};
use colored::Colorize;
use std::borrow::Borrow;

pub fn add(args:Vec<String>) {
    if !file_exists("./.litt") { // I think this will suffice
        println!("{}: not a litt repository ","fatal".red());
        return;
    }
    if args[0] == "." {
        let result = compress_files_in_parallel(scanfiles_and_ignoremt(".")).expect("TODO: panic staging.rs L12");
        //79084877629677394630c63b1af455110ce8e9180670217dbeb1071482800736
        // for i in result {
        //     println!("{} This is HashMap 0 - -",i.0);
        //     println!("{} This is HashMap 1 - -",i.1);
        // }


    }
    else { // IDK if this needs to be multithreaded too because the user prob will enter the names of like 5 or 6 files max ?
        for file in args { // DO NOT USE THIS NOW! // TODO
            if file_exists(&file) {
                println!("{}", "Incomplete Feature use the . instead!!".red());
                blob(&file); // This needs to be changed to add file to index
                println!("File compressed {} :",file);}
            else {
                println!("'{}' did not match any file",file.to_string().red());
            }
        }
    }
}





