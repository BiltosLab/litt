use std::{process::exit, time::SystemTime};

use chrono::Local;
use colored::Colorize;

use crate::{commits::{checkout_commit, get_current_commit}, file_exists, filetostring, scanfiles_and_ignoremt, stringtofile};
use crate::compute_vec_hash;





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



pub fn merge(){
    
}

/* TODO BLOCK
i think the best way to do a merge and the simplest is when we have a fastforward case which is literally one of the simplest forms of branching then merging
simply let's assume we're on master then we branch off and continue there, assuimg we havent gone back to master and did a change all we can simply do 
is create a new merge type of commit where we point the master pointer to the new branches last commit and that will be the head of master.
*/

pub fn merge_commit(message: &str,master:String,slave:String) {
    if get_heads().is_empty(){
        return;
    }
    let author = "Laith Shishani"; // we will change this to fetch from a config file but just a test for now
    let email = "mrlaith44@gmail.com"; // we will change this to fetch from a config file but just a test for now
    let mut commit: Vec<String> = Vec::new();
    let current_branch = get_branch().0;

    commit.push(format!("parent <{}>", slave));
    commit.push(format!("parent <{}>", master));

    commit.push(format!(
        "author {} <{}> {} {}",
        author,
        email,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards?")
            .as_secs(),
        Local::now().format("%z")
    ));
    commit.push(message.to_string());
    println!("{:#?}", commit);
    let hashof = compute_vec_hash(&commit);
    let _ = stringtofile(format!("./.litt/objects/{}", &hashof).as_str(), commit);
    let _ = stringtofile(
        format!("./.litt/refs/heads/{}", current_branch).as_str(),
        vec![hashof.clone()],
    );

    println!("Merged successfully! Hash: {}", hashof);}