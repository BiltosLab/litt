use std::collections::HashMap;

use chrono::{TimeZone, Utc};
use colored::Colorize;

use crate::
    commits::{get_current_commit, parse_commit_data}
;

pub fn log() {
    let mut currcommit = get_current_commit();
    let mut data = parse_commit_data(currcommit.clone()).unwrap_or_default();
    if !data.is_empty() {
        loop {
            if data.contains_key("parent") { // Exists in every commit after the first
                print_commit_info(&currcommit, &data);
                currcommit = data.get("parent").unwrap().to_string();
                data = parse_commit_data(currcommit.clone()).unwrap_or_default();
            }
            else if data.contains_key("tree") { // Exists in every commit 
                print_commit_info(&currcommit, &data);
                break;
            }
            else if !data.is_empty() { // if data is not empty but there are no matches for parent or tree that means we have corruption
                println!("{}: Corruption in commit object ", "fatal".red());
                break;
            }
        }
    } 
}


fn print_commit_info(currcommit:&String,data:&HashMap<String, String>){
    {
        let dateunix: &str = &data.get("commit_time").unwrap();
        let stamp = dateunix.parse::<i64>().unwrap_or_default();

        // println!("{:#?}", &firstdata);
        println!("commit {}", currcommit);
        println!("Author: {} <{}>", &data.get("author_name").unwrap(),&data.get("author_email").unwrap());
        println!(
            "Date: {:?} {}\n",
            Utc.timestamp_opt(stamp, 0).unwrap(),
            &data.get("timezone").unwrap()
        );
        println!("  {}", &data.get("message").unwrap());  
        println!("\n");
         
    }
}