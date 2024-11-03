/*
    This file contains all the functions that do operations of files 
*/
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::MAIN_SEPARATOR;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::{self, DirBuilder, File, OpenOptions}, io::{self, BufRead, BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}};
use std::time::SystemTime;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::MetadataExt;
use colored::Colorize;
use crate::parsingops::insert_new_index_entries;

use crate::parsingops::IndexEntry;
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

// TODO: after scanning all the files need to check if they were modified or not
// and check the files against the index and return another Vec<String> containing the modifed files only.
// .
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
    let _ = stringtofile("DEBUGSCANMT.txt",filelist.clone());
    filelist
}

pub fn scan_objects(hash:&str) -> String {
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


    "no".to_string()
}


/*
Tbh we will need a config file for this application which will be stored in .config i suppose
it will store the name and email so we can parse that file for that info ,
the commit message will first check if the file is empty then if not we will proceed with the committing part
parse the commit message and take that info from it 
we will need a subroutine for creating an obj for each type aka create_treeobj create_commitobj etc
tags will be skipped till i know what they do .

*/


#[cfg(windows)]
fn simulate_ino(file_path: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    file_path.hash(&mut hasher);
    hasher.finish()
}

pub fn extract_file_info(file_path: &str, sha: String) -> Result<IndexEntry, io::Error> {
    let metadata = fs::metadata(file_path)?;

    let ctime = metadata
        .created()
        .unwrap_or(SystemTime::now())
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let mtime = metadata
        .modified()
        .unwrap_or(SystemTime::now())
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    #[cfg(unix)]
    let dev = metadata.dev();
    #[cfg(windows)]
    let dev = 1;

    #[cfg(unix)]
    let ino = metadata.ino();
    #[cfg(windows)]
    let ino = simulate_ino(file_path);

    #[cfg(unix)]
    let mode = metadata.mode();
    #[cfg(windows)]
    let mode = if metadata.is_file() {
        0o100644
    } else if metadata.is_dir() {
        0o040755
    } else {
        0
    };

    #[cfg(unix)]
    let uid = metadata.uid();
    #[cfg(windows)]
    let uid = 0;

    #[cfg(unix)]
    let gid = metadata.gid();
    #[cfg(windows)]
    let gid = 0;

    let size = metadata.len();

    let entry = IndexEntry {
        entry_number: 1,
        ctime,
        mtime,
        dev: dev as u32,
        ino,
        mode: mode as u32,
        uid: uid as u32,
        gid: gid as u32,
        size,
        sha, // Use the hash provided as a parameter
        flags: 9,
        assume_valid: false,
        extended: false,
        stage: (false, false),
        name: file_path.to_string(),
    };

    Ok(entry)
}

pub fn compress_files_in_parallel(
    file_paths: Vec<String>
) -> Result<(HashMap<String, String>,Vec<IndexEntry>), io::Error> {
    let mut handles = vec![];

    let file_hash_map = Arc::new(Mutex::new(HashMap::new()));
    let file_info_vec = Arc::new(Mutex::new(Vec::new()));

    let file_paths = Arc::new(file_paths);

    for inputfile in &*file_paths {
        let inputfile = inputfile.clone();
        let file_hash_map = Arc::clone(&file_hash_map);
        let file_info_vec = Arc::clone(&file_info_vec);

        let handle = thread::spawn(move || {
            match computehashmt(&inputfile) {
                Ok(outputfile) => {
                    {
                        let mut hash_map = file_hash_map.lock().unwrap();
                        hash_map.insert(outputfile.clone(), inputfile.clone());
                    }

                    if let Err(e) = compressfile(&inputfile, &("./.litt/objects/".to_owned() + &outputfile)) {
                        eprintln!("Error compressing file {}: {}", inputfile, e);
                    } else {
                        println!("Successfully compressed {} to {}", inputfile, outputfile);
                    }

                    // Pass the computed hash to `extract_file_info`
                    match extract_file_info(&inputfile, outputfile.clone()) {
                        Ok(file_info) => {
                            let mut file_info_vec = file_info_vec.lock().unwrap();
                            file_info_vec.push(file_info);
                        }
                        Err(e) => {
                            eprintln!("Error extracting file info for {}: {}", inputfile, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error computing hash for file {}: {}", inputfile, e);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_hash_map = Arc::try_unwrap(file_hash_map)
        .unwrap()
        .into_inner()
        .unwrap();

    let final_file_info_vec = Arc::try_unwrap(file_info_vec)
        .unwrap()
        .into_inner()
        .unwrap();
    // DEBUG
    let mut temp_vec:Vec<String> = vec![];
    for i in &final_file_info_vec {
        println!("FILEINFOVEC {:#?}", i);
        temp_vec.push(format!("{:#?}",i));

    }
    temp_vec.push("-------------------------------------------------------------------------------------------------------------------".to_string());

    for i in &final_hash_map {
        println!("HASHMAP{:#?}", i);
        temp_vec.push(format!("{:#?}",i));
    }

    let _ = stringtofile("FILEDEBUG.txt", temp_vec);

    insert_new_index_entries(final_file_info_vec.clone(),final_hash_map.clone());

    Ok((final_hash_map,final_file_info_vec))
}

pub fn computehashmt(file: &str) -> Result<String, io::Error> {
    let input_file = File::open(file)?;
    let mut reader = BufReader::new(input_file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096]; // 4KB buffer

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

// pub fn computehashmt(file: &str) -> Result<String, io::Error> {
//     let input_file = File::open(file)?;
//     let reader = BufReader::new(input_file);

//     let chunk_size = 4096; // 4KB chunks
//     let mut handles = vec![];

//     let hash_results = Arc::new(Mutex::new(vec![]));

//     for chunk in reader.bytes().collect::<Result<Vec<u8>, _>>()?.chunks(chunk_size).map(|c| c.to_vec()) {
//         let chunk = Arc::new(chunk);
//         let hash_results = Arc::clone(&hash_results);

//         let handle = thread::spawn(move || {
//             let mut hasher = Sha256::new();
//             hasher.update(&*chunk);
//             let hash = format!("{:x}", hasher.finalize());

//             let mut results = hash_results.lock().unwrap();
//             results.push(hash);
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     let final_hash = {
//         let hash_results = hash_results.lock().unwrap();
//         let mut hasher = Sha256::new();
//         for hash in &*hash_results {
//             hasher.update(hash.as_bytes());
//         }
//         format!("{:x}", hasher.finalize())
//     };

//     Ok(final_hash)
// }

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


pub fn blob(filename:&str){
    let a = computehashmt(filename).unwrap();
    compressfile(filename, ("./.litt/objects/".to_owned()+&a).as_str()).unwrap();

}


pub fn catfile(hashoffile:&str){
    let obj= scan_objects(hashoffile);
    println!("{}",obj);
    decompressfile(&obj, "./.litt/tempf").unwrap();
    println!("{}",filetostring("./.litt/tempf").unwrap().join("\n").blue());

}



pub fn split_path(path: &str) -> Vec<&str> {
    path.split(MAIN_SEPARATOR).collect()
}



