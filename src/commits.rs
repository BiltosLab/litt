use std::{collections::HashSet, vec};
use colored::Colorize;

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
