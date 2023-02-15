use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file
    let file = File::open("input.txt").expect("Failed to open input.txt");

    // Read the file line by line
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // Process each line
        count += line.chars().count();
    }

    // Output the result
    println!("The total number of characters is: {}", count);
}