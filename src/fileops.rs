use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::canonicalize;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::path::MAIN_SEPARATOR;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use crate::parsingops::{index_parser, insert_new_index_entries};
use colored::Colorize;
#[cfg(not(target_os = "windows"))]
use std::os::unix::fs::MetadataExt;

use crate::parsingops::IndexEntry;

pub fn filetostring(filetoparse: &str) -> Result<Vec<String>, io::Error> {
    let mut f = File::open(filetoparse)?;
    let mut tokens: Vec<String> = vec![];
    let mut reader = io::BufReader::new(f);

    for line in reader.lines() {
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

    if path.starts_with(".") {
        // Only code that works from AI :D eno wow
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

pub fn stringtofile(filepath: &str, content: Vec<String>) -> Result<(), std::io::Error> {
    // this one will truncate or overwrite the entire file
    let content = content.join("\n");
    fs::write(filepath, content)
}

pub fn scanfiles_and_ignoremt(realpath: &str, ignorefiles: bool) -> Vec<String> {
    // Get the ignore list
    let ignore: HashSet<String> = if ignorefiles {
        littignore().unwrap() // Load the ignore list normally
    } else {
        HashSet::new() // Create an empty HashSet if ignore is false
    };

    // Shared filelist using Arc and Mutex for thread-safe access
    let filelist = Arc::new(Mutex::new(Vec::new()));

    // Vector to hold thread handles
    let mut handles = vec![];

    // Check the directory
    if let Ok(dirf) = fs::read_dir(realpath) {
        for path in dirf {
            if let Ok(path) = path {
                let filelist = Arc::clone(&filelist);
                let ignore = ignore.clone(); // Clone the ignore list for each thread

                let handle = thread::spawn(move || {
                    if let Ok(metta) = path.metadata() {
                        let path_str = path.path().to_str().unwrap().to_string();

                        // Skip if the path is in the ignore list
                        if ignore
                            .iter()
                            .any(|ignore_path| path_str.contains(ignore_path))
                        {
                            return;
                        }

                        if metta.is_dir() {
                            // Recurse into subdirectories in a separate thread
                            let sublist =
                                scanfiles_and_ignoremt(&path.path().to_string_lossy(), ignorefiles);
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
    //let _ = stringtofile("DEBUGSCANMT.txt",filelist.clone());
    filelist
}
// Result<(HashMap<String, String>, Vec<IndexEntry>), io::Error> 


pub fn scan_for_staging(realpath: &str, ignorefiles: bool) -> (Vec<String>, HashMap<String, String>) {
    // Get the ignore list
    let ignore: HashSet<String> = if ignorefiles {
        littignore().unwrap() // Load the ignore list normally
    } else {
        HashSet::new() // Create an empty HashSet if ignore is false
    };

    // Parse the index to retrieve the existing index entries
    let (_header, index_entries, _checksum) = index_parser();

    // Convert index_entries to a HashMap for quick lookups by file name
    let index_map: HashMap<String, String> = index_entries
        .into_iter()
        .map(|entry| (entry.name, entry.sha))
        .collect();

    // Shared filelist and file_hash_map using Arc and Mutex for thread-safe access
    let filelist = Arc::new(Mutex::new(Vec::new()));
    let file_hash_map = Arc::new(Mutex::new(HashMap::new()));

    // Vector to hold thread handles
    let mut handles = vec![];

    // Check the directory
    if let Ok(dirf) = fs::read_dir(realpath) {
        for path in dirf {
            if let Ok(path) = path {
                let filelist = Arc::clone(&filelist);
                let file_hash_map = Arc::clone(&file_hash_map);
                let ignore = ignore.clone();
                let index_map = index_map.clone();

                let handle = thread::spawn(move || {
                    if let Ok(metta) = path.metadata() {
                        // Get the normalized absolute path
                        let path_str = path.path().to_str().unwrap().to_string();

                        // Skip if the path is in the ignore list
                        if ignore.iter().any(|ignore_path| path_str.contains(ignore_path)) {
                            return;
                        }

                        if metta.is_dir() {
                            // Recurse into subdirectories in a separate thread
                            let sublist = scan_for_staging(&path.path().to_string_lossy(), ignorefiles);
                            let mut filelist_lock = filelist.lock().unwrap();
                            filelist_lock.extend(sublist.0); // Append filelist from subdirectory scan

                            let mut file_hash_map_lock = file_hash_map.lock().unwrap();
                            file_hash_map_lock.extend(sublist.1); // Append file_hash_map from subdirectory scan
                        } else if metta.is_file() {
                            // Compute the file's hash
                            let file_hash = match computehashmt(&path_str) {
                                Ok(hash) => hash,
                                Err(e) => {
                                    eprintln!("Error computing hash for file {}: {}", path_str, e);
                                    return;
                                }
                            };

                            // Debug print for path and hash
                            println!("Checking file: {}", path_str);
                            println!("Computed hash: {}", file_hash);

                            // Store the <file_hash, path_str> in file_hash_map
                            let mut file_hash_map_lock = file_hash_map.lock().unwrap();
                            file_hash_map_lock.insert(file_hash.clone(), path_str.clone());

                            // Check if the file is in index_map and if its hash matches
                            let should_add = match index_map.get(&path_str) {
                                Some(existing_hash) => {
                                    println!("Found in index with hash: {}", existing_hash);
                                    existing_hash != &file_hash // Add if hashes differ
                                },
                                None => true, // Add if not in index
                            };

                            // Debug print for the decision to add or not
                            println!("Should add: {}", should_add);

                            // Add file to filelist if necessary
                            if should_add {
                                let mut filelist_lock = filelist.lock().unwrap();
                                filelist_lock.push(path_str);
                            }
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

    // Unwrap Arc Mutex to get the final lists
    let filelist = Arc::try_unwrap(filelist).unwrap().into_inner().unwrap();
    let file_hash_map = Arc::try_unwrap(file_hash_map).unwrap().into_inner().unwrap();

    // Return both filelist and file_hash_map
    (filelist, file_hash_map)
}


pub fn scan_objects(hash: &str) -> String {
    if let Ok(dirf) = fs::read_dir("./.litt/objects") {
        for path in dirf {
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata() {
                    if metta.is_dir() {
                        //if ignore.contains(path.file_name().to_string_lossy().to_string().borrow_mut()) {continue;}
                        continue;
                    } else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());
                        if path
                            .file_name()
                            .to_str()
                            .unwrap()
                            .to_string()
                            .contains(hash)
                        {
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
pub fn compress_files_in_parallel(file_paths: Vec<String>,final_hash_map:HashMap<String, String>) -> Result<Vec<IndexEntry>, io::Error> {
    let mut handles = vec![];

    // let file_hash_map = Arc::new(Mutex::new(HashMap::new()));
    let file_info_vec = Arc::new(Mutex::new(Vec::new()));
    let file_paths = Arc::new(file_paths);

    for inputfile in &*file_paths {
        let inputfile = inputfile.clone();
        // let file_hash_map = Arc::clone(&file_hash_map);
        let file_info_vec = Arc::clone(&file_info_vec);

        let handle = thread::spawn(move || {
            // Compute hash and directly use it to populate both file_hash_map and file_info_vec
            match computehashmt(&inputfile) {
                Ok(outputfile) => {
                    // {
                    //     let mut hash_map = file_hash_map.lock().unwrap();
                    //     hash_map.insert(outputfile.clone(), inputfile.clone());
                    // }

                    // Compress the file using the computed hash as the output filename
                    if let Err(e) =
                        compressfile(&inputfile, &format!("./.litt/objects/{}", outputfile))
                    {
                        eprintln!("Error compressing file {}: {}", inputfile, e);
                    } else {
                        println!("Successfully compressed {} to {}", inputfile, outputfile);
                    }

                    // Pass the computed hash to `extract_file_info` immediately and update both
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

    // let final_hash_map = Arc::try_unwrap(file_hash_map)
    //     .unwrap()
    //     .into_inner()
    //     .unwrap();

    let final_file_info_vec = Arc::try_unwrap(file_info_vec)
        .unwrap()
        .into_inner()
        .unwrap();

    // // DEBUG
    // let mut temp_vec: Vec<String> = vec![];
    // for i in &final_file_info_vec {
    //     println!("FILEINFOVEC {:#?}", i);
    //     temp_vec.push(format!("{:#?}", i));
    // }
    // temp_vec.push("-------------------------------------------------------------------------------------------------------------------".to_string());

    // for i in &final_hash_map {
    //     println!("HASHMAP {:#?}", i);
    //     temp_vec.push(format!("{:#?}", i));
    // }

    // let _ = stringtofile("FILEDEBUG.txt", temp_vec);

    insert_new_index_entries(final_file_info_vec.clone(), final_hash_map.clone());

    Ok((final_file_info_vec))
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

pub fn compute_vec_hash(content: &Vec<String>) -> String {
    let mut hasher = Sha256::new();

    for line in content {
        hasher.update(line);
    }

    format!("{:x}", hasher.finalize())
}

pub fn mkdir(path: &str) {
    DirBuilder::new().recursive(true).create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
}

pub fn touch(path: &str) {
    let mut f = File::create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_file());
}

pub fn decompressfile(inputfile: &str, outputfile: &str) -> std::io::Result<()> {
    let input_file = File::open(inputfile)?;
    let output_file = File::create(outputfile)?;
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut decoder = DeflateDecoder::new(&mut reader);
    std::io::copy(&mut decoder, &mut writer)?;
    Ok(())
}

pub fn compressfile(inputfile: &str, outputfile: &str) -> std::io::Result<()> {
    let input_file = File::open(inputfile)?;
    let output_file = File::create(outputfile)?;
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);
    let mut encoder = DeflateEncoder::new(&mut writer, Compression::default());
    std::io::copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

pub fn find_full_hash(partial_hash: &str) -> Result<String, io::Error> {
    if partial_hash.len() < 7 {
        eprintln!(
            "{}",
            "Partial hash must be at least 7 characters long".red()
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Partial hash must be at least 7 characters long",
        ));
    }

    // Retrieve all object files from the directory
    let object_files = scanfiles_and_ignoremt("./.litt/objects/", false);
    //println!("{:#?}",object_files);
    // Search for a file whose name starts with the partial hash
    for file_path in object_files {
        if let Some(file_name) = file_path.split('/').last() {
            if file_name.starts_with(partial_hash) {
                return Ok(file_name.to_string());
            }
        }
    }

    // If no match is found, return an error
    eprintln!(
        "{}",
        "No matching hash found please enter a valid hash for a commit".red()
    );
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No matching hash found",
    ))
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

pub fn blob(filename: &str) {
    let a = computehashmt(filename).unwrap();
    compressfile(filename, ("./.litt/objects/".to_owned() + &a).as_str()).unwrap();
}

pub fn catfile(hashoffile: &str) {
    let obj = scan_objects(hashoffile);
    println!("{}", obj);
    decompressfile(&obj, "./.litt/tempf").unwrap();
    println!(
        "{}",
        filetostring("./.litt/tempf").unwrap().join("\n").blue()
    );
}

pub fn split_path(path: &str) -> Vec<&str> {
    path.split(MAIN_SEPARATOR).collect()
}
