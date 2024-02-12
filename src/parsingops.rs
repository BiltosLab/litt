use crate::filetostring;
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

pub fn indexparser() -> Vec<String>{
    filetostring("./.litt/index").unwrap()
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