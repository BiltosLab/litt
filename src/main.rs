use std::{fs,env, process::exit, io};
use colored::*;
use crate::filestuff::filetostring;
mod littinit;
mod staging;
mod filestuff;
mod diff;
mod commits;
fn main() -> Result<(), io::Error> {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {exit(1)}
    let cmd = &args[1];
    match cmd.as_str() {
        "init" => littinit::init(),
        "add" => staging::add(),
        "commit" => commits::commit(),
        "status" => status()?,
        "log" => log(),
        _ => println!("Unknown command: {}", cmd),
    }
    println!("DONE!");
    exit(0);
}





fn status() ->Result<(), io::Error>  { //template for status func
    let original_lines = filetostring("./src/main.rs")?; // Just a note here : This original lines Vec<String> will get fetched from the database file using the commit number and our currect files will get compared to it 
    let modified_lines= filetostring("./src/main1.rs")?; //adbasdds
    let linediff = diff::find_diff_lines(original_lines, modified_lines);
    //println!("Formatted Diff:\n{:?}", diff_lines);
    println!("Modified Lines Test:\n{}", linediff.join("\n"));
    /*Seems like t he diff function works as expected now the TODO HERE IS implement a database file and diff between it 
    and the source files that are supposed to be in the repo */
    Ok(())
}

fn log() {  //template for log func
    println!("Commit history:");
}