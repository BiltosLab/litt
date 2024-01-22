/*
    This file contains all the functions that do operations of files 
*/
use std::{fs,fs::{File, OpenOptions, DirBuilder},io::{Write,BufRead, self, BufReader, BufWriter},io::Read};
use sha2::{Sha256,Digest};
use std::borrow::BorrowMut;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;


pub fn filetostring(filetoparse:&str) -> Result<Vec<String>, io::Error>{ // Function to Parse files line by line into a Vec<String>
    let mut f = File::open(filetoparse)?;
    let mut tokens:Vec<String> = vec![];
    let mut reader = io::BufReader::new(f);

    for line in reader.lines(){
        let line = line?;
        tokens.push(line.to_string());
    }
    Ok(tokens)
}


pub fn stringtofile(filepath:&str,content:Vec<String>) -> Result<(), std::io::Error> { // this one will truncate or overwrite the entire file 
    let content = content.join("\n");
    fs::write(filepath, content)
}


// this one will not delete any content it'll just append it to the file which is what i need for commit history :D
// It does an entire Vector
pub fn appendv_to_file(file_path: &str, lines: Vec<String>) -> io::Result<()> { 
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) 
        .open(file_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
// this one will not delete any content it'll just append it to the file which is what i need for commit history :D
// Also this one does 1 string only at a time
pub fn appendstr_to_file(file_path: &str, line: String) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}

pub fn search_and_destroy(file_path: &str, string_to_delete: &str) -> Result<(), std::io::Error>  {
    let filecontent = BufReader::new(File::open(file_path)?);
    let mut newfile: Vec<String> = Vec::new();

    for line in filecontent.lines() {
        let line = line?;
        if !line.contains(string_to_delete) {
            newfile.push(line);
        };
    }

    let mut f = File::create(file_path)?;
    for line in newfile {
        writeln!(f, "{}", line)?;
    }

    Ok(())
}

pub fn scanfiles_andignore(realpath:&str,ignore:&Vec<String>) -> Vec<String> { 
    let mut filelist:Vec<String> = Vec::new();
    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){ 
                    if metta.is_dir(){
                        if ignore.contains(path.file_name().to_string_lossy().to_string().borrow_mut()) {continue;}
                        filelist.extend(scanfiles_andignore(&path.path().to_string_lossy(),ignore));
                    }
                    else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());
                        filelist.push(path.path().to_str().unwrap().to_string());
                        //println!("{:?}",path.path().to_str().unwrap());
                    }
                }
                
            }
            
        }
    }


    return filelist;
}

pub fn scanfiles(realpath:&str) -> Vec<String> { 
    let mut filelist:Vec<String> = Vec::new();
    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){ 
                    if metta.is_dir(){
                        filelist.extend(scanfiles(&path.path().to_string_lossy()));
                    }
                    else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());
                        filelist.push(path.path().to_str().unwrap().to_string());
                        //println!("{:?}",path.path().to_str().unwrap());
                    }
                }
                
            }
            
        }
    }


    return filelist;
}

/*
Tbh we will need a config file for this application which will be stored in .config i suppose
it will store the name and email so we can parse that file for that info ,
the commit message will first check if the file is empty then if not we will proceed with the committing part
parse the commit message and take that info from it 
we will need a subroutine for creating an obj for each type aka create_treeobj create_commitobj etc
tags will be skipped till i know what they do .

*/

pub fn create_object(objtype:&str)
{
    // match first for objtype blob,tree,commit,tag 
    // we will add the type to the very top of the object file
    // then add neccessary content wether its the commit author parent message or whatever 
    // after finishing it up we hash it then compress it and give it a filename of the hash we got 
    // preferably we mimic how git does it by removing the first 2 letters and using it as the 
    //folder name then the rest of it will be the file name inside that folder
    // tree objects have the following format <File Permissions> <File type> <SHA-256 HASH> <Filename>
    let mut objfile:Vec<String>= Vec::new();
    match objtype {
        "blob" => {objfile.push("BLOB".to_string());
    },
        "tree" => {objfile.push("TREE".to_string());
    },
        "commit" => {objfile.push("COMMIT".to_string());
    },
        _ => println!("Unknown command: {}", objtype),
    }
    if let Err(err) = appendv_to_file(compute_vec_hash(&objfile).as_str(),objfile){
        eprintln!("Error {}",err);
    }
}

pub fn computehash(file: &str) -> Result<String, io::Error> { // Need to change this to return result instead but its fine for testing i guess
    // Open the file
    let mut file = File::open(file).unwrap();

    // Create a SHA-256 "hasher"
    let mut hasher = Sha256::new(); // Rust analyzer thinks this's an error or something just ignore.

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
/*
while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    } 
    // another implementation for that loop i feel this is better
    // but w.e we can take care of it later current loop causes no trouble till we deal with the
    // unwrap stuff.
    */



    
    // Finalize the hash and get the result as a byte array
    let result = format!("{:x}", hasher.finalize());
    Ok(result)
}

pub fn compute_vec_hash(content: &Vec<String>) -> String {
    let mut hasher = Sha256::new();

    for line in content {
        hasher.update(line);
    }

    format!("{:x}", hasher.finalize())
}


pub fn mkdir(path:&str) { 
    DirBuilder::new()
    .recursive(true)
    .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    
}

pub fn touch(path:&str) {
let mut f = File::create(path).unwrap();
assert!(fs::metadata(path).unwrap().is_file());

}


pub fn decompressfile(inputfile:&str,outputfile:&str) -> std::io::Result<()>{
    let input_file = File::open(inputfile)?;
    let output_file = File::create(outputfile)?;
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut decoder = DeflateDecoder::new(&mut reader);
    std::io::copy(&mut decoder, &mut writer)?;
    Ok(())
}


pub fn compressfile(inputfile:&str,outputfile:&str) -> std::io::Result<()> {
    let input_file = File::open(inputfile)?;
    let output_file = File::create(outputfile)?;
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut encoder = DeflateEncoder::new(&mut writer, Compression::default());
    std::io::copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}
