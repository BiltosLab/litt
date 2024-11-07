use std::{collections::{HashMap, HashSet}, fs, path::PathBuf, process::exit, time::{SystemTime, UNIX_EPOCH}, vec};
use colored::Colorize;
use chrono::offset::Local;
use crate::{
    fileops::{self, compute_vec_hash, split_path, stringtofile}, filetostring, find_full_hash, parsingops::{self, IndexEntry}
};
#[derive(Clone)]
#[derive(Debug)]
struct Treeobj {
    entry_type: String,
    hash: String,
    name: String,
}


pub fn commit(option:&str,message:&str) {
    let (_index, indexentries, _indcheck) = parsingops::index_parser();
    let mut root_tree_object: Vec<String> = vec![];
    let mut added_dirs: HashSet<String> = HashSet::new();
    let first = fileops::file_exists("./.litt/refs/heads/master");
    if message.is_empty(){
        eprintln!("{}","Empty commit message".red());
        return;
    }

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
    // Get previous commit if it exists.
    // let head = filetostring("./.litt/refs/heads/master").unwrap();
    // let data = parse_commit_data(head[0].clone()).unwrap_or_default();
    // let root_tree_hash = data.get("tree").unwrap().to_string();
    // let parent = data.get("parent").map(|s| s.to_string()).unwrap_or(String::from(""));
    // let first_commit = data.contains_key("parent");
    

    let root_hash = compute_vec_hash(&root_tree_object);
    let _ = stringtofile(format!("./.litt/objects/{}", root_hash).as_str(), root_tree_object);
    println!("TREE OBJ ROOT LOCATED IN {}", root_hash);
    commit_object(message, !first, root_hash);
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




fn commit_object(message:&str,first:bool,sha_root:String){
    let author = "Laith Shishani"; // we will change this to fetch from a config file but just a test for now
    let email = "mrlaith44@gmail.com"; // we will change this to fetch from a config file but just a test for now
    let mut commit:Vec<String> = Vec::new();
    let current_branch = &filetostring("./.litt/HEAD").unwrap_or_default()[0];




    commit.push(format!("tree <{}>",sha_root));
    if !first{
        let prev_commit_hash = filetostring(format!("./.litt/refs/heads/{}", current_branch).as_str()).unwrap();
        commit.push(format!("parent <{}>",prev_commit_hash[0]));
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


// Extract the commit to a folder for now
pub fn checkout_commit(phash:String) {
    // let head = filetostring("./.litt/refs/heads/master").unwrap();
    let hash = find_full_hash(&phash).unwrap_or_default();
    println!("{}",hash.red());

    let data = parse_commit_data(hash).unwrap_or_default();
    let root_tree_hash = data.get("tree").unwrap().to_string();
    // let parent = data.get("parent").map(|s| s.to_string()).unwrap_or(String::from(""));
    // let first_commit = data.contains_key("parent");

    let root_path = PathBuf::from("./COMMIT");
    let _ = fs::create_dir_all(&root_path); 

    // Begin recursive tree walking
    treewalker(root_tree_hash, root_path);
}

fn treewalker(tree_hash: String, current_path: PathBuf) {
    let tree_entries = parse_tree_object(tree_hash);

    for entry in tree_entries {
        let entry_path = current_path.join(&entry.name);

        if entry.entry_type == "blob" {
            println!("Created file: {:?}", entry_path);

            fileops::decompressfile(&format!("./.litt/objects/{}", entry.hash), entry_path.to_str().unwrap()).unwrap();
        } else if entry.entry_type == "tree" {

            fileops::mkdir(entry_path.to_str().unwrap());
            println!("Created directory: {:?}", entry_path);
            treewalker(entry.hash, entry_path);
        }
    }
}

fn parse_commit_data(hash:String) -> Result<HashMap<String, String>, &'static str> {
    let mut parsed_data = HashMap::new();
    let lines = filetostring(&format!("./.litt/objects/{}",hash)).unwrap();
    for line in lines {
        if line.starts_with("tree") {
            // Extract the tree hash
            if let Some(hash) = line.split_whitespace().nth(1) {
                parsed_data.insert("tree".to_string(), hash.trim_matches('<').trim_matches('>').to_string());
            }
        } else if line.starts_with("parent") {
            // println!("DEBUG {}",line);
            // Extract the parent hash
            if let Some(hash) = line.split_whitespace().nth(1) {
                // println!("DEBUG {}",hash);
                parsed_data.insert("parent".to_string(), hash.trim_matches('<').trim_matches('>').to_string());
            }
        } else if line.starts_with("author") {
            // Extract author information: name, email, timestamp, timezone
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                // Extract name and email within "<>"
                let mut name = String::new();
                let mut email = String::new();
                for part in &parts[1..] {
                    if part.starts_with('<') && part.ends_with('>') {
                        email = part.trim_matches('<').trim_matches('>').to_string();
                        break;
                    } else {
                        name.push_str(part);
                        name.push(' ');
                    }
                }
                name = name.trim().to_string();

                // Extract timestamp and timezone
                let timestamp = parts[parts.len() - 2];
                let timezone = parts[parts.len() - 1];

                parsed_data.insert("author_name".to_string(), name);
                parsed_data.insert("author_email".to_string(), email);
                parsed_data.insert("commit_time".to_string(), timestamp.to_string());
                parsed_data.insert("timezone".to_string(), timezone.to_string());
            }
        }
    }

    if parsed_data.is_empty() {
        Err("No data found in input")
    } else {
        Ok(parsed_data)
    }
}




fn parse_tree_object(hash:String) -> Vec<Treeobj> {
    let mut entries = Vec::new();
    let lines = filetostring(&format!("./.litt/objects/{}",hash)).unwrap();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let entry = Treeobj {
                entry_type: parts[0].to_string(),
                hash: parts[1].to_string(),
                name: parts[2].to_string(),
            };
            entries.push(entry);
        }
    }

    entries
}


