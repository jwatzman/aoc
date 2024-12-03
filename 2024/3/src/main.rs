use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut res = 0;
    for (_, [l_str, r_str]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let l: i32 = l_str.parse().expect("expected number");
        let r: i32 = r_str.parse().expect("expected number");
        res += l * r;
    }
    println!("{res}");
}
