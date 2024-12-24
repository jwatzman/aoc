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

fn main() {
    let args: Vec<_> = env::args().collect();
    let init_secrets = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r = 0;
    for init_secret in &init_secrets {
        let mut secret = *init_secret;
        for _ in 0..2000 {
            secret = evolve(secret);
        }
        r += secret;
    }

    println!("{r}")
}
