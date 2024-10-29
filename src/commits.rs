use std::{collections::HashSet, time::{SystemTime, UNIX_EPOCH}, vec};
use colored::Colorize;
use chrono::offset::Local;
use crate::{
    fileops::{self, compute_vec_hash, split_path, stringtofile},
    parsingops::{self, IndexEntry},
};

pub fn commit() {
    let (_index, indexentries, _indcheck) = parsingops::index_parser();
    let mut root_tree_object: Vec<String> = vec![];
    let mut added_dirs: HashSet<String> = HashSet::new();

    for entry in &indexentries {
        let splittedpath = split_path(&entry.name);
        let hash = &entry.sha;

        if splittedpath.len() == 2 {
            // File in root directory
            root_tree_object.push(format!("blob {} {}", &hash, splittedpath[1]));
        } else if splittedpath.len() > 2 && !added_dirs.contains(splittedpath[1]) {
            // Directory in root directory
            added_dirs.insert(splittedpath[1].to_string());
            let tree_hash = tree_object(&splittedpath[0..2].join("/"), &indexentries, &mut added_dirs);
            root_tree_object.push(format!("tree {} {}", tree_hash, splittedpath[1]));
        }
    }

    let root_hash = compute_vec_hash(&root_tree_object);
    let _ = stringtofile(format!("./.litt/objects/{}", root_hash).as_str(), root_tree_object);
    println!("TREE OBJ ROOT LOCATED IN {}", root_hash);
    commit_object("HI", true, root_hash,"".to_string());
}

fn tree_object(dir: &str, entries: &[IndexEntry], added_dirs: &mut HashSet<String>) -> String {
    let mut sub_tree_object: Vec<String> = Vec::new();

    for entry in entries {
        let splittedpath = split_path(&entry.name);
        let hash = &entry.sha;
        let current_level = dir.split('/').count() + 1;

        if splittedpath.len() == current_level && entry.name.starts_with(dir) {
            sub_tree_object.push(format!("blob {} {}", hash, splittedpath.last().unwrap()));
        } else if splittedpath.len() > current_level && entry.name.starts_with(dir) {
            let sub_dir = format!("{}/{}", dir, splittedpath[current_level - 1]);
            if !added_dirs.contains(&sub_dir) {
                added_dirs.insert(sub_dir.clone());
                let subtree_hash = tree_object(&sub_dir, entries, added_dirs);
                sub_tree_object.push(format!("tree {} {}", subtree_hash, splittedpath[current_level - 1]));
            }
        }
    }

    if !sub_tree_object.is_empty() {
        let hashof = compute_vec_hash(&sub_tree_object);
        let _ = stringtofile(format!("./.litt/objects/{}", hashof).as_str(), sub_tree_object);
        println!("TREE OBJ for '{}' LOCATED IN {}", dir, hashof);
        hashof
    } else {
        "".to_string()
    }
}




fn commit_object(message:&str,first:bool,sha_root:String,sha_parent:String){
    let author = "Laith Shishani"; // we will change this to fetch from a config file but just a test for now
    let email = "mrlaith44@gmail.com"; // we will change this to fetch from a config file but just a test for now
    let mut commit:Vec<String> = Vec::new();
    let current_branch = "master";
    commit.push(format!("tree <{}>",sha_root));
    if !first{
        commit.push(format!("parent <{}>",sha_parent));
    }
    commit.push(format!("author {} <{}> {} {}",author,email,SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards?").as_secs(),Local::now().format("%z")));
    commit.push(message.to_string());
    println!("{:#?}",commit);
    let hashof = compute_vec_hash(&commit);
    let _ = stringtofile(format!("./.litt/objects/{}", &hashof).as_str(), commit);
    let _ = stringtofile(format!("./.litt/refs/heads/{}", current_branch).as_str(), vec![hashof.clone()]);

    
    
    
    println!("Commited successfully! Hash: {}",hashof);

}



/*
tree <SHA-1 of root tree>
parent <SHA-1 of previous commit>  # only if this is not the first commit
author John Doe <johndoe@example.com> 1729864922 +0200
committer John Doe <johndoe@example.com> 1729864922 +0200

Initial commit

*/

