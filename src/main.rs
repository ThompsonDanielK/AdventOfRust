pub mod puzzles;
pub mod reader;
pub mod writer;

use std::io::{self, Write};

fn main() {
    let ascii_art = r#"
    ___       _                             _       ___       __    ___                     _     
   /   \   __| |   __ __    ___    _ _     | |_    / _ \     / _|  | _ \   _  _     ___    | |_   
   | - |  / _` |   \ V /   / -_)  | ' \    |  _|  | (_) |   |  _|  |   /  | +| |   (_-<    |  _|  
   |_|_|  \__,_|   _\_/_   \___|  |_||_|   _\__|   \___/   _|_|_   |_|_\   \_,_|   /__/_   _\__|  
 _|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""| 
 "`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'  
 "#;

    println!("{}", ascii_art);

    let puzzle_number = loop {
        print!("Please enter the puzzle number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(number) => break number,
            Err(_) => println!("Error: Please enter a valid positive integer."),
        }
    };

    let file_path = "input.txt";

    let text_contents = match reader::text_reader(file_path) {
        Ok(contents) => {
            println!("File read successfully UwU:");
            contents
        }
        Err(e) => {
            eprintln!("Oopsie daisy, Error Reading file >w>: {}", e);
            return;
        }
    };

    let result = match puzzle_number {
        1 => puzzles::puzzle_1::solve(text_contents),
        _ => {
            eprintln!("Error: Puzzle number {} is not valid.", puzzle_number);
            return;
        }
    };

    println!("The answer is: {}! Great Work :3", result);

    let vec_string_result = vec![result.to_string()];

    match writer::text_writer("output.txt", vec_string_result) {
        Ok(_) => println!("Results written successfully to output.txt."),
        Err(e) => eprintln!("Error writing results to file: {}", e),
    }
}
