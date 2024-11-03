use core::panic;
use std::collections::HashMap;
use std::process::exit;
use std::{cmp::Ordering, ops::Index, string::ParseError};
use std::fmt::format;
use colored::Colorize;

use crate::{filetostring, fileops::stringtofile};
use crate::fileops::compute_vec_hash;

#[derive(Debug)]
pub struct IndexHeader {
  signature:String,
  version:i8,
  entries:u64,
}
impl Default for IndexHeader {
    fn default() -> Self {
      Self {
      signature:"".to_string(),
      version:0,
      entries:0,
    }
    }
}
#[derive(Debug)]
pub struct IndexChecksum {
  checksum:bool,
  sha:String,
}
impl Default for IndexChecksum {
    fn default() -> Self {
      Self {
      checksum:false,
      sha:"".to_string(),
    }
    }
}
#[derive(Debug)]
#[derive(Clone)]
pub struct IndexEntry {
    pub entry_number: u32,
    pub ctime: f64,
    pub mtime: f64,
    pub dev: u32,
    pub ino: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub sha: String,
    pub flags: u16,
    pub assume_valid: bool,
    pub extended: bool,
    pub stage: (bool, bool),
    pub name: String,
}

impl Default for IndexEntry {
    fn default() -> Self {
        Self {
            entry_number: 0,
            ctime: 0.0,
            mtime: 0.0,
            dev: 0,
            ino: 0,
            mode: 0,
            uid: 0,
            gid: 0,
            size: 0,
            sha: "".to_string(),
            flags: 0,
            assume_valid: false,
            extended: false,
            stage: (false, false),
            name: "".to_string(),
        }
    }
}
impl PartialEq for IndexEntry {
  fn eq(&self, other: &Self) -> bool {
      self.ctime == other.ctime &&
      self.mtime == other.mtime &&
      self.dev == other.dev &&
      self.ino == other.ino &&
      self.mode == other.mode &&
      self.uid == other.uid &&
      self.gid == other.gid &&
      self.size == other.size &&
      self.sha == other.sha &&
      self.flags == other.flags &&
      self.assume_valid == other.assume_valid &&
      self.extended == other.extended &&
      self.stage == other.stage &&
      self.name == other.name
  }
}


pub fn index_parser() -> (IndexHeader, Vec<IndexEntry>, IndexChecksum){
  let mut entries: Vec<IndexEntry> = Vec::new(); // WE SHOULD ADD ENTRIES TO THIS AND THEN WHEN WRITING WE PUSH THIS TO THE FILE.
  let mut indexheader:IndexHeader = Default::default();
  let mut indexchecksum:IndexChecksum = Default::default();

    let file=filetostring("./.litt/index").unwrap();
    let mut entry:Vec<String> = vec![];
    let mut header:Vec<String> = vec![];
    let mut checksum:Vec<String> = vec![];
    if file.get(0).expect("Failed to read file") == "[header]"{
    for k in 1..4 {header.push(file.get(k).unwrap().to_string())}
    indexheader=indexheaderparser(header).expect("Invalid Header");
    for mut i in 4..file.len(){
      if file.get(i).unwrap() == "[entry]"{
        i += 1;
        for j in i..i+15{
          entry.push(file.get(j).unwrap().to_string());
        }
        // dont we need to increment i by 15 here? since we parsed the entire entry in the above for loop
        // its prob a performance hit checking every index when we already parsed the entry no?
        i += 15; // Testing
        entries.push(indexentryparser(entry.clone()).unwrap());
      }
        else { continue }
      if file.get(i).unwrap() == "[checksum]"{
        i +=1;
        for i in i..file.len(){checksum.push(file.get(i).unwrap().to_string());}
        indexchecksum=indexchecksumparser(checksum).expect("Invalid");
        break;
      }

    }
  }
  else {
      eprintln!("Index File Corrupted?");
  }
    // entriestostring(entries.get(1).expect("Invalid"));
    // unsigned_byte_sort_structs(&mut entries);
    // println!("{:#?}",indexheader);
    // println!("{:#?}",entries);
    // println!("{:#?}",indexchecksum);
    (indexheader,entries,indexchecksum)

}

