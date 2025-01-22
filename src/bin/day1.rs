use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path1 = Path::new("./src/input/input1.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input1.txt");

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

    let mut first_nums: Vec<i32> = Vec::new();
    let mut second_nums: Vec<i32> = Vec::new();

    for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut vals: Vec<_> = line.split(' ').collect();
        vals.retain(|x| *x != "");
        assert_eq!(vals.len(), 2);

        for (idx, val) in vals.iter().enumerate() {
            match val.parse::<i32>() {
                Ok(num) => {
                    if idx == 0 {
                        first_nums.push(num);
                    } else if idx == 1 {
                        second_nums.push(num);
                    } else {
                        panic!("more numbers than expected");
                    }
                }
                _ => panic!("could not convert to a number as expected"),
            }
        }
    }

    first_nums.sort();
    second_nums.sort();

    let mut distances: Vec<i32> = Vec::new();

    for (first, second) in first_nums.iter().zip(second_nums.iter()) {
        distances.push((first - second).abs());
    }

    let total_distance: i32 = distances.iter().sum();
    total_distance.to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut first_nums: Vec<i32> = Vec::new();
    let mut second_nums: Vec<i32> = Vec::new();

    for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut vals: Vec<_> = line.split(' ').collect();
        vals.retain(|x| *x != "");
        assert_eq!(vals.len(), 2);

        for (idx, val) in vals.iter().enumerate() {
            match val.parse::<i32>() {
                Ok(num) => {
                    if idx == 0 {
                        first_nums.push(num);
                    } else if idx == 1 {
                        second_nums.push(num);
                    } else {
                        panic!("more numbers than expected");
                    }
                }
                _ => panic!("could not convert to a number as expected"),
            }
        }
    }

    first_nums.sort();
    second_nums.sort();

    let mut scores: Vec<i32> = Vec::new();

    for first in first_nums.iter() {
        let mut ctr = 0;

        for second in second_nums.iter() {
            if first == second {
                ctr += 1;
            }

            if second > first {
                break;
            }
        }

        scores.push(ctr*first);
    }

    let total_score: i32 = scores.iter().sum();
    total_score.to_string()
}
