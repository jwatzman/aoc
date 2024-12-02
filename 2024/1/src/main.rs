use std::env;
use std::fs;
use std::iter::zip;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in contents.split('\n') {
        let sp: Vec<_> = line.split_whitespace().collect();
        if sp.len() != 2 {
            continue;
        }
        v1.push(sp[0].parse().expect("Expected a number"));
        v2.push(sp[1].parse().expect("Expected a number"));
    }

    v1.sort();
    v2.sort();

    let mut res = 0;
    for (a, b) in zip(v1, v2) {
        res += (a - b).abs();
    }

    println!("{res}");
}
