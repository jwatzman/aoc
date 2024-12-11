use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

fn evolve(memo: &mut HashMap<(Stone, u64), u64>, depth: u64, stone: Stone) -> u64 {
    if depth == 0 {
        return 1;
    }

    let k = (stone.clone(), depth);
    if let Some(n) = memo.get(&k) {
        return *n;
    }

    let n = stone
        .blink()
        .into_iter()
        .map(|s| evolve(memo, depth - 1, s))
        .sum();

    memo.insert(k, n);
    return n;
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
    let stones = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut memo = HashMap::new();
    let n: u64 = stones.into_iter().map(|s| evolve(&mut memo, 75, s)).sum();
    println!("{n}");
}
