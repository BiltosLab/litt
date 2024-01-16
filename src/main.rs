use std::{fs,env, process::exit};
use colored::*;
use differ::Differ;

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
    let exam:Vec<String> = filetostring("./src/main.rs");

    for line in exam{ 
        println!("{}",line);
    }
    println!("Committed changes.");

}

fn status() { //template for status func
    let original_lines:Vec<String> = filetostring("./src/main1.rs");
    let modified_lines:Vec<String> = filetostring("./src/main.rs");
    let (diff_lines,linediff) = diff::find_diff_lines(&original_lines, &modified_lines);
    //println!("Formatted Diff:\n{:?}", diff_lines);
    println!("Modified Lines Test:\n{}", linediff.join("\n"));

}

fn log() {  //template for log func
    println!("Commit history:");
}