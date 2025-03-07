use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use aoc_lib::{self, grid::{Grid, Direction}};

// Use case specific
#[derive(PartialEq, Clone)]
enum Items {
    Obstacle,
    Visited(bool, bool, bool, bool),
    Unvisited,
}

fn main() {
    println!("{}", aoc_lib::info());
    let path1 = Path::new("./src/input/input6.txt");

    let (output1, map) = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input6.txt");

    let output2 = part2(&path2, map);
    dbg!(output2);
}

fn part1(path: &Path) -> (String, Grid<Items>) {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = Grid::<Items>::new();

    // Populate map
    for line in lines {
        let line = line.unwrap();

        let new_grid_row: Vec<Items> = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '.' => Items::Unvisited,
                '#' => Items::Obstacle,
                '^' => {
                    map.pos.x = i;
                    map.pos.y = map.grid.len();
                    Items::Visited(true, false, false, false)
                }
                _ => panic!("Invalid character encountered when parsing grid"),
            })
            .collect();

        map.grid.push(new_grid_row);
    }

    // Simulate guard, counter at one for start position
    let mut curr_dir = Direction::Up;
    let mut on_map = true;
    let mut counter = 1;

    while on_map {
        match map.grid[map.pos.y][map.pos.x] {
            Items::Unvisited => {
                let up = curr_dir == Direction::Up;
                let right = curr_dir == Direction::Right;
                let down = curr_dir == Direction::Down;
                let left = curr_dir == Direction::Left;
                map.grid[map.pos.y][map.pos.x] = Items::Visited(up, right, down, left);
                counter += 1;
            }
            Items::Visited(up, right, down, left) => match curr_dir {
                Direction::Up => {
                    map.grid[map.pos.y][map.pos.x] = Items::Visited(true, right, down, left)
                }
                Direction::Right => {
                    map.grid[map.pos.y][map.pos.x] = Items::Visited(up, true, down, left)
                }
                Direction::Down => {
                    map.grid[map.pos.y][map.pos.x] = Items::Visited(up, right, true, left)
                }
                Direction::Left => {
                    map.grid[map.pos.y][map.pos.x] = Items::Visited(up, right, down, true)
                }
            },
            Items::Obstacle => panic!("Ended up on obstacle"),
        }

        match map.peek(&curr_dir, None) {
            // Check if obstacle, turn direction
            Ok(Items::Obstacle) => match curr_dir {
                Direction::Up => curr_dir = Direction::Right,
                Direction::Right => curr_dir = Direction::Down,
                Direction::Down => curr_dir = Direction::Left,
                Direction::Left => curr_dir = Direction::Up,
            },
            // Irrelevant
            Ok(_) => (),
            Err(_) => (),
        };

        // Take a step
        match map.move_pos(&curr_dir, None) {
            // Check if new position is outside of map
            // Set on_map to false
            Err(_) => on_map = false,
            Ok(_) => (),
        }
    }

    // Print the grid
    println!("pos: ({}, {})", map.pos.x, map.pos.y);
    println!("curr_dir: {:?}", curr_dir);
    map.print_grid(|x| match x {
        Items::Visited(_, _, _, _) => 'X',
        Items::Unvisited => '.',
        Items::Obstacle => '#',
    });

    (counter.to_string(), map)
}

fn part2(path: &Path, map: Grid<Items>) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    // Extract positions to try
    let mut try_pos: Vec<(usize, usize)> = map
        .grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, item)| match item {
                    Items::Visited(_, _, _, _) => Some((i, j)),
                    Items::Unvisited => None,
                    Items::Obstacle => None,
                })
        })
        .collect();

    // Populate map
    let mut map = Grid::<Items>::new();
    for line in lines {
        let line = line.unwrap();

        let new_grid_row: Vec<Items> = line
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                '.' => Items::Unvisited,
                '#' => Items::Obstacle,
                '^' => {
                    map.pos.x = i;
                    map.pos.y = map.grid.len();
                    Items::Visited(false, false, false, false)
                }
                _ => panic!("Invalid character encountered when parsing grid"),
            })
            .collect();

        map.grid.push(new_grid_row);
    }

    // Remove start position from positions to try
    try_pos.swap_remove(
        try_pos
            .iter()
            .position(|(y, x)| *y == map.pos.y && *x == map.pos.x)
            .expect("Start position was not part of the positions to try"),
    );

    // Create hashset to store positions that yield a loop
    let mut set = HashSet::new();
    println!("Length of try_pos: {}", try_pos.len());

    // Try each position
    for (y, x) in try_pos.into_iter() {
        // Setup new map
        let mut curr_map = map.clone();
        curr_map.grid[y][x] = Items::Obstacle;

        // Simulate guard
        let mut curr_dir = Direction::Up;
        let mut on_map = true;

        while on_map {
            match curr_map.grid[curr_map.pos.y][curr_map.pos.x] {
                Items::Unvisited => {
                    let up = curr_dir == Direction::Up;
                    let right = curr_dir == Direction::Right;
                    let down = curr_dir == Direction::Down;
                    let left = curr_dir == Direction::Left;
                    curr_map.grid[curr_map.pos.y][curr_map.pos.x] =
                        Items::Visited(up, right, down, left);
                }
                Items::Visited(up, right, down, left) => match curr_dir {
                    Direction::Up => {
                        if up {
                            set.insert((x, y));
                            break;
                        }
                        curr_map.grid[curr_map.pos.y][curr_map.pos.x] =
                            Items::Visited(true, right, down, left);
                    }
                    Direction::Right => {
                        if right {
                            set.insert((x, y));
                            break;
                        }
                        curr_map.grid[curr_map.pos.y][curr_map.pos.x] =
                            Items::Visited(up, true, down, left);
                    }
                    Direction::Down => {
                        if down {
                            set.insert((x, y));
                            break;
                        }
                        curr_map.grid[curr_map.pos.y][curr_map.pos.x] =
                            Items::Visited(up, right, true, left);
                    }
                    Direction::Left => {
                        if left {
                            set.insert((x, y));
                            break;
                        }
                        curr_map.grid[curr_map.pos.y][curr_map.pos.x] =
                            Items::Visited(up, right, down, true);
                    }
                },
                Items::Obstacle => panic!("Ended up on obstacle"),
            }

            // Turn until no obstacles in the way, this happens because we try 
            // to insert obstacles into paths where we previously were right after
            // turning
            let mut peek_result = curr_map.peek(&curr_dir, None);
            while peek_result == Ok(&Items::Obstacle) {
                // Turn
                match peek_result {
                    // Check if obstacle, turn direction
                    Ok(Items::Obstacle) => match curr_dir {
                        Direction::Up => curr_dir = Direction::Right,
                        Direction::Right => curr_dir = Direction::Down,
                        Direction::Down => curr_dir = Direction::Left,
                        Direction::Left => curr_dir = Direction::Up,
                    },
                    // Irrelevant
                    Ok(_) => (),
                    Err(_) => (),
                };

                // Try new direction
                peek_result = curr_map.peek(&curr_dir, None);
            }

            // Take a step
            match curr_map.move_pos(&curr_dir, None) {
                // Check if new position is outside of map
                // Set on_map to false
                Err(_) => on_map = false,
                Ok(_) => (),
            }
        }
    }

    set.len().to_string()
}
