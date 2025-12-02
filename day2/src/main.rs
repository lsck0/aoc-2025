#![allow(clippy::needless_return)]

use std::fs;

// const SAMPLE_PATH: &str = "./input/sample.txt";
const REAL_PATH: &str = "./input/real.txt";

fn solve_part1() {
    fn id_is_invalid(id: u64) -> bool {
        let digits: u32 = id.ilog10() + 1;
        if !digits.is_multiple_of(2) {
            return false;
        }

        let offset = digits / 2;
        for i in 0..offset {
            let digit_at_i = (id / 10u64.pow(i)) % 10;
            let digit_at_i_plus_offset = (id / 10u64.pow(i + offset)) % 10;

            if digit_at_i != digit_at_i_plus_offset {
                return false;
            }
        }

        return true;
    }

    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut invalid_id_acc: u64 = 0;

    content.trim_end().split(',').for_each(|range| {
        let parts = range.split('-').collect::<Vec<_>>();
        assert!(parts.len() == 2);

        let start: u64 = parts[0].parse().expect("to parse start");
        let end: u64 = parts[1].parse().expect("to parse end");

        for id in start..=end {
            if id_is_invalid(id) {
                invalid_id_acc += id;
            }
        }
    });

    println!("{invalid_id_acc}");
}

fn solve_part2() {
    fn id_is_invalid(id: u64) -> bool {
        let digits: u32 = id.ilog10() + 1;
        let id_string = id.to_string();

        for sequence_length in 1..digits {
            let mut is_repeating = true;

            if !digits.is_multiple_of(sequence_length) {
                continue;
            }

            let sequence = &id_string[0..sequence_length as usize];

            for offset in (0..digits).step_by(sequence_length as usize) {
                let part = &id_string[offset as usize..(offset + sequence_length) as usize];
                if part != sequence {
                    is_repeating = false;
                    break;
                }
            }

            if is_repeating {
                return true;
            }
        }

        return false;
    }

    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut invalid_id_acc: u64 = 0;

    content.trim_end().split(',').for_each(|range| {
        let parts = range.split('-').collect::<Vec<_>>();
        assert!(parts.len() == 2);

        let start: u64 = parts[0].parse().expect("to parse start");
        let end: u64 = parts[1].parse().expect("to parse end");

        for id in start..=end {
            if id_is_invalid(id) {
                invalid_id_acc += id;
            }
        }
    });

    println!("{invalid_id_acc}");
}

fn main() {
    solve_part1();
    solve_part2();
}
