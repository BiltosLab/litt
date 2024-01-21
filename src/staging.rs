use std::{fs,fs::DirBuilder,fs::File,io::{Write, self},io::Read, borrow::{Borrow, BorrowMut}};
use sha2::{Sha256, Digest};
use crate::{filestuff::{filetostring, appendstr_to_file, search_and_destroy,scanfiles_andignore,create_object}, diff::find_diff_lines};



pub fn add() { //template for add func
    let mut ignorelist:Vec<String> = Vec::new();
    ignorelist.push(".git".to_string());
    ignorelist.push("target".to_string());
    // Add logic to parse a new file called 
    //.littignore later that will push everyline to this Vec and ignore anything in that list
   // while !checkfiletracker(scanfile_andignore("./",&ignorelist)){ filetrackupdater(scanfile_andignore("./",&ignorelist));}
    // for an unknown reason running filetrackerupdater once if the file has been changed aka a file has been deleted and list doesnt equal the file because of that it makes the file act funny idk 
    //this seems to have fixed it
    create_object("blob");



    println!("Added changes to the staging area.");

}

// realpath = Obviously the real path for example "./" , ignore Vector has the names of the files we wanna ignore gonna do something above
// to parse a file called .littignore (I know XD)



/*
while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    } 
    // another implementation for that loop i feel this is better
    // but w.e we can take care of it later current loop causes no trouble till we deal with the
    // unwrap stuff.
    */



