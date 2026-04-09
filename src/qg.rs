use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::process;

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
    
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);
    
    /*
     *  _     ___   ____ ___ ____
     * | |   / _ \ / ___|_ _/ ___|
     * | |  | | | | |  _ | | |
     * | |__| |_| | |_| || | |___
     * |_____\___/ \____|___\____|
    */

    for line in reader.lines() { // checks each line for match, if found, returns 0, else returns 1
        let line = line.unwrap();
        if line.contains(pattern) {
            process::exit(0);
        }
    }
    process::exit(1);
}



