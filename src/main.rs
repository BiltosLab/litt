use std::{fs,env, process::exit};
use colored::*;
<<<<<<< HEAD
=======
use differ::Differ;
>>>>>>> 5681678df4983228bcbee7c1230f127b14310c15

use crate::fileparser::filetostring;
mod littinit;
mod staging;
mod fileparser;
mod diff;
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {exit(1)}
    let cmd = &args[1];
    match cmd.as_str() {
        "init" => littinit::init(),
        "add" => staging::add(),
        "commit" => commit(),
        "status" => status(),
        "log" => log(),
        _ => println!("Unknown command: {}", cmd),
    }
    println!("DONE!");
    exit(0);
}




fn commit() { //template for commit func
    //TODO HERE
    /*
    Gen a commit number from the SHA-256 Hash then what we do is we load the diffs to the database with the name of author 
    and all that but also we use the commit hash to return to it and rebuild the file like thaat.
    */
    println!("Committed changes.");

}

fn status() { //template for status func
    let original_lines:Vec<String> = filetostring("./src/main.rs"); // Just a note here : This original lines Vec<String> will get fetched from the database file using the commit number and our currect files will get compared to it 
    let modified_lines:Vec<String> = filetostring("./src/main1.rs"); //adbasdds
    let linediff = diff::find_diff_lines(original_lines, modified_lines);
    //println!("Formatted Diff:\n{:?}", diff_lines);
    println!("Modified Lines Test:\n{}", linediff.join("\n"));
    /*BLAH BLAH */
}

fn log() {  //template for log func
    println!("Commit history:");
}