#![allow(clippy::needless_return)]

use std::fs;

// const SAMPLE_PATH: &str = "./input/sample.txt";
const REAL_PATH: &str = "./input/real.txt";

fn solve_part1() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let lines = content.lines().collect::<Vec<&str>>();

    let mut rows = vec![];
    for line in &lines[0..lines.len() - 1] {
        let row = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        rows.push(row);
    }

    let operation = lines[lines.len() - 1]
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut acc: i64 = 0;

    for (index, operation) in operation.iter().enumerate() {
        match *operation {
            "+" => {
                for row in &rows {
                    acc += row[index] as i64;
                }
            }
            "*" => {
                let mut local_acc: i64 = 1;
                for row in &rows {
                    local_acc *= row[index] as i64;
                }
                acc += local_acc;
            }
            _ => {}
        }
    }

    println!("{acc}");
}

/// this is horrible, why didnt i think of zipping the lines,,, still less unwrap than cloudflare code
fn solve_part2() {
    let content = fs::read_to_string(REAL_PATH).expect("to read input file");

    let lines = content.lines().collect::<Vec<&str>>();

    let mut acc: i64 = 0;

    let mut cursor: usize = 0;
    'outer: loop {
        let mut op = ' ';
        let mut values = vec![];
        for _ in &lines {
            values.push(vec![]);
        }

        'inner: loop {
            if op == ' '
                && let Some(c) = lines.last().unwrap().chars().nth(cursor)
            {
                op = c;
            }

            for (index, line) in lines.iter().enumerate() {
                values[index].push(line.chars().nth(cursor).unwrap());
            }

            cursor += 1;

            if let Some(c) = lines.last().unwrap().chars().nth(cursor + 1)
                && c != ' '
            {
                break 'inner;
            }

            if cursor == lines[0].len() {
                break 'inner;
            }
        }

        let mut transposed = vec![];
        for _ in &lines {
            transposed.push(vec![]);
        }

        for index in (0..values[0].len()).rev() {
            for (line_index, _line) in lines.iter().enumerate() {
                if line_index == values.len() - 1 {
                    continue;
                }
                transposed[index].push(values[line_index][index]);
            }
        }

        let mut local_acc = match op {
            '*' => 1,
            _ => 0,
        };
        for row in &transposed {
            if row.is_empty() {
                continue;
            }

            let row_str = row.iter().collect::<String>();
            if row_str.trim().is_empty() {
                continue;
            }

            let row_value = row_str.trim().parse::<i32>().unwrap();

            match op {
                '+' => {
                    acc += row_value as i64;
                }
                '*' => {
                    local_acc *= row_value as i64;
                }
                _ => {}
            }
        }
        acc += local_acc;

        if cursor >= lines[0].len() {
            break 'outer;
        }
    }

    println!("{acc}");
}

fn main() {
    solve_part1();
    solve_part2();
}
