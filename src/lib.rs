use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn grep(file: &str, pattern: &str) -> std::io::Result<String> {
    /*
     *  __     ___    ____  ___    _    ____  _     _____ ____
     *  \ \   / / \  |  _ \|_ _|  / \  | __ )| |   | ____/ ___|
     *   \ \ / / _ \ | |_) || |  / _ \ |  _ \| |   |  _| \___ \
     *    \ V / ___ \|  _ < | | / ___ \| |_) | |___| |___ ___) |
     *     \_/_/   \_\_| \_\___/_/   \_\____/|_____|_____|____/
    */
    
    let mut output = String::new();

    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);
    
    /*
     *  _     ___   ____ ___ ____
     * | |   / _ \ / ___|_ _/ ___|
     * | |  | | | | |  _ | | |
     * | |__| |_| | |_| || | |___
     * |_____\___/ \____|___\____|
    */

    for (linen, line) in reader.lines().enumerate() { // runs the matching algorithm on every line
        let line = line.unwrap();
        if line.contains(pattern) {
            // println!("{}", linen + 1); // inefficient and stupid, never use it
            output.push_str(&format!("{}\n", linen + 1)); // pushes the string and does some formatting
            }
        }
        Ok(output)
}


