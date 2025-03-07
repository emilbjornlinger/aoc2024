use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use aoc_lib;

fn main() {
    println!("{}", aoc_lib::info());
    let path1 = Path::new("./src/input/input6.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input6.txt");

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

    // Idea: Don't need to actually use the grid. Parse the grid, save the 
    // positions of each digit in a Vec of Vec (one vec for each digit, create 
    // a translation function from char to row index), then check each pair within
    // the same digit positions. Store the resulting antinode positions (that are withing the grid) 
    // in a HashMap to handle collisions.

    "hej".to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    "hej".to_string()
}
