use crate::{fileops::*};
use branch::{checkout_branch, create_new_branch, get_branch, get_heads};
use colored::*;
use commits::compare_commit_to_staging;
use std::{env, fs, io, process::exit};
mod commits;
mod diff;
mod fileops;
mod init;
mod parsingops;
mod staging;
mod log;
mod branch;
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
            else if args.len() == 3 {
                checkout(args[2].clone());
            }
        },
        "branch" => {
            if args.len() < 3 {
                println!("len = {}",args.len());
                for i in get_heads(){
                    if i == get_branch().0{
                        println!(" * {}",i.green());
                    }
                    else {
                        println!("   {}",i);
                    }
                }
            }
            else if args.len() == 3 {
                create_new_branch(args[2].clone());
            }
        }
        _ => println!("Unknown command: {}", cmd),
    }
    println!("DONE!");

    exit(0);
}

fn status() -> Result<(), io::Error> {
    let cmp = compare_commit_to_staging();

    let a =scan_for_staging(".",true);


    let head:String = get_branch().0;
    println!("On branch {}",head);
    if !&cmp.0.is_empty(){
        println!("Changes to be committed:\n  (use \"litt commit ...\" to commit)");
        // if !head.is_empty(){
            for i in &cmp.0{
                println!("{}",format!("\tmodified:  {}",i).green());
            }
        // }
            println!("no changes added to commit (use \"litt add\")");
    }
    if !&a.0.is_empty() {
            println!("Changes not staged for commit:\n  (use \"litt add <file>...\" to update what will be committed)");
            // if !head.is_empty(){
                for i in &a.0{
                    println!("{}",format!("\tmodified:  {}",i).red());
                }
            // }
                println!("no changes added to commit (use \"litt add\")");
        
    }
    if a.0.is_empty() && cmp.0.is_empty() {
        println!("\n\nnothing to commit, working tree clean");
    }
    // let a = compare_commit_to_staging();
    Ok(())
}


// git status
// On branch master
// Your branch is up to date with 'origin/master'.

// Changes not staged for commit:
//   (use "git add <file>..." to update what will be committed)
//   (use "git restore <file>..." to discard changes in working directory)
//         modified:   src/fileops.rs
//         modified:   src/main.rs


// On branch master
// Your branch is up to date with 'origin/master'.

// Changes to be committed:
//   (use "git restore --staged <file>..." to unstage)
//         modified:   src/fileops.rs
//         modified:   src/main.rs


fn checkout(target:String){
    if file_exists(&format!("./.litt/refs/heads/{}",target))
    {
        checkout_branch(target);
    }
    else {
        let partial_hash = target;
        commits::checkout_commit(partial_hash);
    }
}


fn helpcom() {
    println!("Litt Usage:\nlitt <first arg> <second arg> <third arg>\nEX: litt add . OR litt add file1.c file2.c\n");
}


    //template for diff func test
    // let original_lines = filetostring("./src/main.rs")?; // Just a note here : This original lines Vec<String> will get fetched from the database file using the commit number and our current files will get compared to it
    // let modified_lines = filetostring("./src/main1.rs")?; //adbasdds
    // let linediff = diff::find_diff_lines(original_lines, modified_lines);
    // println!("Modified Lines Test:\n{}", linediff.join("\n"));



    // mod