use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use aoc_lib;

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    println!("{}", aoc_lib::info());
    let path1 = Path::new("./src/input/input8.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input8.txt");

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

    let mut symbols = String::from("");

    for ch in 'a'..='z' {
        symbols.push(ch);
    }

    for ch in 'A'..='Z' {
        symbols.push(ch);
    }

    for ch in '0'..='9' {
        symbols.push(ch);
    }

    let mut positions: Vec<Vec<Position>> = Vec::new();
    for _ in 0..symbols.len() {
        positions.push(Vec::<Position>::new());
    }

    let mut initialized = false;
    let mut grid_size_x: i32 = 0;
    let mut grid_size_y: i32 = 0;

    for (y, line) in lines.into_iter().enumerate() {
        let line = line.unwrap();
        if !initialized {
            grid_size_x = line.len() as i32;
            initialized = true;
        }
        grid_size_y += 1;

        for (x, ch) in line.chars().enumerate() {
            match symbols.find(ch) {
                Some(idx) => positions[idx].push(Position {
                    x: x as i32,
                    y: y as i32,
                }),
                None => (),
            }
        }
    }

    let mut set_of_antinodes = HashSet::<Position>::new();

    for symbol_positions in positions.into_iter() {
        for pair in unordered_pairs(&symbol_positions).iter() {
            let x_step = pair.0.x - pair.1.x;
            let y_step = pair.0.y - pair.1.y;

            let anti_0_x = pair.0.x + x_step;
            let anti_0_y = pair.0.y + y_step;
            let anti_1_x = pair.1.x - x_step;
            let anti_1_y = pair.1.y - y_step;

            if anti_0_x >= 0 && anti_0_x < grid_size_x && anti_0_y >= 0 && anti_0_y < grid_size_y {
                set_of_antinodes.insert(Position {
                    x: anti_0_x,
                    y: anti_0_y,
                });
            }

            if anti_1_x >= 0 && anti_1_x < grid_size_x && anti_1_y >= 0 && anti_1_y < grid_size_y {
                set_of_antinodes.insert(Position {
                    x: anti_1_x,
                    y: anti_1_y,
                });
            }
        }
    }

    // Idea: Don't need to actually use the grid. Parse the grid, save the
    // positions of each digit in a Vec of Vec (one vec for each digit, create
    // a translation function from char to row index), then check each pair within
    // the same digit positions. Store the resulting antinode positions (that are within the grid)
    // in a HashMap to handle collisions.

    set_of_antinodes.len().to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut symbols = String::from("");

    for ch in 'a'..='z' {
        symbols.push(ch);
    }

    for ch in 'A'..='Z' {
        symbols.push(ch);
    }

    for ch in '0'..='9' {
        symbols.push(ch);
    }

    let mut positions: Vec<Vec<Position>> = Vec::new();
    for _ in 0..symbols.len() {
        positions.push(Vec::<Position>::new());
    }

    let mut initialized = false;
    let mut grid_size_x: i32 = 0;
    let mut grid_size_y: i32 = 0;

    for (y, line) in lines.into_iter().enumerate() {
        let line = line.unwrap();
        if !initialized {
            grid_size_x = line.len() as i32;
            initialized = true;
        }
        grid_size_y += 1;

        for (x, ch) in line.chars().enumerate() {
            match symbols.find(ch) {
                Some(idx) => positions[idx].push(Position {
                    x: x as i32,
                    y: y as i32,
                }),
                None => (),
            }
        }
    }

    let mut set_of_antinodes = HashSet::<Position>::new();

    for symbol_positions in positions.into_iter() {
        for pair in unordered_pairs(&symbol_positions).iter() {
            // Calculate steps
            let x_step = pair.0.x - pair.1.x;
            let y_step = pair.0.y - pair.1.y;

            let mut anti_0_x = pair.0.x;
            let mut anti_0_y = pair.0.y;

            // Trace back from first point
            let mut on_grid = true;
            while on_grid {
                // Add position
                set_of_antinodes.insert(Position {
                    x: anti_0_x,
                    y: anti_0_y,
                });

                // Calculate new positions
                anti_0_x = anti_0_x + x_step;
                anti_0_y = anti_0_y + y_step;

                // Check if still on grid
                if anti_0_x < 0 || anti_0_x >= grid_size_x || anti_0_y < 0 || anti_0_y >= grid_size_y {
                    on_grid = false;
                }
            }
            
            let mut anti_1_x = pair.1.x;
            let mut anti_1_y = pair.1.y;

            // Trace forward from second point
            on_grid = true;
            while on_grid {
                // Add position
                set_of_antinodes.insert(Position {
                    x: anti_1_x,
                    y: anti_1_y,
                });

                anti_1_x = anti_1_x - x_step;
                anti_1_y = anti_1_y - y_step;

                if anti_1_x < 0 || anti_1_x >= grid_size_x || anti_1_y < 0 || anti_1_y >= grid_size_y {
                    on_grid = false;
                }
            }
        }
    }

    set_of_antinodes.len().to_string()
}

fn unordered_pairs<T>(values: &Vec<T>) -> Vec<(&T, &T)> {
    let mut created_pairs = Vec::new();

    for (idx, first_val) in values.iter().enumerate() {
        for second_val in values[idx + 1..].iter() {
            created_pairs.push((first_val, second_val));
        }
    }

    created_pairs
}
