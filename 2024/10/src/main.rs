use aoc_util::try_get;
use std::collections::HashSet;
use std::env;
use std::fs;

type RC = i8;
type Pt = aoc_util::Pt<RC>;
type Ht = i8;
type Topo = Vec<Vec<Ht>>;

const DELTAS: [Pt; 4] = [
    Pt { row: -1, col: 0 },
    Pt { row: 1, col: 0 },
    Pt { row: 0, col: -1 },
    Pt { row: 0, col: 1 },
];

fn parse_input(contents: String) -> (Topo, HashSet<Pt>) {
    let mut topo = Vec::new();
    let mut heads = HashSet::new();

    for (row, l) in contents.split('\n').enumerate() {
        if l.len() == 0 {
            continue;
        }

        let mut row_vec = Vec::new();
        for (col, c) in l.trim().chars().enumerate() {
            let h: Ht = c.to_digit(10).unwrap().try_into().unwrap();
            row_vec.push(h);

            let p = Pt {
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap(),
            };
            if h == 0 {
                heads.insert(p);
            }
        }

        topo.push(row_vec);
    }

    return (topo, heads);
}

fn wander(topo: &Topo, start: &Pt, level: Ht) -> u64 {
    if level == 10 {
        return 1;
    }

    let mut subscore = 0;
    for d in &DELTAS {
        let new_pt = start + d;
        match try_get(topo, &new_pt) {
            Some(h) if *h == level => subscore += wander(topo, &new_pt, level + 1),
            _ => (),
        }
    }

    return subscore;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (topo, heads) = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut score = 0;
    for head in heads {
        score += wander(&topo, &head, 1);
    }

    println!("{score}");
}
