use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let pattern = &args[2];

    let f = File::open(file).expect("Could not open file");
    let reader = BufReader::new(f);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(pattern) {
            println!("{pattern} @ line {}", line_number + 1);
        }
    }
}

