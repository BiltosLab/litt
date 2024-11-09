use crate::fileops::compress_files_in_parallel;
use crate::scan_for_staging;
use crate::file_exists;
use crate::scanfiles_and_ignoremt;
use colored::Colorize;

pub fn add(args: Vec<String>) {
    if !file_exists("./.litt") {
        // I think this will suffice
        println!("{}: not a litt repository ", "fatal".red());
        return;
    }
    if args[0] == "." {
        let (file_list, hashmp) = scan_for_staging(".", true);
        if !file_list.is_empty() {
            let _result =
                compress_files_in_parallel(file_list, hashmp).expect("TODO: panic staging.rs");
        } else {
            println!("{}", "No changes to add to staging area".bright_cyan());
        }
        // We need to change this to only compress/add to staging modified files ONLY.[DONE]
    } else {// Bugged if you add a filename only to the args it will fail for whatever reason aka litt add text.txt but if you do litt add ./text.txt it works
        println!("ARGS DEBUG : {:#?}", args);
        let mut all_files_ok: bool = true;
        for file in &args {
            if !file_exists(&file) {
                println!("'{}' did not match any file", file.to_string().red());
                all_files_ok = false;
            }
        }
        if all_files_ok { // this is such a bad solution for the problem but i really cba 
            let hashmp = scan_for_staging(".", true);
            let mut files:Vec<String> = vec![];
            for i in args{
                match hashmp.1.values().find(|path| path.contains(&i)) 
                {
                    Some(matched_path) => {println!("Matched path in the map: {}", matched_path);
                    files.push(matched_path.to_string());},
                    None => println!("No matching path found in the map for {}", &i),
                }
            }
            let _ = compress_files_in_parallel(files, hashmp.1).expect("TODO: panic staging.rs");
        }
    }
}