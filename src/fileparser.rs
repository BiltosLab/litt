use std::{fs::File,io::Write,io::Read};


fn parsefile(filetoparse:&str){
    let mut f = File::open(filetoparse).unwrap();
    f.read_to_string(buf)

}