use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;
use regex::Regex;

use aoc_lib;

fn main() {
    println!("{}", aoc_lib::info());
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

    let mut is_rules = true;
    let mut rules: HashSet<(i32, i32)> = HashSet::new();
    let mut counter = 0;

    'outer: for line in lines.into_iter() {
        let line = line.unwrap();

        // Switch type of input
        if line == "" {
            is_rules = false;
            continue;
        }

        if is_rules {
            let vals: Vec<_> = line.split('|').collect();
            assert_eq!(vals.len(), 2);

            let mut before: i32 = 0;
            let mut after: i32 = 0;

            for (idx, val) in vals.iter().enumerate() {
                match val.parse::<i32>() {
                    Ok(num) => {
                        if idx == 0 {
                            before = num;
                        } else if idx == 1 {
                            after = num;
                        } else {
                            panic!("more numbers than expected");
                        }
                    }
                    _ => panic!("could not convert to a number as expected"),
                }
            }

            // Push to hashmap of rules
            rules.insert((before, after));
        } else {
            let vals_string: Vec<_> = line.split(',').collect();
            let mut vals: Vec<i32> = Vec::new();

            for val in vals_string.iter() {
                match val.parse::<i32>() {
                    Ok(num) => {
                        vals.push(num);
                        for prev_num in vals.iter() {
                            match rules.get(&(num, *prev_num)) {
                                Some(_) => continue 'outer,
                                _ => ()
                            }
                        }
                    },
                    _ => panic!("could not convert to a number as expected"),
                }
            }

            // Add the middle element to a counter
            counter += vals[vals.len()/2];
        }
    }


    counter.to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut is_rules = true;
    let mut rules: HashSet<(i32, i32)> = HashSet::new();
    let mut counter = 0;

    'outer: for line in lines.into_iter() {
        let line = line.unwrap();

        // Switch type of input
        if line == "" {
            is_rules = false;
            continue;
        }

        if is_rules {
            let vals: Vec<_> = line.split('|').collect();
            assert_eq!(vals.len(), 2);

            let mut before: i32 = 0;
            let mut after: i32 = 0;

            for (idx, val) in vals.iter().enumerate() {
                match val.parse::<i32>() {
                    Ok(num) => {
                        if idx == 0 {
                            before = num;
                        } else if idx == 1 {
                            after = num;
                        } else {
                            panic!("more numbers than expected");
                        }
                    }
                    _ => panic!("could not convert to a number as expected"),
                }
            }

            // Push to hashmap of rules
            rules.insert((before, after));
        } else {
            let vals_string: Vec<_> = line.split(',').collect();
            let mut vals: Vec<i32> = Vec::new();
            let mut incorrect = false;

            for val in vals_string.iter() {
                match val.parse::<i32>() {
                    Ok(num) => {
                        vals.push(num);
                        for prev_num in vals.iter() {
                            match rules.get(&(num, *prev_num)) {
                                Some(_) => incorrect = true,
                                _ => ()
                            }
                        }
                    },
                    _ => panic!("could not convert to a number as expected"),
                }
            }

            // Fix order and add to counter if incorrect
            if incorrect {
                for curr_pos in 0..vals.len() {
                    for prev_pos in 0..curr_pos {
                        match rules.get(&(vals[prev_pos], vals[curr_pos])) {
                            Some(_) => {
                                let tmp = vals[prev_pos];
                                vals[prev_pos] = vals[curr_pos];
                                vals[curr_pos] = tmp;
                            },
                            _ => ()
                        }
                    }
                }

                // Add the middle element to a counter
                counter += vals[vals.len()/2];
            }
        }
    }


    counter.to_string()
}
