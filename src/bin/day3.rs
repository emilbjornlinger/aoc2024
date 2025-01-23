use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path1 = Path::new("./src/input/input3.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input3.txt");

    let output2 = part2(&path2);
    dbg!(output2);
}

fn part1(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    // IDEA: Make a while loop over each line with a custom index
    // Create a utility function to check if querying an index is out of bounds
    // Create a state machine that does the parsing and transitions into the correct
    // states and do the correct action, if it fails, it remains on the same index and
    // starts over. Loop over each line and let state machine handle each line. Put 
    // the state machine in start state and feed it a line. Then call begin. It will go
    // until the end of the line and then arive in finished state and output the total value
    // for that line. Begin has a loop that calls execute on the given state and checks if the
    // state is not finished. SOMETHING LIKE THIS.

    for (_, line) in lines.enumerate() {
        let line = line.unwrap();
    }

    "Hello".to_string()
}

fn part2(path: &Path) -> String {
    "Hello".to_string()
}
