use colored::*;

<<<<<<< HEAD
pub fn find_diff_lines(file1: Vec<String>, file2: Vec<String>) -> Vec<String> {
    let mut differences: Vec<String> = Vec::new();

    let max_lines = file1.len().max(file2.len());

    for i in 0..max_lines {
        match (file1.get(i), file2.get(i)) {
            (Some(line1), Some(line2)) if line1 == line2 => {
                //differences.push(format!("  {}", line1));
            }
            (Some(line1), Some(line2)) => {
                differences.push(format!("- {}", line1));
                differences.push(format!("+ {}", line2));
            }
            (Some(line1), None) => {
                differences.push(format!("- {}", line1));
            }
            (None, Some(line2)) => {
                differences.push(format!("+ {}", line2));
            }
            _ => unreachable!(), // This case should not happen due to max_lines
        }
    }

    differences
}










/*
 for (i, (original_line, modified_line)) in original_lines.iter().zip(modified_lines).enumerate() {
        if original_line != modified_line {
            different_lines.push(i + 1); // Add 1 to index for human-readable line numbers
            linediffself.push(format!("-{}\t{}", i, modified_line).red().to_string());
            println!("{}",original_line.red());
            println!("{}",modified_line.red());
=======
// Failed to use any gpt to generate the function i really want :( so i have to write everything with my limited rust knowledge oh well..
pub fn find_diff_lines(original_lines: &Vec<String>, modified_lines: &Vec<String>) -> (Vec<usize>,Vec<String> ) {
    // 1. Calculate differences using a diff library
    // 2. Format differences with coordinates
    // 3. Color modified lines
    // 4. Return formatted diff and colored modified lines


    let mut different_lines = Vec::new();
    let mut linediffself:Vec<String> = Vec::new();


    for (i, (original_line, modified_line)) in original_lines.iter().zip(modified_lines).enumerate() {
        if original_line != modified_line {
            different_lines.push(i + 1); // Add 1 to index for human-readable line numbers
            linediffself.push(format!("+{}\t{}", i, modified_line).green().to_string());
>>>>>>> 5681678df4983228bcbee7c1230f127b14310c15
            //linediffself.push(modified_line.to_string()); //debug line with no modifications 
           // println!("DEBUG i EQUAL {}",i);
        }
    }
    if original_lines.len() != modified_lines.len() {
        let longer_lines = if original_lines.len() > modified_lines.len() {
            original_lines
        } else {
            modified_lines
        };
<<<<<<< HEAD
        let shorter_lines = if original_lines.len() < modified_lines.len() {
            original_lines
        } else {
            modified_lines
        };
=======
>>>>>>> 5681678df4983228bcbee7c1230f127b14310c15
        for i in different_lines.len()..longer_lines.len() {
            different_lines.push(i + 1);
            //println!("DEBUG i EQUAL {}",i);
            //println!("{:?}",longer_lines.get(i).unwrap());
            //linediffself.push(format!("+{}\t{:?}\n", i, longer_lines).green().to_string());
<<<<<<< HEAD
            println!("+{} {}",i,longer_lines.get(i).unwrap().green());
            println!("{}",shorter_lines.get(i).unwrap().green());
=======
            println!("+{} {}",i,longer_lines.get(i).unwrap());
>>>>>>> 5681678df4983228bcbee7c1230f127b14310c15
        }
    }

    (different_lines,linediffself)
<<<<<<< HEAD
} */


/*
pub fn find_diff_lines(original_lines: &Vec<String>, modified_lines: &Vec<String>) -> (Vec<usize>,Vec<String> ) {

    let mut difference:Vec<String> = Vec::new();
    let mut location:Vec<usize> = Vec::new();
    if original_lines.len() == modified_lines.len() {
        for i in 0..original_lines.len(){
            if original_lines.get(i) != modified_lines.get(i) {
                difference.push(modified_lines.get(i).unwrap().to_string());
                location.push(i);
            }
        }
    }





(location,difference)

}

 */
=======
}
>>>>>>> 5681678df4983228bcbee7c1230f127b14310c15
