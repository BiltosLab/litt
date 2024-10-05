/*
    This file contains all the functions that do operations of files 
*/
use std::{fs::{self, DirBuilder, File, OpenOptions},io::{self, BufRead, BufReader, BufWriter, Read, Write},path::{Path, PathBuf}};
use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::thread;
use sha2::{Sha256,Digest};
use std::borrow::BorrowMut;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use flate2::read::DeflateDecoder;
use std::collections::{HashMap, HashSet};

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

/*pub fn littignore() -> Result<Vec<String>, io::Error> {
    let file = filetostring("./.littignore")?;
    Ok(file)
}*/
fn path_fix(path: String) -> String {
    let mut path = path;

    if path.starts_with(".") { // Only code that works from AI :D eno wow
        path = path[1..].to_string();
    }
    if path.contains("/") {
        path = path.replace("/", r"\");
    }
    
    path
}
#[cfg(target_os = "windows")]
pub fn littignore() -> Result<HashSet<String>, io::Error> { 
    let mut newfile: Vec<String> = vec![];
    for path in filetostring(".littignore")? {
        newfile.push(path_fix(path));
    }
    let mut hashset_of_strings: HashSet<_> = newfile.into_iter().collect(); 
    hashset_of_strings.insert(".litt".to_string());
    Ok(hashset_of_strings)
}
#[cfg(not(target_os = "windows"))]
pub fn littignore() -> Result<HashSet<String>, io::Error> { 
    let file = filetostring("./.littignore")?; 
    let mut hashset_of_strings: HashSet<_> = file.into_iter().collect(); 
    hashset_of_strings.insert("/.litt".to_string());
    Ok(hashset_of_strings)
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

// pub fn scanfiles_and_ignore(realpath:&str) -> Vec<String> {
//     let ignore = littignore().unwrap();
//     let mut filelist:Vec<String> = Vec::new();
//
//     if let Ok(dirf) = fs::read_dir(realpath)
//     {
//         for path in dirf{
//             if let Ok(path) = path {
//                 if let Ok(metta) = path.metadata(){
//                     let path_str = path.path().to_str().unwrap().to_string();
//                     if ignore.iter().any(|ignore_path| path_str.contains(ignore_path)) {
//                         continue;
//                     }
//                     if metta.is_dir(){
//                         //if ignore.contains(path.file_name().to_string_lossy().to_string().borrow_mut()) {continue;}
//                         filelist.extend(scanfiles_and_ignore(&path.path().to_string_lossy()));
//                     }
//                     else if metta.is_file() {
//                         //println!("{:?}",path.path().to_str().unwrap());
//                         filelist.push(path.path().to_str().unwrap().to_string());
//                         //println!("{:?}",path.path().to_str().unwrap());
//                     }
//                 }
//
//             }
//
//         }
//     }
//
//
//     filelist
// }

pub fn scanfiles_and_ignoremt(realpath: &str) -> Vec<String> {
    // Get the ignore list
    let ignore = littignore().unwrap();

    // Shared filelist using Arc and Mutex for thread-safe access
    let filelist = Arc::new(Mutex::new(Vec::new()));

    // Vector to hold thread handles
    let mut handles = vec![];

    // Check the directory
    if let Ok(dirf) = fs::read_dir(realpath) {
        for path in dirf {
            if let Ok(path) = path {
                let filelist = Arc::clone(&filelist);
                let ignore = ignore.clone();  // Clone the ignore list for each thread

                let handle = thread::spawn(move || {
                    if let Ok(metta) = path.metadata() {
                        let path_str = path.path().to_str().unwrap().to_string();

                        // Skip if the path is in the ignore list
                        if ignore.iter().any(|ignore_path| path_str.contains(ignore_path)) {
                            return;
                        }

                        if metta.is_dir() {
                            // Recurse into subdirectories in a separate thread
                            let sublist = scanfiles_and_ignoremt(&path.path().to_string_lossy());
                            let mut filelist_lock = filelist.lock().unwrap();
                            filelist_lock.extend(sublist);
                        } else if metta.is_file() {
                            // Add file to filelist
                            let mut filelist_lock = filelist.lock().unwrap();
                            filelist_lock.push(path.path().to_str().unwrap().to_string());
                        }
                    }
                });

                handles.push(handle);
            }
        }
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Return the final list of files
    let filelist = Arc::try_unwrap(filelist).unwrap().into_inner().unwrap();
    filelist
}

pub fn scanobjects(hash:&str) -> String { 
    if let Ok(dirf) = fs::read_dir("./.litt/objects")
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){ 
                    if metta.is_dir(){
                        //if ignore.contains(path.file_name().to_string_lossy().to_string().borrow_mut()) {continue;}
                        continue;
                    }
                    else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());
                            if  path.file_name().to_str().unwrap().to_string().contains(hash){
                                return path.path().to_str().unwrap().to_string();
                            }
                        //println!("{:?}",path.path().to_str().unwrap());
                    }
                }
                
            }
            
        }
    }


    return "no".to_string();
}