pub fn indexentryparser(entrystr:Vec<String>) -> Result<IndexEntry, ParseError> {
  let mut entry:IndexEntry = Default::default();
      for line in entrystr {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        let (key, value) = (parts[0].trim(), parts[1].trim());

        match key {
          "entry" => entry.entry_number = value.parse().expect("Failed to read entry"), // expect testing idk still learning rust.
          "ctime" => entry.ctime = value.parse().unwrap(),
          "mtime" => entry.mtime = value.parse().unwrap(),
          "dev" => entry.dev = value.parse().unwrap(),
          "ino" => entry.ino = value.parse().unwrap(),
          "mode" => entry.mode = value.parse().unwrap(),
          "uid" => entry.uid = value.parse().unwrap(),
          "gid" => entry.gid = value.parse().unwrap(),
          "size" => entry.size = value.parse().unwrap(),
          "sha" => entry.sha = value.to_string(),
          "flags" => entry.flags = value.parse().unwrap(),
          "assume-valid" => entry.assume_valid = value.parse().unwrap(),
          "extended" => entry.extended = value.parse().unwrap(),
          "stage" => {
              let stage_parts: Vec<&str> = value.split(',').collect();
              entry.stage = (stage_parts[0].parse().unwrap(), stage_parts[1].parse().unwrap());
          },
          "name" => entry.name = value.to_string(), 
          _ => {}
      }
      
      }
      Ok(entry)
}




/*
* Bit 0: 0 = unmodified, 1 = version from our branch
* Bit 1: 0 = unmodified, 1 = version from their branch
*/
fn entries_to_string_vec(entry:&IndexEntry) -> Vec<String> {
  let mut stringout:Vec<String> = Vec::new();
  stringout.push("[entry]".to_string());
  stringout.push(format!("  entry = {}",entry.entry_number));
  stringout.push(format!("  ctime = {}",entry.ctime));
  stringout.push(format!("  mtime = {}",entry.mtime));
  stringout.push(format!("  dev  = {}",entry.dev));
  stringout.push(format!("  ino = {}",entry.ino));
  stringout.push(format!("  mode = {}",entry.mode));
  stringout.push(format!("  uid = {}",entry.uid));
  stringout.push(format!("  gid = {}",entry.gid));
  stringout.push(format!("  size = {}",entry.size));
  stringout.push(format!("  sha = {}",entry.sha));
  stringout.push(format!("  flags = {}",entry.flags));
  stringout.push(format!("  assume-valid = {}",entry.assume_valid));
  stringout.push(format!("  extended = {}",entry.extended));
  stringout.push(format!("  stage = {},{}",entry.stage.0,entry.stage.1));
  stringout.push(format!("  name = {}",entry.name));

    stringout
}

// [header]
// signature = DIRC
// version = 3
// entries = 5

fn indexheader_to_string_vec(header:&IndexHeader,entries:i128) -> Vec<String> {
    let mut stringout:Vec<String> = Vec::new();
    stringout.push("[header]".to_string());
    stringout.push(format!("  signature = {}",header.signature));
    stringout.push(format!("  version = {}",header.version));
    stringout.push(format!("  entries = {}",entries.to_string()));
    stringout
}
// [checksum]
// checksum = True
// sha = 1ef0972eb948e6229240668effcb9c600fe5888d
fn indexchecksum_to_string_vec(index_checksum: IndexChecksum, checksum:String) -> Vec<String> {
    let mut stringout:Vec<String> = Vec::new();
    stringout.push("[checksum]".to_string());
    stringout.push(format!("  checksum = {}",index_checksum.checksum));
    stringout.push(format!("  sha = {}",checksum));
    stringout
}

fn indexheaderparser(header:Vec<String>) -> Result<IndexHeader, ParseError>{
  let mut index_header:IndexHeader = Default::default();
  for line in header{
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    let (key, value) = (parts[0].trim(), parts[1].trim());
    match key { 
      "signature" => index_header.signature=value.parse().unwrap(),
      "version" => index_header.version = value.parse().unwrap(),
      "entries" => index_header.entries = value.parse().expect("Failed at entries"),
      _ => {} // Ignore unknown keys
    }
  }
  Ok(index_header)
}

