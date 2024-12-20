use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_input(contents: &String) -> (HashSet<&str>, Vec<&str>) {
    let mut it = contents.split('\n');
    let towel_patterns = it.next().unwrap().split(',').map(|s| s.trim()).collect();

    it.next();

    let towel_designs = it.filter(|s| !s.is_empty()).collect();

    return (towel_patterns, towel_designs);
}

fn can_make_design(towel_patterns: &HashSet<&str>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for i in 1..=design.len() {
        let towel = &design[..i];
        if towel_patterns.contains(towel) {
            if can_make_design(towel_patterns, &design[i..]) {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let (towel_patterns, towel_designs) = parse_input(&contents);

    let mut r = 0;
    for design in towel_designs {
        dbg!(&design);
        let can_make = can_make_design(&towel_patterns, design);
        if can_make {
            r += 1;
        }
    }

    println!("{r}");
}
