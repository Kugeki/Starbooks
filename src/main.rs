use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("[E] Not enough arguments.");
    }
    let filename = &args[1];

    println!("File to read: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("[E] With file reading.");
    
    println!("Book text:\n{}", contents);
}
