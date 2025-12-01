use std::{env, fs};

fn parse_line(line: &str) -> i64 {
    let n: i64 = line[1..].parse().unwrap();
    match line.chars().next().unwrap() {
        'L' => -n,
        'R' => n,
        _ => panic!("Invalid char"),
    }
}

fn parse_input(contents: String) -> Vec<i64> {
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
    for n in &input {
        let was_zero = pos == 0;

        pos += n;
        result += pos.div_euclid(100).abs();
        pos = pos.rem_euclid(100);

        if *n < 0 {
            if was_zero {
                result -= 1;
            }
            if pos == 0 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
