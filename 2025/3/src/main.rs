use std::{env, fs};

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
        .collect()
}

fn parse_input(contents: String) -> Vec<Vec<u8>> {
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

    let mut tot = 0_u32;
    for line in input {
        let max = line[..line.len() - 1]
            .iter()
            .enumerate()
            .fold(None, |prev, v| match prev {
                None => Some(v),
                Some(prev) => {
                    if prev.1 < v.1 {
                        Some(v)
                    } else {
                        Some(prev)
                    }
                }
            })
            .unwrap();

        let max2 = line[max.0 + 1..].iter().max().unwrap();

        tot += u32::from(*max.1) * 10 + u32::from(*max2);
    }

    println!("{}", tot);
}
