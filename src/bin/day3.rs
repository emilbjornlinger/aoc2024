use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;

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

    let mut valid_expressions: Vec<String> = Vec::new();
    let mut multiplications: Vec<i32> = Vec::new();

    let re_expression = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_digits = Regex::new(r"\d+").unwrap();

    for line in lines.into_iter() {
        let line = line.unwrap();

        let mut expressions: Vec<String> = re_expression.find_iter(&line).map(|m| m.as_str().to_string()).collect();

        valid_expressions.append(&mut expressions);
    }

    for exp in valid_expressions.iter() {
        let digits: Vec<i32> = re_digits.find_iter(exp).map(|m| m.as_str().parse::<i32>().unwrap()).collect();

        assert!(digits.len() == 2);
        multiplications.push(digits[0]*digits[1]);
    }

    multiplications.iter().sum::<i32>().to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut valid_expressions: Vec<String> = Vec::new();
    let mut multiplications: Vec<i32> = Vec::new();

    let re_expression = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let re_digits = Regex::new(r"\d+").unwrap();

    for line in lines.into_iter() {
        let line = line.unwrap();

        let mut expressions: Vec<String> = re_expression.find_iter(&line).map(|m| m.as_str().to_string()).collect();

        valid_expressions.append(&mut expressions);
    }

    let mut enabled = true;
    for exp in valid_expressions.iter() {
        if exp == "do()" {
            enabled = true;
        } else if exp == "don't()" {
            enabled = false;
        } else {
            if enabled {
                let digits: Vec<i32> = re_digits.find_iter(exp).map(|m| m.as_str().parse::<i32>().unwrap()).collect();

                assert!(digits.len() == 2);
                multiplications.push(digits[0]*digits[1]);
            }
        }
    }

    multiplications.iter().sum::<i32>().to_string()
}
