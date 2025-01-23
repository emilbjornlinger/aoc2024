use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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

    let mut safe_records = 0;

    'outer: for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut vals_string: Vec<_> = line.split(' ').collect();
        vals_string.retain(|x| *x != "");
        let mut vals: Vec<i32> = Vec::new();

        for val in vals_string.iter() {
            vals.push(val.parse::<i32>().unwrap());
        }

        let mut slope: i32 = 0;

        for (idx, window) in vals.windows(2).enumerate() {
            if idx == 0 {
                slope = window[1] - window[0];
                if slope == 0 {
                    continue 'outer;
                } else if slope.abs() > 3 {
                    continue 'outer;
                }
            } else {
                let diff = window[1] - window[0];
                if diff == 0 {
                    continue 'outer;
                } else if diff.abs() > 3 {
                    continue 'outer;
                } else if (diff > 0 && slope < 0) || (diff < 0 && slope > 0) {
                    continue 'outer;
                }
            }
        }

        safe_records += 1;
    }

    safe_records.to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut safe_records = 0;

    'outer: for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        let mut vals_string: Vec<_> = line.split(' ').collect();
        vals_string.retain(|x| *x != "");
        let mut vals: Vec<i32> = Vec::new();

        for val in vals_string.iter() {
            vals.push(val.parse::<i32>().unwrap());
        }

        'perm: for i in 0..vals.len() as usize {
            let mut slope: i32 = 0;
            let mut val_copy = vals.clone();

            val_copy.remove(i);

            for (idx, window) in val_copy.windows(2).enumerate() {
                if idx == 0 {
                    slope = window[1] - window[0];
                    if slope == 0 {
                        continue 'perm;
                    } else if slope.abs() > 3 {
                        continue 'perm;
                    }
                } else {
                    let diff = window[1] - window[0];
                    if diff == 0 {
                        continue 'perm;
                    } else if diff.abs() > 3 {
                        continue 'perm;
                    } else if (diff > 0 && slope < 0) || (diff < 0 && slope > 0) {
                        continue 'perm;
                    }
                }
            }
            safe_records += 1;
            continue 'outer;
        }
    }

    safe_records.to_string()
}

fn check_valid(first: i32, second: i32, slope: i32) -> bool {
    let mut result = true;

    let diff = second - first;
    if diff == 0 {
        result = false;
    } else if diff.abs() > 3 {
        result = false;
    } else if (diff > 0 && slope < 0) || (diff < 0 && slope > 0) {
        result = false;
    }

    result
}

fn check_valid_slope(first: i32, second: i32) -> (i32, bool) {
    let mut result = true;

    let slope = second - first;
    if slope == 0 {
        result = false;
    } else if slope.abs() > 3 {
        result = false;
    }

    (slope, result)
}
