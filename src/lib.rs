use std::fs::File;
use std::io::{BufRead, BufReader};

#[inline]
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
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let mut linen = 0; 

    /*
     *  _     ___   ____ ___ ____
     * | |   / _ \ / ___|_ _/ ___|
     * | |  | | | | |  _ | | |
     * | |__| |_| | |_| || | |___
     * |_____\___/ \____|___\____|
     */

        while reader.read_line(&mut line).unwrap() > 0 { // runs the matching algorithm on every line
        if line.contains(pattern) {
            // println!("{}", linen + 1); // inefficient and stupid, never use it
            output.push_str(&(linen + 1).to_string()); // pushes the string and does some formatting
            output.push(' '); // pushes a space
        }
        line.clear();
        linen += 1;
    }
    Ok(output)
}

#[inline]
pub fn qgrep(file: &str, pattern: &str) -> i8 {
    /*
     *  __     ___    ____  ___    _    ____  _     _____ ____
     *  \ \   / / \  |  _ \|_ _|  / \  | __ )| |   | ____/ ___|
     *   \ \ / / _ \ | |_) || |  / _ \ |  _ \| |   |  _| \___ \
     *    \ V / ___ \|  _ < | | / ___ \| |_) | |___| |___ ___) |
     *     \_/_/   \_\_| \_\___/_/   \_\____/|_____|_____|____/
     */

    let f = File::open(file).unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    
    /*
     *  _     ___   ____ ___ ____
     * | |   / _ \ / ___|_ _/ ___|
     * | |  | | | | |  _ | | |
     * | |__| |_| | |_| || | |___
     * |_____\___/ \____|___\____|
     */

    while reader.read_line(&mut line).unwrap() > 0 { // checks each line for match, if found, returns 0, else returns 1
        if line.contains(pattern) {
            return 0;
        }
    }
    -1
}
