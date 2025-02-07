use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;

fn main() {
    let path1 = Path::new("./src/input/input5.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input5.txt");

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

    // IDEA: Hash all the ordering rules pairs, then check each pair 
    // in reverse order to what they appear in the input, if the pair 
    // exists in the hashmap, the print order is not valid. Specifically:
    // Check backwards, so if all pairs looking at pages coming before the 
    // current page is part of the hashmap, print order is not valid.
    
    // SECOND IDEA: Create a DAG from the ordering rules, then check each input
    // if it conforms to the topological sort. Is there an easy way to do this 
    // check? Not sure anything better than linear exists for each pair of nodes
    // so don't go with this idea. Would be interesting to find something that is 
    // actually more efficient though.

    for line in lines.into_iter() {
        let line = line.unwrap();
        println!("{line}")
    }

    "Hello".to_string()
}

fn part2(path: &Path) -> String {
    "Hello".to_string()
}
