use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

use aoc_lib::combination;

struct Instance {
    target: u64,
    vals: Vec<u64>
}

fn main() {
    println!("{}", aoc_lib::info());
    let path1 = Path::new("./src/input/input7.txt");

    let (output1, invalid_instances) = part1(&path1);
    dbg!(output1);

    let output2 = part2(invalid_instances);
    dbg!(output1 + output2);
}

fn part1(path: &Path) -> (u64, Vec<Instance>) {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut counter = 0;
    let mut invalid_instances = Vec::<Instance>::new();

    let now = Instant::now();

    for line in lines.into_iter() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(':').collect();

        let target: u64 = parts[0]
            .parse::<u64>()
            .expect("Couldn't parse target number");

        let vals: Vec<u64> = parts[1].split(' ').filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x.parse::<u64>().expect("Couldn't parse number"))
            }
        }).collect();

        if valid_calibration_plus_mul(target, &vals) {
            counter += target;
        } else {
            invalid_instances.push(Instance {
                target,
                vals,
            });
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    (counter, invalid_instances)
}

fn part2(instances: Vec<Instance>) -> u64 {
    let mut counter = 0;

    let now = Instant::now();

    for instance in instances.into_iter() {
        if valid_calibration_plus_mul_concat(instance.target, &instance.vals) {
            counter += instance.target;
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    counter
}

fn valid_calibration_plus_mul(target: u64, vals: &Vec<u64>) -> bool {
    let mut found_calibration = false;

    let base: usize = 2;
    let num_elem_last_level: usize = base.pow((vals.len() - 1) as u32);
    let mut closures = Vec::<Box::<dyn Fn(u64, u64) -> u64>>::new();

    closures.push(Box::new(|x: u64, y: u64| x + y));
    closures.push(Box::new(|x: u64, y: u64| x * y));

    let tree = combination::memoized_combination_tree(vals, base, closures);

    for entry in tree[tree.len()-num_elem_last_level..].into_iter() {
        if entry.value == target {
            found_calibration = true;
            break;
        }
    }

    found_calibration
}

fn valid_calibration_plus_mul_concat(target: u64, vals: &Vec<u64>) -> bool {
    let mut found_calibration = false;

    let base: usize = 3;
    let num_elem_last_level: usize = base.pow((vals.len() - 1) as u32);
    let mut closures = Vec::<Box::<dyn Fn(u64, u64) -> u64>>::new();

    closures.push(Box::new(|x: u64, y: u64| {
        (x.to_string() + &y.to_string()).parse::<u64>().unwrap()
    }));
    closures.push(Box::new(|x: u64, y: u64| x + y));
    closures.push(Box::new(|x: u64, y: u64| x * y));

    let tree = combination::memoized_combination_tree(vals, base, closures);

    for entry in tree[tree.len()-num_elem_last_level..].into_iter() {
        if entry.value == target {
            found_calibration = true;
            break;
        }
    }

    found_calibration
}
