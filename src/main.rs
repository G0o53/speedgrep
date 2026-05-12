use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    /*
     *  __     ___    ____  ___    _    ____  _     _____ ____
     *  \ \   / / \  |  _ \|_ _|  / \  | __ )| |   | ____/ ___|
     *   \ \ / / _ \ | |_) || |  / _ \ |  _ \| |   |  _| \___ \
     *    \ V / ___ \|  _ < | | / ___ \| |_) | |___| |___ ___) |
     *     \_/_/   \_\_| \_\___/_/   \_\____/|_____|_____|____/
     */

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let pattern = &args[2];

    // set buffer to do str_push to it every time a new match is found
    let mut buffer = String::new();

    // set the count to redo the loop and find more lines after is equal to 64
    let mut count = 0;

    let mut line = String::new();
    let mut linen = 0;

    let f = File::open(file).unwrap();
    let mut reader = BufReader::new(f);
    
    /* stdout lock */

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    /*
     *  _     ___   ____ ___ ____
     * | |   / _ \ / ___|_ _/ ___|
     * | |  | | | | |  _ | | |
     * | |__| |_| | |_| || | |___
     * |_____\___/ \____|___\____|
     */

    
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.contains(pattern) {
            // println!("{}", linen + 1); // inefficient and stupid, never use it
            buffer.push_str(&(linen + 1).to_string()); // pushes the string and does some formatting
            buffer.push('\n'); // pushes a newline
            count += 1;

            // checking if the loop has run 64 times, if yes, it flushes the buffer and sets count
            // to zero

            if count == 64 { // checks if count is 64
                out.write_all(buffer.as_bytes()).unwrap(); // prints buffer with lock
                out.flush().unwrap();
                buffer.clear(); // clears buffer
                count = 0; // resets count
            }
        }
    line.clear(); // reuse the same allocation
    linen += 1;
    }

    /* flushes the last bit of buffer into stdout */
    
    if !buffer.is_empty() { // ! means not empty, goose
        out.write_all(buffer.as_bytes()).unwrap(); // flushes the buffer with lock
    }
}
