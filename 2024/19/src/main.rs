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

fn can_make_design(towel_patterns: &HashSet<&str>, design: &str) -> usize {
    if design.is_empty() {
        return 1;
    }

    let mut r = 0;
    for i in 1..=design.len() {
        let towel = &design[..i];
        if towel_patterns.contains(towel) {
            r += can_make_design(towel_patterns, &design[i..]);
        }
    }

    return r;
}

fn reduce<'a>(towel_patterns: &'a HashSet<&str>) -> HashSet<&'a str> {
    let mut r = towel_patterns.clone();
    for pattern in towel_patterns.iter() {
        r.remove(pattern);
        if can_make_design(&r, pattern) == 0 {
            r.insert(pattern);
        }
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let (towel_patterns, towel_designs) = parse_input(&contents);
    let reduced_towel_patterns = reduce(&towel_patterns);

    let mut r = 0;
    for design in towel_designs {
        if can_make_design(&reduced_towel_patterns, design) > 0 {
            r += can_make_design(&towel_patterns, design);
        }
    }

    println!("{r}");
}
