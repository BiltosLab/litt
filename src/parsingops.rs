use std::string::ParseError;

use crate::filetostring;
#[derive(Debug)]
struct IndexEntry {
    entry_number: u32,
    ctime: f64,
    mtime: f64,
    dev: u32,
    ino: u64,
    mode: u32,
    uid: u32,
    gid: u32,
    size: u64,
    sha1: String,
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
          sha1: "".to_string(),
          flags: 0,
          assume_valid: false,
          extended: false,
          stage: (false, false),
          name: "".to_string(),
      }
  }
}

pub fn indexparser() -> Vec<String>{
  let mut entries: Vec<IndexEntry> = Vec::new();
    let file=filetostring("./.litt/index").unwrap();

    let mut a:Vec<String> = vec![];
    for mut i in 0..file.len(){
      if file.get(i).unwrap() == "[entry]"{
        i += 1;
        for j in i..i+15{
          a.push(file.get(j).unwrap().to_string());
        }
        entries.push(indexentryparser(a.clone()).unwrap());
      }
    }
    println!("{:#?}",entries);
    file

}

fn indexentryparser(entrystr:Vec<String>) -> Result<IndexEntry, ParseError> {
  let mut entry:IndexEntry = Default::default();
      for line in entrystr {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        let (key, value) = (parts[0].trim(), parts[1].trim());

        match key {
          "entry" => entry.entry_number = value.parse().unwrap(),
          "ctime" => entry.ctime = value.parse().unwrap(),
          "mtime" => entry.mtime = value.parse().unwrap(),
          "dev" => entry.dev = value.parse().unwrap(),
          "ino" => entry.ino = value.parse().unwrap(),
          "mode" => entry.mode = value.parse().unwrap(),
          "uid" => entry.uid = value.parse().unwrap(),
          "gid" => entry.gid = value.parse().unwrap(),
          "size" => entry.size = value.parse().unwrap(),
          "sha1" => entry.sha1 = value.to_string(), // Assuming sha1 is a string
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
  sha1 = d5f7fc3f74f7dec08280f370a975b112e8f60818
  flags = 9
  assume-valid = False
  extended = False
  stage = (False, False)
  name = added.txt

[checksum]
  checksum = True
  sha1 = 1ef0972eb948e6229240668effcb9c600fe5888d
   */