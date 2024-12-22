use std::env;
use std::fs;
use std::process;

fn main() {
    // 1. Handle command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <search_term> <file_path>", args[0]);
        process::exit(1);
    }

    let search_term = &args[1];
    let file_path = &args[2];

    // 2. Read the file
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    // 3. Search for the term and print matching lines
    for line in contents.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}