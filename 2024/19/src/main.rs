use std::collections::HashMap;
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

fn count_all_ways<'a>(
    memo: &mut HashMap<&'a str, usize>,
    towel_patterns: &HashSet<&str>,
    design: &'a str,
) -> usize {
    if let Some(n) = memo.get(design) {
        return *n;
    }

    if design.is_empty() {
        return 1;
    }

    let mut r = 0;
    for i in 1..=design.len() {
        let towel = &design[..i];
        if towel_patterns.contains(towel) {
            r += count_all_ways(memo, towel_patterns, &design[i..]);
        }
    }

    memo.insert(design, r);
    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let (towel_patterns, towel_designs) = parse_input(&contents);

    let mut r = 0;
    let mut memo = HashMap::new();

    for design in towel_designs {
        r += count_all_ways(&mut memo, &towel_patterns, design);
    }

    println!("{r} ({})", memo.len());
}
