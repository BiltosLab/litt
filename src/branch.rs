use std::process::exit;

use colored::Colorize;

use crate::{commits::{checkout_commit, get_current_commit}, file_exists, filetostring, scanfiles_and_ignoremt, stringtofile};






// Return branch name + if heads file is empty indicating fresh repo.
pub fn get_branch()->(String,bool){
    let head: Vec<String> = if file_exists("./.litt/HEAD") {
        filetostring("./.litt/HEAD").unwrap_or_default()
    } else {
        Vec::new()
    };
    if head.is_empty(){
        eprintln!("{}","Corruption in HEAD".red());
        exit(1);
    }
    let heads = scanfiles_and_ignoremt("./.litt/refs/heads", false);
    if heads.is_empty(){return (head[0].to_string(),true);}
    else {
        return (head[0].to_string(),false);
    }
}

pub fn create_new_branch(name:String) {
    let heads = scanfiles_and_ignoremt("./.litt/refs/heads", false);
    if heads.is_empty(){eprintln!("{}","Do your first commit then you can branch.".red());return;}
    let curr_head_hash = get_current_commit();
    let _ = stringtofile(&format!("./.litt/refs/heads/{}",name), vec![curr_head_hash]);
    println!("Branch {} was created successfully!",name.green());
}

pub fn get_heads() -> Vec<String> {
    let heads = scanfiles_and_ignoremt("./.litt/refs/heads", false);
    let mut final_heads: Vec<String> = Vec::new();
    for i in heads {
        final_heads.push(i.rsplit('/').next().unwrap().to_string());
    }
    return final_heads;
}


pub fn checkout_branch(name:String){
    // let heads = get_heads();
    // if heads.contains(&name){ // i know we checked already but its w.e
        let hashfile = filetostring(&format!("./.litt/refs/heads/{}", name)).unwrap_or_default();

        checkout_commit(hashfile[0].clone());
        let bname: Vec<String> = vec![name];
        let _ = stringtofile("./.litt/HEAD", bname);
    // }
    // else {
    //     eprintln!("branch {} doesnt exist",name.red());
    //     return;
    // }

}