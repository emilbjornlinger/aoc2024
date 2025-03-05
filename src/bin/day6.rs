use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use aoc_lib;

// Use case specific
enum Items {
    Obstacle,
    Visited,
    Unvisited,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    x: usize,
    y: usize,
}

// Abstract this out into lib
struct Grid<T> {
    grid: Vec<Vec<T>>,
    pos: Position,
}

impl<T> Grid<T> {
    fn new() -> Self {
        Grid {
            grid: Vec::<Vec<T>>::new(),
            pos: Position { x: 0, y: 0 },
        }
    }

    fn move_pos(&mut self, dir: &Direction, steps: Option<usize>) -> Result<(), &'static str> {
        // Assert grid has been initialized
        assert!(
            !(self.grid.len() == 0) || !(self.grid[0].len() == 0),
            "Grid is of size 0"
        );

        // Default to 1 step
        let steps = match steps {
            Some(i) => i,
            None => 1,
        };

        let result = match dir {
            Direction::Up => {
                if self.pos.y - steps > 0 {
                    self.pos.y = self.pos.y - steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Down => {
                if self.pos.y + steps < self.grid.len() {
                    self.pos.y = self.pos.y + steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Left => {
                if self.pos.x - steps > 0 {
                    self.pos.x = self.pos.x - steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Right => {
                if self.pos.x - steps < self.grid[0].len() {
                    self.pos.x = self.pos.x - steps;
                    Ok(())
                } else {
                    Err("Out of bounds")
                }
            }
        };

        result
    }

    fn peek<'a>(&'a mut self, dir: &Direction, steps: Option<usize>) -> Result<&'a T, &'static str> {
        // Assert grid has been initialized
        assert!(
            !(self.grid.len() == 0) || !(self.grid[0].len() == 0),
            "Grid is of size 0"
        );

        // Default to 1 step
        let steps = match steps {
            Some(i) => i,
            None => 1,
        };

        let result = match dir {
            Direction::Up => {
                if self.pos.y - steps > 0 {
                    Ok(&self.grid[self.pos.y - steps][self.pos.x])
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Down => {
                if self.pos.y + steps < self.grid.len() {
                    Ok(&self.grid[self.pos.y + steps][self.pos.x])
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Left => {
                if self.pos.x - steps > 0 {
                    Ok(&self.grid[self.pos.y][self.pos.x - steps])
                } else {
                    Err("Out of bounds")
                }
            }
            Direction::Right => {
                if self.pos.x - steps < self.grid[0].len() {
                    Ok(&self.grid[self.pos.y][self.pos.x + steps])
                } else {
                    Err("Out of bounds")
                }
            }
        };

        result
    }
}

fn main() {
    println!("{}", aoc_lib::info());
    let path1 = Path::new("./src/input/input6.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input6.txt");

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
                    Items::Unvisited
                }
                _ => panic!("Invalid character encountered when parsing grid"),
            })
            .collect();

        map.grid.push(new_grid_row);
    }

    // Simulate guard
    let mut curr_dir = Direction::Up;
    let mut on_map = true;

    while on_map {
        // Check if current pos is unvisited
        // set to visited 
        // increase counter
        
        // Check if next position is an obstacle
        // Turn direction
        match map.peek(&curr_dir, None) {
            // Continue here using turn_right function
        }
        
        // Check if next position is outside of map
        // Set on_map to false

        // Take a step
    }


    "hej".to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    "hej".to_string()
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