fn indexchecksumparser(checksumh:Vec<String>) -> Result<IndexChecksum, ParseError> {
  let mut indexchecksum:IndexChecksum = Default::default();
  for line in checksumh{
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    let (key, value) = (parts[0].trim(), parts[1].trim());
    match key { 
      "checksum" => indexchecksum.checksum = value.parse().unwrap(),
      "sha" => indexchecksum.sha = value.parse().unwrap(),
      _ => {} // Ignore unknown keys
    }
  }
  Ok(indexchecksum)
}


pub fn insert_new_index_entries(newentries: Vec<IndexEntry>, map: HashMap<String, String>) {
  let (mut indexheader, mut entries, mut indexchecksum) = index_parser();
  let mut diff_entries: Vec<IndexEntry> = vec![];
  if newentries.len() != map.len() {
      eprintln!("{}","BUGGED: Unequal length of new entries and map".red());
      eprintln!("New entries length: {}, Map length: {}", newentries.len(), map.len());
      exit(1);
  }

  if entries.is_empty() {
      entries.extend(newentries);
  } else {
      let mut existing_name_map: HashMap<&String, usize> = HashMap::new();
      let mut existing_hash_map: HashMap<&String, usize> = HashMap::new();
      for (index, entry) in entries.iter().enumerate() {
          existing_name_map.insert(&entry.name, index);
          existing_hash_map.insert(&entry.sha, index);
      }

      let mut modifications = Vec::new();
      for new_entry in &newentries {
          if let Some(&existing_index) = existing_name_map.get(&new_entry.name) {
              let existing_entry = &entries[existing_index];
              if existing_entry == new_entry {
                  continue;
              } else {
                  modifications.push((existing_index, new_entry.clone()));
              }
          } else if let Some(&existing_index) = existing_hash_map.get(&new_entry.sha) {
              let existing_entry = &entries[existing_index];
              if existing_entry.name != new_entry.name {
                  diff_entries.push(new_entry.clone());
              }
          } else {
              diff_entries.push(new_entry.clone());
          }
      }
      for (index, new_entry) in modifications {
          entries[index] = new_entry;
      }
      entries.extend(diff_entries);
  }

  // DEBUG
  println!("Entries before deletion: {:#?}", entries);

  
  entries.retain(|entry| {
      let exists = map.values().any(|path| path == &entry.name);
      if !exists {
          // DEBUG
          println!("Removing deleted file from entries: {}", entry.name);
      }
      exists
  });


  unsigned_byte_sort_structs(&mut entries);
  if let Err(e) = stringtofile("./.litt/index", stitch_index_file(indexheader, entries.clone(), indexchecksum)) {
      eprintln!("Error writing to index file: {}", e);
  } else {
      println!("Successfully wrote to index file.");
  }
}





fn stitch_index_file(index_header: IndexHeader,entries :Vec<IndexEntry>,index_checksum: IndexChecksum) -> Vec<String>{
    let mut mainfile:Vec<String> = Vec::new();
    let mut strentires:Vec<String> = Vec::new();
    let entlen = entries.len();
    strentires.extend(indexheader_to_string_vec(&index_header, entlen as i128));
    for entry in entries {
        strentires.extend(entries_to_string_vec(&entry));
    }
    strentires.extend(indexchecksum_to_string_vec(index_checksum,compute_vec_hash(&strentires)));

    strentires
}



fn unsigned_byte_sort_structs(entries: &mut Vec<IndexEntry>) {
  entries.sort_unstable_by(|a, b| {
      // 1. Compare names using unsigned byte comparison
      match a.name.as_bytes().cmp(b.name.as_bytes()) {
          Ordering::Equal => {
              // 2. If names are equal, compare stage.0 using bool as u8
              let a_stage0 = a.stage.0 as u8;
              let b_stage0 = b.stage.0 as u8;
              match a_stage0.cmp(&b_stage0) {
                  Ordering::Equal => {
                      // 3. If stage.0 are equal, compare stage.1 using bool as u8
                      let a_stage1 = a.stage.1 as u8;
                      let b_stage1 = b.stage.1 as u8;
                      a_stage1.cmp(&b_stage1)
                  }
                  // 4. Otherwise, return stage.0 comparison result
                  other => other,
              }
          }
          // 5. Otherwise, return the name comparison result
          other => other,
      }
  });
    let mut i = 0;
    for entry in entries {
        i += 1;
        entry.entry_number = i;
    }
}

