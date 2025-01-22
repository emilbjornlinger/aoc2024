use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path1 = Path::new("./src/input/input2.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input2.txt");

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

    let safe_records = 0;

    'outer: for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut vals_string: Vec<_> = line.split(' ').collect();
        vals_string.retain(|x| *x != "");
        let mut vals: Vec<i32> = Vec::new(); 

        for val in vals_string.iter() {
            vals.push(val.parse::<i32>().unwrap()); 
        }

        let diff: i32 = 0;

        for (idx, window) in vals.windows(2).enumerate() {
            if idx == 0 {
                diff = window[1] - window[0];
                if diff == 0 {
                    continue 'outer;
                }
            }
        }
    }

    let mut distances: Vec<i32> = Vec::new();

    for (first, second) in first_nums.iter().zip(second_nums.iter()) {
        distances.push((first - second).abs());
    }

    let total_distance: i32 = distances.iter().sum();
    total_distance.to_string()
}

fn part2(path: &Path) -> String {
    "Hello".to_string()
}
