use crate::fileops::compress_files_in_parallel;
use crate::scan_for_staging;
use crate::file_exists;
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
    } else {
        println!("ARGS DEBUG : {:#?}", args);
        let mut all_files_ok: bool = true;
        for file in &args {
            if !file_exists(&file) {
                println!("'{}' did not match any file", file.to_string().red());
                all_files_ok = false;
            }
        }
        if all_files_ok {
            let hashmp = scan_for_staging(".", true);
            let _ = compress_files_in_parallel(args, hashmp.1).expect("TODO: panic staging.rs");
        }
    }
}
