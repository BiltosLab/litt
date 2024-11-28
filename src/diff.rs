use colored::*;


// This is not good code, need to rewrite later.
pub fn find_diff_lines(original_file: Vec<String>, modified_file: Vec<String>) -> Vec<String> { 
    // need to make this give the location of the diff later on
    let mut differences: Vec<String> = Vec::new();

    let max_lines = original_file.len().max(modified_file.len());
    for i in 0..max_lines {
        match (original_file.get(i), modified_file.get(i)) {
            (Some(line1), Some(line2)) if line1 == line2 => {
                // differences.push(format!("  {}", line1.blue()));
            }
            (Some(line1), Some(line2)) => {
                differences.push(format!("- {}", line1.red()));
                differences.push(format!("+ {}", line2.green()));
            }
            (Some(line1), None) => {
                differences.push(format!("- {}", line1.red()));
            }
            (None, Some(line2)) => {
                differences.push(format!("+ {}", line2.green()));
            }
            _ => unreachable!(), // This case should not happen due to max_lines
        }
    }
    // println!("Percentage of Similarity ? {:#?}%",similarity_percentage(original_file, modified_file));
    differences
}


// Func TODO First function here should do the following : We need to have a record of some sorts either index or a new way of doing it idk
// We go through each file we can access meaning its not in .littignore and compare the names first see which files are new and which are the same
// then we put the new files aside and compare the hash of existing files and their counterparts in the commit and then we print which are new and which got modified only.

pub fn similarity_percentage(original_file: Vec<String>, modified_file: Vec<String>) -> f32 {
    let max_lines = original_file.len().max(modified_file.len());
    let mut equal_line_count = 0;
    for i in 0..max_lines {
        match (original_file.get(i), modified_file.get(i)) {
            (Some(line1), Some(line2)) if line1 == line2 => {
                // differences.push(format!("  {}", line1.blue()));
                equal_line_count+=1;
            }
            (Some(_line1), Some(_line2)) => {
            }
            (Some(_line1), None) => {
            }
            (None, Some(_line2)) => {
            }
            _ => unreachable!(), // This case should not happen due to max_lines
        }
    }
    equal_line_count as f32/max_lines as f32*100.0
}



