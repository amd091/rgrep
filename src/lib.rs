use colored::*;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn search_directories(search_term: &str, directory: String, file_type: &str) {
    let path = Path::new(&directory);

    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap();

        for entry in entries {
            //let entry = entry.unwrap();
            let entry_path = entry.unwrap().path();
            //println!("{:?}", entry_path);
            if entry_path.is_dir() {
                let directory = match entry_path.to_str() {
                    Some(s) => s,
                    None => {
                        println!("Invalid UTF-8 sequence in directory name: {} ",directory);
                        return;
                    }
                };
                search_directories(search_term,directory.to_string(),file_type)

            // } else if entry_path.is_file() && entry_path.to_str().unwrap().ends_with(&file_type) {
            //     let file_name = entry_path.to_str().unwrap();
            //     string_search(file_name, search_term)
            // }
            } else if entry_path.is_file() && entry_path.to_str().unwrap().ends_with(&file_type) {
                
                match entry_path.to_str() {
                    Some(file_name) => string_search(file_name, search_term),
                    None => println!("Failed to convert file path to string: {:?}", entry_path),
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
            let parts: Vec<&str> = line.split(search_term).collect();
            let line_num_colored = format!("Line {}", line_num);
            let colored_line = format!(
                "{}{}{}",
                parts[0].to_owned(),
                search_term.green().bold(),
                parts[1].to_owned()
            );
            println!(
                " {} :     {} {}",
                line_num_colored.blue(),
                file_name.cyan(),
                colored_line
            );
        }
    }
}
