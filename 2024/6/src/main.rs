use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Lab {
    obstructions: HashSet<(i16, i16)>,
    max: (i16, i16),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Guard {
    pos: (i16, i16),
    dir: (i16, i16),
}

fn parse_input(contents: String) -> (Lab, Guard) {
    let mut obstructions = HashSet::new();
    let mut lab_max = None;
    let mut guard = None;

    for (row, l) in contents.split('\n').enumerate() {
        for (col, c) in l.trim().chars().enumerate() {
            let row16 = i16::try_from(row).unwrap();
            let col16 = i16::try_from(col).unwrap();
            if c == '#' {
                obstructions.insert((row16, col16));
            } else if c == '^' {
                guard = Some(Guard {
                    pos: (row16, col16),
                    dir: (-1, 0),
                });
            } else if c == '.' {
            } else {
                panic!("Unknown character");
            }
            lab_max = Some((row16, col16));
        }
    }

    return (
        Lab {
            obstructions,
            max: lab_max.unwrap(),
        },
        guard.unwrap(),
    );
}

fn addp(p1: &(i16, i16), p2: &(i16, i16)) -> (i16, i16) {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn rot(dir: &(i16, i16)) -> (i16, i16) {
    (dir.1, -dir.0)
}

fn step(lab: &Lab, mut guard: Guard) -> Option<Guard> {
    let new_pos = addp(&guard.pos, &guard.dir);
    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 > lab.max.0 || new_pos.1 > lab.max.1 {
        return None;
    }
    if lab.obstructions.contains(&new_pos) {
        guard.dir = rot(&guard.dir);
        return step(lab, guard);
    }
    guard.pos = new_pos;
    return Some(guard);
}

fn does_loop(lab: &Lab, guard: &Guard) -> bool {
    let mut guard = guard.clone();
    let mut all_pos = HashSet::new();

    loop {
        if all_pos.contains(&guard) {
            return true;
        }

        all_pos.insert(guard.clone());
        match step(&lab, guard) {
            None => return false,
            Some(new_guard) => guard = new_guard,
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (lab, guard) = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r = 0;
    for row in 0..(lab.max.0 + 1) {
        for col in 0..(lab.max.1 + 1) {
            let p = (row, col);
            if lab.obstructions.contains(&p) || p == guard.pos {
                continue;
            }

            let mut new_lab = lab.clone();
            new_lab.obstructions.insert(p);
            if does_loop(&new_lab, &guard) {
                r += 1;
            }
        }
        println!("({row})");
    }

    println!("{r}");
}
