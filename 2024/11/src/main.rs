use std::env;
use std::fs;

#[derive(Debug)]
struct Stone(u64);

impl Stone {
    fn blink(self) -> Vec<Self> {
        let n = self.0;
        if n == 0 {
            return vec![Stone(1)];
        }

        let s = n.to_string();
        let l = s.len();
        if l % 2 == 0 {
            let s1 = &s[..l / 2];
            let s2 = &s[l / 2..];

            return vec![Stone(s1.parse().unwrap()), Stone(s2.parse().unwrap())];
        }

        return vec![Stone(n.checked_mul(2024).unwrap())];
    }
}

fn parse_input(contents: String) -> Vec<Stone> {
    let mut r = Vec::new();

    for s in contents.split_ascii_whitespace() {
        if s.len() == 0 {
            continue;
        }

        r.push(Stone(s.parse().unwrap()));
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut stones = parse_input(fs::read_to_string(&args[1]).unwrap());

    /*
    let it = stones.into_iter();
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    let it = it.flat_map(|s| s.blink());
    */

    for _ in 0..25 {
        stones = stones.into_iter().flat_map(|s| s.blink()).collect();
    }

    let n = stones.len();
    println!("{n}");
}
