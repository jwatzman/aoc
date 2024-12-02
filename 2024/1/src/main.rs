use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut v1: Vec<i32> = Vec::new();
    let mut h2: HashMap<i32, i32> = HashMap::new();

    for line in contents.split('\n') {
        let sp: Vec<_> = line.split_whitespace().collect();
        if sp.len() != 2 {
            continue;
        }

        v1.push(sp[0].parse().expect("Expected a number"));

        let k2: i32 = sp[1].parse().expect("Eexpected a number");
        let v2 = h2.get(&k2).unwrap_or(&0);
        h2.insert(k2, v2 + 1);
    }

    let mut res = 0;
    for n in v1 {
        res += n * h2.get(&n).unwrap_or(&0);
    }
    println!("{res}");
}
