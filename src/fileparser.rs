use std::{fs,fs::{File, OpenOptions},io::{Write,BufRead, self, BufReader},io::Read};


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
pub fn appendstr_to_file(file_path: &str, line: String) -> io::Result<()> { 
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) 
        .open(file_path)?;
        writeln!(file, "{}", line)?;

    Ok(())
}


pub fn isfileincommit() -> bool {

    
    
    true
}

pub fn search_and_destroy(file_path: &str, line_to_delete: String) -> Result<(), std::io::Error> {
    let filecontent = filetostring(&file_path);
    let mut newfile:Vec<String> = Vec::new();

    for line in filecontent
    {
        if !line.contains(&line_to_delete){
            newfile.push(line);
        };
    }
    let mut f = File::create(file_path).unwrap();
    assert!(fs::metadata(file_path).unwrap().is_file());



    stringtofile(file_path,newfile)
}

/* 
pub fn search_and_destroy(file_path: &str, line_to_delete: &str) -> io::Result<()> {
    // Open the file in read mode
    let file_in = File::open(file_path)?;

    // Create a temporary file to write the modified content
    let temp_file_path = "temp_file.txt";
    let file_out = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_file_path)?;

    // Create a buffered reader for the input file
    let reader = BufReader::new(file_in);

    // Read lines into a Vec<String>
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Create a Vec<String> with lines excluding the one to delete
    let modified_lines: Vec<String> = lines
        .iter()
        .filter(|&line| line != line_to_delete)
        .cloned()
        .collect();

    // Create a buffered writer for the output file
    let mut writer = io::BufWriter::new(file_out);

    // Write the modified lines to the temporary file
    for line in &modified_lines {
        writeln!(writer, "{}", line)?;
    }

    // Rename the temporary file to the original file
    std::fs::rename(&temp_file_path, file_path)?;

    Ok(())
}
*/



/*pub fn search_and_destroy(file_path: &str, line_to_delete: String) -> io::Result<()> {
        // Open the file in read mode
        let file_in = File::open(file_path)?;
    
        // Create a temporary file to write the modified content
        let temp_file_path = "tempfiledonttouch";
        let file_out = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_file_path)?;
    
        // Create a buffered reader for the input file
        let reader = BufReader::new(file_in);
    
        // Create a buffered writer for the output file
        let mut writer = io::BufWriter::new(file_out);
    
        // Iterate through the lines, skipping the one to delete
        for line in reader.lines() {
            let current_line = line?;
            if current_line != line_to_delete {
                writeln!(writer, "{}", current_line)?;
            }
        }
    
        // Rename the temporary file to the original file
        std::fs::rename(&temp_file_path, file_path)?;
    
        Ok(())
    }
    */

