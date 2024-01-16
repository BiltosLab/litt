use colored::*;

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
        for i in different_lines.len()..longer_lines.len() {
            different_lines.push(i + 1);
            //println!("DEBUG i EQUAL {}",i);
            //println!("{:?}",longer_lines.get(i).unwrap());
            //linediffself.push(format!("+{}\t{:?}\n", i, longer_lines).green().to_string());
            println!("+{} {}",i,longer_lines.get(i).unwrap().green());
        }
    }

    (different_lines,linediffself)
}