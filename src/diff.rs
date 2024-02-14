use colored::*;

pub fn find_diff_lines(file1: Vec<String>, file2: Vec<String>) -> Vec<String> { // need to make this give the location of the diff later on
    let mut differences: Vec<String> = Vec::new();

    let max_lines = file1.len().max(file2.len());

    for i in 0..max_lines {
        match (file1.get(i), file2.get(i)) {
            (Some(line1), Some(line2)) if line1 == line2 => {
                //differences.push(format!("  {}", line1));
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

    differences
}


