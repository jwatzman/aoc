use std::collections::HashMap;
use std::collections::HashSet;
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

fn make_deps(rules: &Vec<(u8, u8)>) -> HashMap<u8, HashSet<u8>> {
    let mut m = HashMap::new();
    for rule in rules {
        if !m.contains_key(&rule.1) {
            m.insert(rule.1, HashSet::new());
        }
        m.get_mut(&rule.1).unwrap().insert(rule.0);
    }

    return m;
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

fn tsort(r: &mut Vec<u8>, item: &u8, deps: &HashMap<u8, HashSet<u8>>, done: &mut HashSet<u8>) {
    if done.contains(&item) {
        return;
    }

    for dep in deps.get(item).unwrap_or(&HashSet::new()) {
        tsort(r, dep, deps, done);
    }

    r.push(*item);
    done.insert(*item);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (rules, updates) = parse_input(fs::read_to_string(&args[1]).unwrap());
    let mut bad_updates = Vec::new();

    for update in &updates {
        if !rules.iter().all(|rule| is_rule_followed(update, rule)) {
            bad_updates.push(update);
        }
    }

    let mut res: u32 = 0;
    for bad in bad_updates {
        let bad_set: HashSet<u8> = bad.iter().cloned().collect();
        let deps = make_deps(
            &rules
                .iter()
                .cloned()
                .filter(|r| bad_set.contains(&r.0) && bad_set.contains(&r.1))
                .collect(),
        );
        let mut fixed_update = Vec::new();
        let mut done = HashSet::new();
        for item in &bad_set {
            tsort(&mut fixed_update, item, &deps, &mut done);
        }

        let mid: u32 = fixed_update[fixed_update.len() / 2].into();
        res += mid;

        for rule in &rules {
            if !is_rule_followed(&fixed_update, rule) {
                dbg!(bad, &fixed_update, rule);
                panic!("oh no");
            }
        }
    }

    println!("{res}");
}
