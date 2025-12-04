#![allow(clippy::needless_return)]

use std::fs;

// const SAMPLE_PATH: &str = "./input/sample.txt";
const REAL_PATH: &str = "./input/real.txt";

// . = empty
// @ = occupied
fn count_neighbors(grid: &[String], row: usize, col: usize) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[0].len() as isize
            && grid[new_row as usize]
                .chars()
                .nth(new_col as usize)
                .expect("to get char")
                == '@'
        {
            count += 1;
        }
    }
    return count;
}

fn solve_part1() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let grid: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let rows = grid.len();
    let columns = grid[0].len();
    let mut acc = 0;

    for row in 0..rows {
        for col in 0..columns {
            let cell = grid[row].chars().nth(col).expect("to get char");
            if cell == '.' {
                continue;
            }

            let neighbors = count_neighbors(&grid, row, col);
            if neighbors < 4 {
                acc += 1;
            }
        }
    }

    println!("{acc}");
}

fn solve_part2() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut grid: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let rows = grid.len();
    let columns = grid[0].len();
    let mut acc = 0;

    loop {
        let mut should_break = true;
        for row in 0..rows {
            for col in 0..columns {
                let cell = grid[row].chars().nth(col).unwrap();
                if cell == '.' {
                    continue;
                }

                let neighbors = count_neighbors(&grid, row, col);
                if neighbors < 4 {
                    acc += 1;
                    grid[row].replace_range(col..=col, ".");
                    should_break = false;
                }
            }
        }

        if should_break {
            break;
        }
    }

    println!("{acc}");
}

fn main() {
    solve_part1();
    solve_part2();
}
