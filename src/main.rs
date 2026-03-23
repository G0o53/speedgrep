use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let pattern = &args[2];
    
    // set buffer to add line number to it each time a match is found
    let mut buffer = String::new();

    // set the ammount until buffer write to 1
    let mut count = 0;

    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);

    for (linen, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(pattern) {
            // println!("{}", linen + 1); // inefficient and stupid
            buffer.push_str(&format!("{}\n", linen + 1));
            count += 1;

            if count == 64 
            {
                println!("{buffer}\n");
                buffer.clear();
                count = 0;

            }
            }
        }
        // reset it to one to do again later
        if !buffer.is_empty() {
            println!("{buffer}"); 
    }
}

