use colored::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn search_directories(search_term: &str, directory: String, file_type: &str) {
    let path = Path::new(&directory);

    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap();
        
        for entry in entries {
            let entry_path = entry.unwrap().path();
            if entry_path.is_dir() {
                let directory = match entry_path.to_str() {
                    Some(s) => s,
                    None => {
                        println!("Invalid UTF-8 sequence in directory name: {} ", directory);
                        continue;
                    }
                };
                
                if is_not_hidden(directory) {
                    search_directories(search_term, directory.to_string(), file_type);
                }
            } else if entry_path.is_file() && file_ends_with(&entry_path,&file_type) {
                if let Some(file_name) = entry_path.to_str() {
                    string_search(file_name, search_term);
                } else {
                    println!("Failed to convert file path to string: {:?}", entry_path);
                }
            }
        }
    } else {
        println!("Not a directory: {:?}", path.to_str().unwrap());
    }
}

pub fn string_search(file_name: &str, search_term: &str) {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}\nError: {}", file_name, e);
            return;
        }
    };
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(..) => {
                return;
            }
        };
        if line.contains(search_term) {
            let line_num = &index + 1;
            print_colored_lines(line, search_term, line_num, file_name);
        }
    }
}

pub fn is_not_hidden(directory: &str) -> bool {
    let parts: Vec<&str> = directory.split('/').filter(|s| !s.is_empty()).collect();
    !parts.iter().any(|s| s.starts_with("."))
}

pub fn print_colored_lines(line: String, search_term: &str, line_num: usize,file_name: &str)  {
    let parts: Vec<&str> = line.split(search_term).collect();
    let line_num_colored = format!("Line {}", line_num);
    let colored_line = format!("{}{}{}", parts[0],search_term.green().bold(), parts[1]);

    println!(
        " {} :     {} {}",
        line_num_colored.blue(),
        file_name.cyan(),
        colored_line
    );

}

pub fn file_ends_with(entry_path: &PathBuf,file_type: &str) -> bool {
    match entry_path.to_str() {
        Some(value) => {
            if value.ends_with(&file_type) {
                return true;
            } else {
                return false;
            }
        },
        None => return false,
    } 
}