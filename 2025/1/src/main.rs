use std::{env, fs};

#[derive(Debug)]
enum Turn {
    Left(u32),
    Right(u32),
}

fn parse_line(line: &str) -> Turn {
    let n: u32 = line[1..].parse().unwrap();
    match line.chars().next().unwrap() {
        'L' => Turn::Left(n),
        'R' => Turn::Right(n),
        _ => panic!("Invalid char"),
    }
}

fn parse_input(contents: String) -> Vec<Turn> {
    contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| parse_line(s))
        .collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let input = parse_input(contents);

    let mut pos: i64 = 50;
    let mut result = 0;
    for turn in &input {
        match turn {
            Turn::Left(n) => pos -= i64::from(*n),
            Turn::Right(n) => pos += i64::from(*n),
        }

        pos = pos.rem_euclid(100);
        if pos == 0 {
            result += 1;
        }
    }

    println!("{}", result);
}
