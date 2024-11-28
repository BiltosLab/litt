use colored::*;
use std::cmp::{max, min};
#[derive(Debug, Clone, PartialEq)]
enum DiffOp {
    Insert(usize, String),
    Delete(usize, String),
    Equal,
}

pub fn find_diff_lines(
    original_file: Vec<String>,
    modified_file: Vec<String>,
) -> Vec<String> {
    let diffs = myers_diff(&original_file, &modified_file);
    let mut result = Vec::new();

    // Add file headers
    result.push(format!("--- original"));
    result.push(format!("+++ modified"));

    let mut i = 0; // Index in original_file
    let mut j = 0; // Index in modified_file

    while i < original_file.len() || j < modified_file.len() {
        // Skip over equal lines
        while i < original_file.len()
            && j < modified_file.len()
            && original_file[i] == modified_file[j]
        {
            i += 1;
            j += 1;
        }

        if i >= original_file.len() && j >= modified_file.len() {
            break;
        }

        // Start of a diff hunk
        let mut hunk_start_original = i + 1;
        let mut hunk_start_modified = j + 1;

        // Collect changes
        let mut hunk_original_lines = Vec::new();
        let mut hunk_modified_lines = Vec::new();

        while (i < original_file.len() && j < modified_file.len() && original_file[i] != modified_file[j])
            || (i < original_file.len() && j >= modified_file.len())
            || (i >= original_file.len() && j < modified_file.len())
        {
            if i < original_file.len() && (j >= modified_file.len() || original_file[i] != modified_file[j]) {
                hunk_original_lines.push((i + 1, original_file[i].clone()));
                i += 1;
            }

            if j < modified_file.len() && (i >= original_file.len() || original_file[i - 1] != modified_file[j]) {
                hunk_modified_lines.push((j + 1, modified_file[j].clone()));
                j += 1;
            }
        }

        let original_range = if hunk_original_lines.is_empty() {
            format!("{},0", hunk_start_original - 1)
        } else if hunk_original_lines.len() == 1 {
            format!("{}", hunk_start_original)
        } else {
            format!("{},{}", hunk_start_original, hunk_original_lines.len())
        };

        let modified_range = if hunk_modified_lines.is_empty() {
            format!("{},0", hunk_start_modified - 1)
        } else if hunk_modified_lines.len() == 1 {
            format!("{}", hunk_start_modified)
        } else {
            format!("{},{}", hunk_start_modified, hunk_modified_lines.len())
        };

        // Hunk header
        result.push(format!(
            "@@ -{} +{} @@",
            original_range, modified_range
        ));

        // Hunk content
        for (line_num, line) in &hunk_original_lines {
            let formatted_line = format!("-{}", line).red().to_string();
            result.push(formatted_line);
        }
        for (line_num, line) in &hunk_modified_lines {
            let formatted_line = format!("+{}", line).green().to_string();
            result.push(formatted_line);
        }
    }

    result
}

fn myers_diff(a: &[String], b: &[String]) -> Vec<DiffOp> {
    let n = a.len();
    let m = b.len();
    let max = n + m;
    let mut v = vec![0usize; 2 * max + 1];
    let mut trace = Vec::new();

    for d in 0..=max {
        let mut v_prev = v.clone();
        for k in (-(d as isize)..=d as isize).step_by(2) {
            let index = (k + max as isize) as usize;

            let x_start;
            if k == -(d as isize)
                || (k != d as isize
                    && v_prev[(k - 1 + max as isize) as usize]
                        < v_prev[(k + 1 + max as isize) as usize])
            {
                x_start = v_prev[(k + 1 + max as isize) as usize];
            } else {
                x_start = v_prev[(k - 1 + max as isize) as usize] + 1;
            }

            let mut x = x_start;
            let mut y = (x as isize - k) as usize;

            while x < n && y < m && a[x] == b[y] {
                x += 1;
                y += 1;
            }

            v[index] = x;

            if x >= n && y >= m {
                trace.push(v.clone());
                return backtrack(a, b, &trace);
            }
        }
        trace.push(v.clone());
    }

    Vec::new()
}

fn backtrack(a: &[String], b: &[String], trace: &[Vec<usize>]) -> Vec<DiffOp> {
    let mut diffs = Vec::new();
    let n = a.len();
    let m = b.len();
    let max = n + m;
    let mut x = n;
    let mut y = m;

    for (d, v) in trace.iter().enumerate().rev() {
        let k = x as isize - y as isize;
        let index = (k + max as isize) as usize;

        let (prev_k, prev_x, prev_y) = if k == -(d as isize)
            || (k != d as isize
                && v[(k - 1 + max as isize) as usize] < v[(k + 1 + max as isize) as usize])
        {
            let prev_k = k + 1;
            let prev_x = v[(prev_k + max as isize) as usize];
            let prev_y = (prev_x as isize - prev_k) as usize;
            (prev_k, prev_x, prev_y)
        } else {
            let prev_k = k - 1;
            let prev_x = v[(prev_k + max as isize) as usize];
            let prev_y = (prev_x as isize - prev_k) as usize;
            (prev_k, prev_x, prev_y)
        };

        while x > prev_x && y > prev_y {
            x -= 1;
            y -= 1;
            diffs.push(DiffOp::Equal);
        }

        if x == prev_x {
            y -= 1;
            diffs.push(DiffOp::Insert(y, b[y].clone()));
        } else {
            x -= 1;
            diffs.push(DiffOp::Delete(x, a[x].clone()));
        }
    }

    diffs.reverse();
    diffs
}


