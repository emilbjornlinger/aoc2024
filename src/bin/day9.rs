use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct FileBlock {
    id: i64,
    size: u8,
    free: bool,
}

fn main() {
    println!("{}", aoc_lib::info());
    let path1 = Path::new("./src/input/input9.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input9.txt");

    let output2 = part2(&path2);
    dbg!(output2);
}

fn part1(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    // Get the one long input line
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();

    let mut queue = VecDeque::<i64>::new();
    let mut curr_file_id: i64 = 0;

    for (idx, num) in line.chars().enumerate() {
        let num = num.to_digit(10).expect("Couldn't parse number");

        if idx % 2 == 1 {
            for _ in 0..num {
                queue.push_back(-1);
            }
        } else {
            for _ in 0..num {
                queue.push_back(curr_file_id);
            }

            curr_file_id += 1;
        }
    }

    // Compress file system
    let mut file_system = Vec::<i64>::new();
    while queue.len() > 0 {
        if let Some(popped_front) = queue.pop_front() {
            if popped_front == -1 {
                let mut more_to_process = true;
                while more_to_process {
                    if let Some(popped_back) = queue.pop_back() {
                        if popped_back != -1 {
                            // Found file fragment to move, stop processing
                            file_system.push(popped_back);
                            more_to_process = false;
                        }
                    } else {
                        // No more data, exit
                        more_to_process = false;
                    }
                }
            } else {
                file_system.push(popped_front);
            }
        }
    }

    let sum = file_system
        .iter()
        .enumerate()
        .map(|(idx, file_id)| idx as i64 * file_id)
        .sum::<i64>();

    sum.to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    // Get the one long input line
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();

    let mut queue = VecDeque::<FileBlock>::new();
    let mut curr_file_id: i64 = 0;

    for (idx, num) in line.chars().enumerate() {
        let num: u8 = num.to_digit(10).expect("Couldn't parse number") as u8;

        if idx % 2 == 1 {
            queue.push_back(FileBlock {
                id: -1,
                size: num,
                free: true,
            });
        } else {
            queue.push_back(FileBlock {
                id: curr_file_id,
                size: num,
                free: true,
            });

            curr_file_id += 1;
        }
    }

    // Compress file system, have one pointer moving back to front (don't 
    // need a queue, vec is fine) and for each FileBlock look at the slice before
    // it and see if you can swap with any free space. If so, do it but think about
    // how you will handle expanded free blocks where the swapped out block was, and 
    // how to handle free memory next to where the file was swapped to.
    todo!();

    // Then expand the FileBlocks into a normal vector and sum
    let sum = file_system
        .iter()
        .enumerate()
        .map(|(idx, file_id)| idx as i64 * file_id)
        .sum::<i64>();

    sum.to_string()
}
