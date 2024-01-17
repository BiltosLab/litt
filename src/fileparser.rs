use std::{fs::File,io::{BufRead, self},io::Read};


pub fn filetostring(filetoparse:&str) -> Vec<String>{ // Function to Parse files line by line into a Vec<String>
    let mut f = File::open(filetoparse).unwrap();
    let mut tokens:Vec<String> = vec![];
    let mut reader = io::BufReader::new(f);

    for line in reader.lines(){
        let line = line.unwrap();
        tokens.push(line.to_string());
    }
    return tokens;
}







