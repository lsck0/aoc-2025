#![allow(clippy::needless_return)]

use num::{Zero, bigint::BigUint};
use std::fs;

// const SAMPLE_PATH: &str = "./input/sample.txt";
const REAL_PATH: &str = "./input/real.txt";

fn solve_part1() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut acc = 0;

    content.trim().lines().for_each(|line| {
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).expect("to have a digit char"))
            .collect::<Vec<u32>>();

        let (first_digit_index, first_digit) = digits[0..digits.len() - 1]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1))
            .expect("to have a max digit");

        let second_digit = digits[first_digit_index + 1..digits.len()]
            .iter()
            .max()
            .expect("to have a max digit");

        let jolts = first_digit * 10 + second_digit;

        acc += jolts;
    });

    println!("{acc}");
}

fn solve_part2() {
    fn number_from_digits(digits: &[u32]) -> BigUint {
        return digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| BigUint::from(*d) * BigUint::from(10u32).pow(i as u32))
            .sum();
    }

    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut acc: BigUint = BigUint::zero();

    content.trim().lines().for_each(|line| {
        let mut digits = line
            .chars()
            .map(|c| c.to_digit(10).expect("to have a digit char"))
            .collect::<Vec<u32>>();

        // greedy bruteforce - really bad, but works
        while digits.len() > 12 {
            let mut options = vec![];

            for i in 0..digits.len() {
                let mut option = digits.clone();
                option.remove(i);
                options.push(option);
            }

            let best_option = options
                .iter()
                .max_by(|a, b| {
                    let a_value = number_from_digits(a);
                    let b_value = number_from_digits(b);
                    a_value.cmp(&b_value)
                })
                .expect("to have a best option");

            digits = best_option.clone();
        }

        let jolts = number_from_digits(&digits);

        acc += jolts;
    });

    println!("{acc}");
}

fn main() {
    solve_part1();
    solve_part2();
}
