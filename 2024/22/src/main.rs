use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_input(contents: String) -> Vec<u64> {
    contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

fn mix_and_prune(a: u64, b: u64) -> u64 {
    (a ^ b) & 0xFFFFFF
}

fn evolve(mut secret: u64) -> u64 {
    secret = mix_and_prune(secret, secret << 6);
    secret = mix_and_prune(secret, secret >> 5);
    secret = mix_and_prune(secret, secret << 11);

    return secret;
}

fn prices(mut secret: u64) -> Vec<i8> {
    let mut r = Vec::new();
    r.reserve(2001);

    for _ in 0..2001 {
        r.push(i8::try_from(secret % 10).unwrap());
        secret = evolve(secret);
    }

    return r;
}

fn tally(tot: &mut HashMap<(i8, i8, i8, i8), u64>, init_secret: u64) {
    let mut seen = HashSet::new();
    let prices = prices(init_secret);

    for i in 4..prices.len() {
        let a = prices[i - 4];
        let b = prices[i - 3];
        let c = prices[i - 2];
        let d = prices[i - 1];
        let e = prices[i];

        let k = (b - a, c - b, d - c, e - d);

        if seen.insert(k) {
            *tot.entry(k).or_insert(0) += u64::try_from(e).unwrap();
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let init_secrets = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut tot = HashMap::new();
    for init_secret in &init_secrets {
        tally(&mut tot, *init_secret);
    }

    let mut best_n = u64::MIN;
    let mut best_seq = None;
    for (seq, n) in tot {
        if n > best_n {
            best_n = n;
            best_seq = Some(seq);
        }
    }

    println!("{best_n} {best_seq:?}");
}
