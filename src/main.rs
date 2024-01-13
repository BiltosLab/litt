use std::{fs,env, process::exit};

use crate::fileparser::filetostring;
mod littinit;
mod staging;
mod fileparser;

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

    /*for line in exam{
        println!("{}",line);
    } 
 */

    println!("{}",exam[12]);
    println!("Committed changes.");
}

fn status() { //template for status func
    println!("Status: No changes");
}

fn log() {  //template for log func
    println!("Commit history:");
}