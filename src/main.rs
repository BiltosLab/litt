use crate::{fileops::*};
use colored::*;
use std::{env, fs, io, process::exit};
mod commits;
mod diff;
mod fileops;
mod init;
mod parsingops;
mod staging;
// figure out how to use env variables to store email,name of the commiter much easier than the other one i was thinking about
// which was create a file in .config/litt and store info there idk tbh
fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        helpcom();
        exit(1);
    }
    let cmd = &args[1];
    match cmd.as_str() {
        "init" => init::init(),
        "add" => {
            if args.len() < 3 {
                println!("ERROR! No args provided for add.");
                exit(1);
            }
            staging::add(args.into_iter().skip(2).collect());
        }
        "commit" => commits::commit(),
        "status" => status()?,
        "log" => log(),
        "cat-file" => {
            if args.len() < 3 {
                println!("ERROR! No args provided for cat-file.");
                exit(1);
            }
            if args[2].len() >= 7 {
                catfile(&args[2]);
            }
        }
        _ => println!("Unknown command: {}", cmd),
    }
    println!("DONE!");

    exit(0);
}

fn status() -> Result<(), io::Error> {
    //template for status func
    let original_lines = filetostring("./src/main.rs")?; // Just a note here : This original lines Vec<String> will get fetched from the database file using the commit number and our current files will get compared to it
    let modified_lines = filetostring("./src/main1.rs")?; //adbasdds
    let linediff = diff::find_diff_lines(original_lines, modified_lines);
    println!("Modified Lines Test:\n{}", linediff.join("\n"));
    Ok(())
}

fn log() {
    // parsingops::test_indextemp();
    // println!("IN LOG EXECUTED SUCCESSFULLY!");

    let mut count = 0;
    let mut a = scanfiles_and_ignoremt(".");
    for i in &mut a {
        count+=1;
        println!("{}", i);
    }
    println!("Found {} files", count);
}

fn helpcom() {
    scanfiles_and_ignoremt(".");
    println!("Litt Usage:\nlitt <first arg> <second arg> <third arg>\nEX: litt add . OR litt add file1.c file2.c\n");
}
