#![allow(clippy::needless_return)]

use std::fs;

// const SAMPLE_PATH: &str = "./input/sample.txt";
const REAL_PATH: &str = "./input/real.txt";

fn solve_part1() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut ranges = vec![];
    let mut ingredients = vec![];

    let mut are_we_ingredient_yet = false;
    for mut line in content.lines() {
        line = line.trim();

        if line.is_empty() {
            are_we_ingredient_yet = true;
            continue;
        }

        if !are_we_ingredient_yet {
            let parts = line.split("-").collect::<Vec<&str>>();
            assert_eq!(parts.len(), 2);
            let start = parts[0].trim().parse::<u64>().unwrap();
            let end = parts[1].trim().parse::<u64>().unwrap();
            ranges.push(start..=end);
            continue;
        }

        if are_we_ingredient_yet {
            let ingredient = line.parse::<u64>().unwrap();
            ingredients.push(ingredient);
            continue;
        }
    }

    let mut acc = 0;

    for ingredient in ingredients {
        for range in &ranges {
            if range.contains(&ingredient) {
                acc += 1;
                break;
            }
        }
    }

    println!("{acc}");
}

fn solve_part2() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let mut ranges = vec![];

    for mut line in content.lines() {
        line = line.trim();

        if line.is_empty() {
            break;
        }

        let parts = line.split("-").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        let start = parts[0].trim().parse::<u64>().unwrap();
        let end = parts[1].trim().parse::<u64>().unwrap();
        ranges.push(start..=end);
    }

    ranges.sort_by_key(|r| *r.start());

    fn merge_ranges(ranges: &mut Vec<std::ops::RangeInclusive<u64>>) {
        let mut i = 0;
        while i < ranges.len() - 1 {
            let current = &ranges[i];
            let next = &ranges[i + 1];

            if current.end() + 1 >= *next.start() {
                let new_end = std::cmp::max(*current.end(), *next.end());
                ranges[i] = *current.start()..=new_end;
                ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    merge_ranges(&mut ranges);

    let mut acc = 0;

    for range in &ranges {
        acc += range.end() - range.start() + 1;
    }

    println!("{acc}");
}

fn main() {
    solve_part1();
    solve_part2();
}
