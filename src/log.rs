use crate::fileops::*;


pub fn log() {
    // parsingops::test_indextemp();
    // println!("IN LOG EXECUTED SUCCESSFULLY!");

    let mut count = 0;
    let mut a = scanfiles_and_ignoremt(".");
    for i in &mut a {
        count+=1;
        println!("{}", i);
    }
    println!("Found {} files", count);
}