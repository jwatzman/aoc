use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let re = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let mut res = 0;
    for c in re.captures_iter(&contents) {
        let base = c.get(0).unwrap().as_str();
        if base == "don't()" {
            enabled = false;
        } else if base == "do()" {
            enabled = true;
        } else if enabled {
            let l: i32 = c.get(2).unwrap().as_str().parse().expect("expected number");
            let r: i32 = c.get(3).unwrap().as_str().parse().expect("expected number");
            res += l * r;
        }
    }

    println!("{res}");
}
