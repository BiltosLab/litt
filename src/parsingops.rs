use crate::filetostring;


pub fn indexparser() -> Vec<String>{
    filetostring("./.litt/index").unwrap()
}