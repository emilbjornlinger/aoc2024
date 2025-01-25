use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;

fn main() {
    let path1 = Path::new("./src/input/input4.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input4.txt");

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

    let re_xmas = Regex::new(r"XMAS").unwrap();

    let my_string = "XMASXMASXMXMASAMX";
    let matches: Vec<&str> = re_xmas.find_iter(my_string).map(|m| m.as_str()).collect();
    for mat in matches.iter() {
        println!("{mat}");
    }

    // east, south, northeast, southeast 
    // then reverse to get the other directions

    // Count regex matches for all directions, you can't 
    // have overlaps in one direction so this should count 
    // correctly

    let mut char_grid: Vec::<Vec::<char>> = Vec::new();

    for line in lines.into_iter() {
        let line = line.unwrap();

        let chars: Vec<char> = line.chars().collect();

        char_grid.push(chars);
    }

    println!("char_grid[1][1]: {}", char_grid[1][1]);

    "Hello".to_string()
}

fn part2(path: &Path) -> String {
    "Hello".to_string()
}
