use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use regex::Regex;

fn main() {
    let path1 = Path::new("./src/input/input4.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input4.txt");

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

    let re_xmas = Regex::new(r"XMAS").unwrap();

    let my_string = "XMASXMASXMXMASAMX";
    let matches: Vec<&str> = re_xmas.find_iter(my_string).map(|m| m.as_str()).collect();
    for mat in matches.iter() {
        println!("{mat}");
    }

    let count = re_xmas.find_iter(my_string).count();
    println!("Total matches: {count}");

    // east, south, northeast, southeast 
    // then reverse to get the other directions

    // Count regex matches for all directions, you can't 
    // have overlaps in one direction so this should count 
    // correctly

    let mut char_grid: Vec::<Vec::<char>> = Vec::new();

    for line in lines.into_iter() {
        let line = line.unwrap();

        let chars: Vec<char> = line.chars().collect();

        char_grid.push(chars);
    }

    let mut east: Vec<String> = Vec::new();
    let mut south: Vec<String> = Vec::new();
    let mut northeast: Vec<String> = Vec::new();
    let mut southeast: Vec<String> = Vec::new();
    let mut west: Vec<String> = Vec::new();
    let mut north: Vec<String> = Vec::new();
    let mut northwest: Vec<String> = Vec::new();
    let mut southwest: Vec<String> = Vec::new();

    let rows = char_grid.len();
    let cols = char_grid[0].len();

    // Populate east
    for row in 0..rows {
        let mut new_string = String::from("");
        for col in 0..cols {
            new_string.push(char_grid[row][col]);
        }

        east.push(new_string);
    }

    // Populate south
    for col in 0..cols {
        let mut new_string = String::from("");
        for row in 0..rows {
            new_string.push(char_grid[row][col]);
        }

        south.push(new_string);
    }

    // Populate northeast
    {
        // Travel up, including diagonal
        'step: for start_row in (0..rows).rev() {
            let mut new_string = String::from("");
            let mut col = 0;
            for row in (0..=start_row).rev() {
                // NOT NEEDED HERE, ADD IF WRITING LIB
                // if col >= cols {
                //     northeast.push(new_string);
                //     continue 'step;
                // }

                new_string.push(char_grid[row][col]);

                col += 1;
            }

            northeast.push(new_string);
        }

        // Travel east, excluding diagonal
        'step: for start_col in 1..cols {
            let mut new_string = String::from("");
            let mut row = rows - 1;
            for col in start_col..cols {
                // NOT NEEDED HERE, ADD IF WRITING LIB
                // if row < 0 {
                //     northeast.push(new_string);
                //     continue 'step;
                // }

                new_string.push(char_grid[row][col]);

                row -= 1;
            }

            northeast.push(new_string);
        }
    }

    // Populate southeast
    {
        // Travel down, including diagonal
        'step: for start_row in 0..rows {
            let mut new_string = String::from("");
            let mut col = 0;
            for row in start_row..rows {
                // NOT NEEDED HERE, ADD IF WRITING LIB
                // if col >= cols {
                //     southeast.push(new_string);
                //     continue 'step;
                // }

                new_string.push(char_grid[row][col]);

                col += 1;
            }

            southeast.push(new_string);
        }

        // Travel east, excluding diagonal
        'step: for start_col in 1..cols {
            let mut new_string = String::from("");
            let mut row = 0;
            for col in start_col..cols {
                // NOT NEEDED HERE, ADD IF WRITING LIB
                // if row >= rows {
                //     southeast.push(new_string);
                //     continue 'step;
                // }

                new_string.push(char_grid[row][col]);

                row += 1;
            }

            southeast.push(new_string);
        }
    }

    // Populate west
    for line in east.iter() {
        west.push(line.chars().rev().collect::<String>());
    }

    // Populate north 
    for line in south.iter() {
        north.push(line.chars().rev().collect::<String>());
    }

    // Populate northwest
    for line in southeast.iter() {
        northwest.push(line.chars().rev().collect::<String>());
    }

    // Populate southwest
    for line in northeast.iter() {
        southwest.push(line.chars().rev().collect::<String>());
    }

    // Count number of matches
    let mut total_count = 0;

    for line in east.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in south.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in northeast.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in southeast.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in west.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in north.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in northwest.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    for line in southwest.iter() {
        total_count += re_xmas.find_iter(line).count();
    }
    println!("total_count: {total_count}");

    total_count.to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut char_grid: Vec::<Vec::<char>> = Vec::new();

    for line in lines.into_iter() {
        let line = line.unwrap();

        let chars: Vec<char> = line.chars().collect();

        char_grid.push(chars);
    }

    let rows = char_grid.len();
    let cols = char_grid[0].len();

    let mut total_matches = 0;

    for row in 0..rows {
        if row + 2 >= rows {
            break;
        }

        for col in 0..cols {
            if col + 2 >= cols {
                break;
            }
            
            let mut query: Vec<Vec<char>> = Vec::new();

            for i in 0..3 {
                let mut row_of_char = Vec::new();

                for j in 0..3 {
                    row_of_char.push(char_grid[row + i][col + j]);
                }

                query.push(row_of_char);
            }

            // print!("\n");
            // for i in 0..3 {
            //     for j in 0..3 {
            //         print!("{}, ", query[i][j]);
            //     }
            //     print!("\n");
            // }

            if is_match(&query) {
                total_matches += 1;
            }
        }
    }

    total_matches.to_string()
}

fn is_match(query: &Vec<Vec<char>>) -> bool {
    assert_eq!(query.len(), 3);
    assert_eq!(query[0].len(), 3);

    let mut result = false;

    let poss_1: Vec<Vec<char>> = vec![vec!['M', '_', 'S'], 
                                      vec!['_', 'A', '_'], 
                                      vec!['M', '_', 'S']];

    let poss_2: Vec<Vec<char>> = vec![vec!['S', '_', 'S'], 
                                      vec!['_', 'A', '_'], 
                                      vec!['M', '_', 'M']];

    let poss_3: Vec<Vec<char>> = vec![vec!['M', '_', 'M'], 
                                      vec!['_', 'A', '_'], 
                                      vec!['S', '_', 'S']];

    let poss_4: Vec<Vec<char>> = vec![vec!['S', '_', 'M'], 
                                      vec!['_', 'A', '_'], 
                                      vec!['S', '_', 'M']];

    let possibilites = vec![poss_1, poss_2, poss_3, poss_4];
    for poss in possibilites.iter() {
        if pattern_match(poss, query) {
            result = true;
        }
    }

    result
}

fn pattern_match(pattern: &Vec<Vec<char>>, query: &Vec<Vec<char>>) -> bool {
    let mut result = true;
    for (row_idx, row) in pattern.iter().enumerate() {
        for (col_idx, ch) in row.iter().enumerate() {
            if *ch == '_' {
                continue;
            } 

            if *ch != query[row_idx][col_idx] {
                result = false;
            }
        }
    }

    result
}
