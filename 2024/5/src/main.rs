use std::env;
use std::fs;

fn parse_input(contents: String) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let mut it = contents.split('\n');
    let mut rules = Vec::new();
    loop {
        let line = it.next().unwrap();
        if line.len() == 0 {
            break;
        }

        let split: Vec<_> = line.split('|').collect();
        rules.push((split[0].parse().unwrap(), split[1].parse().unwrap()));
    }

    let mut updates = Vec::new();
    loop {
        let line = it.next().unwrap();
        if line.len() == 0 {
            break;
        }
        updates.push(line.split(',').map(|n| n.parse().unwrap()).collect());
    }

    return (rules, updates);
}

fn is_rule_followed(update: &Vec<u8>, rule: &(u8, u8)) -> bool {
    let mut found_second = false;

    for page in update {
        if *page == rule.1 {
            found_second = true;
        } else if *page == rule.0 {
            return !found_second;
        }
    }

    return true;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (rules, updates) = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r: u32 = 0;
    for update in &updates {
        if rules.iter().all(|rule| is_rule_followed(update, rule)) {
            let mid: u32 = update[update.len() / 2].into();
            r += mid;
        }
    }

    println!("{r}");
}
