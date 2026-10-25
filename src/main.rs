use std::fs::File;
use std::io;

use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut file_path = String::new();
    println!("Enter the file path to read with file name and extension e.g. assets/test.txt : ");
    while let Ok(_) = io::stdin().read_line(&mut file_path) {
        if file_path.is_empty()  {
            println!("File path is empty");
            continue;
        }
        if Path::new(&file_path.trim()).is_file() {
            break;
        }
        println!("{}: File does not exist or is not a file, please enter a valid file path", file_path);
        file_path.clear();
        continue;
    }
    file_path = file_path.trim().to_string().clone();
    let file = File::open(&file_path);
    match file {
        Ok(file)=>{
            println!("File opened successfully");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        }
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("{}: File not found, please enter a valid file path", &file_path);
                }
                _ => {
                        println!("{}: Error opening file: {}", file_path, error);
                }
            }
        },
    }
}