#![allow(clippy::needless_return)]

use std::fs;

// const SAMPLE_PATH: &str = "./input/sample.txt";
const REAL_PATH: &str = "./input/real.txt";

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Rotation {
    pub direction: Direction,
    pub amount: u32,
}

impl TryFrom<&str> for Rotation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, amount) = value.split_at(1);
        assert!(direction.len() == 1);
        assert!(!amount.is_empty());

        let numeric_amount = amount.parse::<u32>().map_err(|e| e.to_string())?;

        match direction {
            "L" => Ok(Rotation {
                direction: Direction::Left,
                amount: numeric_amount,
            }),
            "R" => Ok(Rotation {
                direction: Direction::Right,
                amount: numeric_amount,
            }),
            _ => Err(format!("invalid direction: {}", direction)),
        }
    }
}

fn solve_part1() {
    const DIAL_SIZE: u32 = 100;
    const DIAL_INITIAL_POSITION: u32 = 50;

    let content = fs::read_to_string(REAL_PATH).expect("to read input file");
    let rotations = content
        .lines()
        .map(|line| Rotation::try_from(line).expect("to parse rotation"))
        .collect::<Vec<Rotation>>();

    let mut dial_at_zero_counter: u32 = 0;
    let mut dial_position: i32 = DIAL_INITIAL_POSITION as i32;

    for rotation in rotations {
        let sign = match rotation.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        dial_position = (dial_position + sign * rotation.amount as i32) % DIAL_SIZE as i32;

        if dial_position == 0 {
            dial_at_zero_counter += 1;
        }
    }

    println!("{dial_at_zero_counter}");
}

fn solve_part2() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");
    let rotations = content
        .lines()
        .map(|line| Rotation::try_from(line).expect("to parse rotation"))
        .collect::<Vec<Rotation>>();

    const DIAL_SIZE: u32 = 100;
    let mut dial_at_zero_counter: u32 = 0;
    let mut dial_position: i32 = 50;

    for mut rotation in rotations {
        let sign = match rotation.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        dial_at_zero_counter += rotation.amount / DIAL_SIZE;
        rotation.amount %= DIAL_SIZE;

        for _ in 0..rotation.amount {
            dial_position = (dial_position + sign) % DIAL_SIZE as i32;
            if dial_position == 0 {
                dial_at_zero_counter += 1;
            }
        }
    }

    println!("{dial_at_zero_counter}");
}

fn main() {
    solve_part1();
    solve_part2();
}
