use crate::fileops::{blob, compress_files_in_parallel};
use crate::scan_for_staging;
use crate::{file_exists, fileops::scanfiles_and_ignoremt};
use colored::Colorize;
use std::borrow::Borrow;

pub fn add(args:Vec<String>) {
    if !file_exists("./.litt") { // I think this will suffice
        println!("{}: not a litt repository ","fatal".red());
        return;
    }
    if args[0] == "." {
        // let file_list = scanfiles_and_ignoremt(".",true);
        let (file_list,hashmp) = scan_for_staging(".",true);
        if !file_list.is_empty(){
            let _result= compress_files_in_parallel(file_list,hashmp).expect("TODO: panic staging.rs L16");
        }
        else {
            println!("{}","No changes to add to staging area".bright_cyan());
        }
        // We need to change this to only compress/add to staging modified files ONLY.


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