/*
Tbh we will need a config file for this application which will be stored in .config i suppose
it will store the name and email so we can parse that file for that info ,
the commit message will first check if the file is empty then if not we will proceed with the committing part
parse the commit message and take that info from it 
we will need a subroutine for creating an obj for each type aka create_treeobj create_commitobj etc
tags will be skipped till i know what they do .

*/

// pub fn compress_files_in_parallel(file_paths: Vec<String>) -> std::io::Result<()> {
//     let mut handles = vec![];
//
//     // Convert file_paths to Arc for safe multithreaded access (if needed)
//     let file_paths = Arc::new(file_paths);
//
//     for inputfile in &*file_paths {
//         let inputfile = inputfile.clone();
//         // Use the calculatehash function to determine the output file name
//         let outputfile = computehashmt(&inputfile)?;
//         // compressfile(filename, ("./.litt/objects/".to_owned()+&a).as_str()).unwrap();
//         // Spawn a new thread for each file compression task
//         let handle = thread::spawn(move || {
//             if let Err(e) = compressfile(&inputfile, ("./.litt/objects/".to_owned()+&outputfile).as_str()) {
//                 eprintln!("Error compressing file {}: {}", inputfile, e);
//             } else {
//                 println!("Successfully compressed {} to {}", inputfile, outputfile);
//             }
//         });
//
//         // Push the thread handle into the vector
//         handles.push(handle);
//     }
//
//     // Wait for all threads to finish
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     Ok(())
// }


pub fn compress_files_in_parallel(file_paths: Vec<String>) -> Result<HashMap<String, String>, io::Error> {
    let mut handles = vec![];

    // HashMap to store file path and its computed hash
    let file_hash_map = Arc::new(Mutex::new(HashMap::new()));

    // Convert file_paths to Arc for safe multithreaded access
    let file_paths = Arc::new(file_paths);

    for inputfile in &*file_paths {
        let inputfile = inputfile.clone();
        let file_hash_map = Arc::clone(&file_hash_map); // Clone Arc for thread-safe access to the HashMap

        // Spawn a new thread for each file compression task
        let handle = thread::spawn(move || {
            // Calculate the hash of the file
            match computehashmt(&inputfile) {
                Ok(outputfile) => {
                    // Add the file path and its computed hash to the HashMap
                    {
                        let mut hash_map = file_hash_map.lock().unwrap();
                        hash_map.insert(outputfile.clone(), inputfile.clone());
                    }

                    // Compress the file with the hash as part of the output file path
                    if let Err(e) = compressfile(&inputfile, &("./.litt/objects/".to_owned() + &outputfile)) {
                        eprintln!("Error compressing file {}: {}", inputfile, e);
                    } else {
                        println!("Successfully compressed {} to {}", inputfile, outputfile);
                    }
                }
                Err(e) => {
                    eprintln!("Error computing hash for file {}: {}", inputfile, e);
                }
            }
        });

        // Push the thread handle into the vector
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access and print the file hash map (optional)
    let final_hash_map = Arc::try_unwrap(file_hash_map)
        .unwrap()
        .into_inner()
        .unwrap();

    Ok(final_hash_map)
}
pub fn computehashmt(file: &str) -> Result<String, io::Error> {
    // Open the file
    let input_file = File::open(file)?;
    let reader = BufReader::new(input_file);

    // Buffer size and chunking parameters
    let chunk_size = 4096; // 4KB chunks
    let mut handles = vec![];

    // Shared vector for storing intermediate hash results
    let hash_results = Arc::new(Mutex::new(vec![]));

    // Iterate over the file in chunks
    for chunk in reader.bytes().collect::<Result<Vec<u8>, _>>()?.chunks(chunk_size).map(|c| c.to_vec()) {
        let chunk = Arc::new(chunk);
        let hash_results = Arc::clone(&hash_results);

        // Spawn a new thread for each chunk to compute its hash
        let handle = thread::spawn(move || {
            let mut hasher = Sha256::new();
            hasher.update(&*chunk);
            let hash = format!("{:x}", hasher.finalize());

            // Store the hash result in the shared vector
            let mut results = hash_results.lock().unwrap();
            results.push(hash);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Collect and combine the intermediate hashes
    let final_hash = {
        let hash_results = hash_results.lock().unwrap();
        let mut hasher = Sha256::new();
        for hash in &*hash_results {
            hasher.update(hash.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    };

    Ok(final_hash)
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


pub fn normalize_path(file_path: &str) -> PathBuf {
    // If the path is relative, convert it to an absolute path
    if !Path::new(file_path).is_absolute() {
        // Assuming the current working directory is the base directory
        let mut abs_path = std::env::current_dir().expect("Failed to get current directory");
        abs_path.push(file_path);
        //println!("Path was not abs{:#?}",abs_path);
        abs_path
    } else {
        // If the path is already absolute, keep it as is
        //println!("Path is already abs{:#?}",PathBuf::from(file_path));
        PathBuf::from(file_path)
        
    }
     
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists()
}