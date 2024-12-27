use std::env;
use std::fs;

fn parse_input(contents: String) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in contents.split("\n\n") {
        let (target, mut lines) = if block.starts_with('#') {
            (
                &mut locks,
                Box::new(block.split("\n").filter(|line| !line.is_empty()))
                    as Box<dyn Iterator<Item = &str>>,
            )
        } else {
            (
                &mut keys,
                Box::new(
                    block
                        .split("\n")
                        .filter(|line| !line.is_empty())
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev(),
                ) as Box<dyn Iterator<Item = &str>>,
            )
        };

        let mut item = vec![0_u8; lines.next().unwrap().len()];
        for line in lines {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    item[i] += 1;
                }
            }
        }

        target.push(item);
    }

    return (keys, locks);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (keys, locks) = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(a, b)| a + b < 6) {
                r += 1;
            }
        }
    }

    println!("{r}");
}
