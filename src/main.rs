use crate::{fileops::*};
use colored::*;
use std::{env, fs, io, process::exit};
mod commits;
mod diff;
mod fileops;
mod init;
mod parsingops;
mod staging;
mod log;
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
        "commit" => {
            // Check for the -m flag and collect all following arguments as the commit message
            if let Some(message_flag) = args.iter().position(|arg| arg == "-m") {
                if message_flag + 1 < args.len() {
                    // Join the arguments after -m to form the commit message
                    let message: String = args[message_flag + 1..].join(" ");
                    commits::commit("-m", &message);
                } else {
                    println!("ERROR! No commit message provided.");
                    exit(1);
                }
            } else {
                println!("ERROR! Missing -m flag for commit message.");
                exit(1);
            }
        },
        "status" => status()?,
        "log" => log::log(),
        "cat-file" => {
            if args.len() < 3 {
                println!("ERROR! No args provided for cat-file.");
                exit(1);
            }
            if args[2].len() >= 7 {
                catfile(&args[2]);
            }
        },
        "checkout" => {
            if args.len() < 3 {
                println!("ERROR! No commit hash provided for checkout.");
                exit(1);
            }
            let partial_hash = args[2].clone();
            commits::checkout_commit(partial_hash);
        }
        _ => println!("Unknown command: {}", cmd),
    }
    println!("DONE!");

    exit(0);
}

fn status() -> Result<(), io::Error> {
    //template for status func
    // let original_lines = filetostring("./src/main.rs")?; // Just a note here : This original lines Vec<String> will get fetched from the database file using the commit number and our current files will get compared to it
    // let modified_lines = filetostring("./src/main1.rs")?; //adbasdds
    // let linediff = diff::find_diff_lines(original_lines, modified_lines);
    // println!("Modified Lines Test:\n{}", linediff.join("\n"));
    
    let a =scan_for_staging(".",true);
    println!("{:#?}",a);

    // let a = scanfiles_and_ignoremt(".", true);

    // for i in a {
    //     println!("{:#?},{:#?}",i,computehashmt(&i).unwrap());
    // }
    Ok(())
}




fn helpcom() {
    println!("Litt Usage:\nlitt <first arg> <second arg> <third arg>\nEX: litt add . OR litt add file1.c file2.c\n");
}
