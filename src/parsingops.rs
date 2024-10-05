use core::panic;
use std::{cmp::Ordering, ops::Index, string::ParseError};
use std::fmt::format;
use crate::{filetostring, filestuff::stringtofile};
use crate::filestuff::compute_vec_hash;

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
pub struct IndexEntry {
    entry_number: u32,
    ctime: f64,
    mtime: f64,
    dev: u32,
    ino: u64,
    mode: u32,
    uid: u32,
    gid: u32,
    size: u64,
    sha: String,
    flags: u16,
    assume_valid: bool,
    extended: bool,
    stage: (bool, bool),
    name: String,
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

pub fn indexparser() -> (IndexHeader,Vec<IndexEntry>,IndexChecksum){ 
  let mut entries: Vec<IndexEntry> = Vec::new(); // WE SHOULD ADD ENTRIES TO THIS AND THEN WHEN WRITING WE PUSH THIS TO THE FILE.
  let mut indexheader:IndexHeader = Default::default();
  let mut indexchecksum:IndexChecksum = Default::default();

    let file=filetostring("./.litt/index").unwrap();
    let mut entry:Vec<String> = vec![];
    let mut header:Vec<String> = vec![];
    let mut checksum:Vec<String> = vec![];
    if file.get(0).expect("Failed to read file") == "[header]"{
    for k in 1..4 {header.push(file.get(k).unwrap().to_string())}
    indexheader=indexheaderparser(header).expect("Invalid");
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

fn indexentryparser(entrystr:Vec<String>) -> Result<IndexEntry, ParseError> {
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
          "sha" => entry.sha = value.to_string(), // Assuming sha is a string
          "flags" => entry.flags = value.parse().unwrap(),
          "assume-valid" => entry.assume_valid = value.parse().unwrap(),
          "extended" => entry.extended = value.parse().unwrap(),
          "stage" => {
              let stage_parts: Vec<&str> = value.split(',').collect();
              entry.stage = (stage_parts[0].parse().unwrap(), stage_parts[1].parse().unwrap());
          },
          "name" => entry.name = value.to_string(), // Assuming name is a string
          _ => {} // Ignore unknown keys
      }
      
      }
      Ok(entry)
}




/*
* Bit 0: 0 = unmodified, 1 = version from ours branch
* Bit 1: 0 = unmodified, 1 = version from theirs branch
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

// TODO now functions to take the 3 struct types 1 of header 1 of checksum and X number of Entry structs
// And convert them to a file so we can generate an entry and insert it to the index file
// also i want to compress and decompress index at use just to save more space.
/*
* Function to add an entry to index
* has to be A--Z or 1--9 Sorted.
* i think best option is to parse the entire file with indexentryparser and the rest the make the new index again
*/
/*
* How this function should work? First take an Vec<IndexEntry> of entires then append new entries to it
* Then sort them based on name ? or as git sorts it then we just rewrite new index
* With new hash at the end?
*/

pub fn test_indextemp () {
    let mut entries: Vec<IndexEntry> = vec![];
    let index_entry = IndexEntry {
        entry_number: 1,
        ctime: 1625097600.0, // Example UNIX timestamp for creation time
        mtime: 1625184000.0, // Example UNIX timestamp for modification time
        dev: 2049, // Example device number
        ino: 12345678, // Example inode number
        mode: 33188, // Example file mode (e.g., regular file with rw-r--r-- permissions)
        uid: 1000, // Example user ID
        gid: 1000, // Example group ID
        size: 1024, // Example file size in bytes
        sha: "9b4b51d8467ce19201efb8a9c9e63e8407bc646e".to_string(), // Example SHA-1 hash
        flags: 0b0010, // Example flags
        assume_valid: false, // Example validity flag
        extended: false, // Example extended flag
        stage: (false, true), // Example stage (e.g., added to the index)
        name: "example_file.txt".to_string(), // Example file name
    };
    entries.push(index_entry);
    insert_new_index_entries(entries);
    println!("EXECUTED SUCCESSFULLY !")
}
pub fn insert_new_index_entries(newentires:Vec<IndexEntry>){
    let (mut indexheader,mut entries,mut indexchecksum) = indexparser();
    entries.extend(newentires);
    unsigned_byte_sort_structs(&mut entries);
    stringtofile("./.litt/index",stitch_index_file(indexheader,entries,indexchecksum)).unwrap();



    //TODO after this basic thing just stitch everything back together into a new index file :D
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

/* This is how git index looks like and that's what we're going to mimic
[header]
  signature = DIRC
  version = 3
  entries = 5

[entry]
  entry = 1
  ctime = 1363549359.0
  mtime = 1363549359.0
  dev = 16777217
  ino = 1154043
  mode = 100644
  uid = 501
  gid = 20
  size = 6
  sha = d5f7fc3f74f7dec08280f370a975b112e8f60818
  flags = 9
  assume-valid = false
  extended = false
  stage = false,false
  name = added.txt
[entry]
  entry = 2
  ctime = 123124.0
  mtime = 12312312.0
  dev = 444412
  ino = 1154043
  mode = 111111
  uid = 244
  gid = 224
  size = 6
  sha = d5f7fc3f74f7dec08280f370a975b112e8f60818
  flags = 9
  assume-valid = false
  extended = false
  stage = false,false
  name = bobo.txt

[checksum]
  checksum = True
  sha = 1ef0972eb948e6229240668effcb9c600fe5888d
   */